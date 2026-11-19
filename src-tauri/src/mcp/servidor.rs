//! Laço principal do servidor MCP. Lê uma mensagem JSON por linha da entrada padrão e responde
//! na saída padrão. A saída padrão é só do protocolo: qualquer diagnóstico vai para a saída de erro.

use super::cofre_ativo::raiz_do_cofre_para_o_mcp;
use super::ferramentas::{chamar_ferramenta, lista_de_ferramentas, ErroDeFerramenta};
use super::protocolo::{
    resposta_de_erro, resposta_de_sucesso, resultado_de_texto, versao_negociada, ERRO_DE_FORMATO,
    METODO_NAO_ENCONTRADO, PARAMETROS_INVALIDOS,
};
use chrono::{Local, NaiveDateTime};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::path::Path;

/// Nome que o servidor se apresenta ao cliente.
const NOME_DO_SERVIDOR: &str = "segundo-cerebro";

/// Instruções enviadas no `initialize`. O cliente costuma mostrá-las ao modelo.
const INSTRUCOES_DO_SERVIDOR: &str = "Ferramentas do cofre do Segundo Cérebro: tarefas e notas. \
Antes de criar uma tarefa, proponha título, prazo e prioridade e espere a confirmação do usuário.";

/// Descobre o cofre e atende mensagens até a entrada padrão ser fechada pelo cliente.
pub fn executar_servidor(argumentos: &[String]) -> Result<(), String> {
    let raiz = raiz_do_cofre_para_o_mcp(argumentos)?;
    let entrada = io::stdin().lock();
    let mut saida = io::stdout().lock();

    for linha in entrada.lines() {
        let linha = linha.map_err(|erro| erro.to_string())?;
        if linha.trim().is_empty() {
            continue;
        }
        let resposta = match serde_json::from_str::<Value>(&linha) {
            Ok(mensagem) => responder(&mensagem, &raiz, Local::now().naive_local()),
            Err(erro) => Some(resposta_de_erro(&Value::Null, ERRO_DE_FORMATO, &format!("JSON inválido: {erro}"))),
        };
        if let Some(resposta) = resposta {
            writeln!(saida, "{resposta}").and_then(|_| saida.flush()).map_err(|erro| erro.to_string())?;
        }
    }
    Ok(())
}

/// Decide a resposta a uma mensagem do cliente. Devolve `None` quando não há o que responder:
/// notificações (sem `id`) e respostas do cliente a pedidos que este servidor nunca fez.
pub(crate) fn responder(mensagem: &Value, raiz: &Path, agora: NaiveDateTime) -> Option<Value> {
    let metodo = mensagem.get("method").and_then(Value::as_str)?;
    // Sem `id` é notificação, e o protocolo proíbe resposta a ela.
    let id = mensagem.get("id")?;
    let parametros = mensagem.get("params").cloned().unwrap_or(Value::Null);

    Some(match metodo {
        "initialize" => resposta_de_sucesso(id, resultado_do_initialize(&parametros)),
        "ping" => resposta_de_sucesso(id, json!({})),
        "tools/list" => resposta_de_sucesso(id, json!({"tools": lista_de_ferramentas()})),
        "tools/call" => responder_chamada_de_ferramenta(id, &parametros, raiz, agora),
        _ => resposta_de_erro(id, METODO_NAO_ENCONTRADO, &format!("método não suportado: {metodo}")),
    })
}

/// Resultado do `initialize`: versão do protocolo, capacidades (só ferramentas) e identificação do servidor.
fn resultado_do_initialize(parametros: &Value) -> Value {
    let versao_pedida = parametros.get("protocolVersion").and_then(Value::as_str);
    json!({
        "protocolVersion": versao_negociada(versao_pedida),
        "capabilities": {"tools": {}},
        "serverInfo": {"name": NOME_DO_SERVIDOR, "version": env!("CARGO_PKG_VERSION")},
        "instructions": INSTRUCOES_DO_SERVIDOR
    })
}

/// Executa `tools/call`. Nome ausente ou ferramenta desconhecida é erro de protocolo;
/// falha da própria ferramenta volta como resultado com `isError`, para o agente ler.
fn responder_chamada_de_ferramenta(id: &Value, parametros: &Value, raiz: &Path, agora: NaiveDateTime) -> Value {
    let Some(nome) = parametros.get("name").and_then(Value::as_str) else {
        return resposta_de_erro(id, PARAMETROS_INVALIDOS, "tools/call precisa do nome da ferramenta");
    };
    let argumentos = parametros.get("arguments").cloned().unwrap_or_else(|| json!({}));

    match chamar_ferramenta(raiz, nome, argumentos, agora) {
        Ok(texto) => resposta_de_sucesso(id, resultado_de_texto(&texto, false)),
        Err(ErroDeFerramenta::Desconhecida) => {
            resposta_de_erro(id, PARAMETROS_INVALIDOS, &format!("ferramenta desconhecida: {nome}"))
        }
        Err(ErroDeFerramenta::Falhou(mensagem)) => resposta_de_sucesso(id, resultado_de_texto(&mensagem, true)),
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use chrono::NaiveDate;
    use std::fs;
    use std::path::PathBuf;

    fn cofre_de_teste(nome: &str) -> PathBuf {
        let raiz = std::env::temp_dir().join(format!("segundo-cerebro-servidor-{nome}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&raiz);
        fs::create_dir_all(raiz.join("_sistema")).unwrap();
        raiz.canonicalize().unwrap()
    }

    fn agora_de_teste() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 11, 17).unwrap().and_hms_opt(8, 0, 0).unwrap()
    }

    #[test]
    fn notificacao_nao_recebe_resposta() {
        let raiz = cofre_de_teste("notificacao");
        let mensagem = json!({"jsonrpc": "2.0", "method": "notifications/initialized"});
        assert!(responder(&mensagem, &raiz, agora_de_teste()).is_none());
    }

    #[test]
    fn initialize_responde_com_ferramentas_e_nome_do_servidor() {
        let raiz = cofre_de_teste("initialize");
        let mensagem = json!({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-03-26"}});
        let resposta = responder(&mensagem, &raiz, agora_de_teste()).unwrap();

        assert_eq!(resposta["id"], 1);
        assert_eq!(resposta["result"]["protocolVersion"], "2025-03-26");
        assert!(resposta["result"]["capabilities"]["tools"].is_object());
        assert_eq!(resposta["result"]["serverInfo"]["name"], NOME_DO_SERVIDOR);
    }

    #[test]
    fn tools_list_traz_as_ferramentas() {
        let raiz = cofre_de_teste("lista");
        let mensagem = json!({"jsonrpc": "2.0", "id": 2, "method": "tools/list"});
        let resposta = responder(&mensagem, &raiz, agora_de_teste()).unwrap();
        assert_eq!(resposta["result"]["tools"].as_array().unwrap().len(), 4);
    }

    #[test]
    fn metodo_desconhecido_com_id_recebe_erro_de_protocolo() {
        let raiz = cofre_de_teste("metodo");
        let mensagem = json!({"jsonrpc": "2.0", "id": 3, "method": "resources/list"});
        let resposta = responder(&mensagem, &raiz, agora_de_teste()).unwrap();
        assert_eq!(resposta["error"]["code"], METODO_NAO_ENCONTRADO);
    }

    #[test]
    fn ferramenta_desconhecida_e_erro_de_protocolo() {
        let raiz = cofre_de_teste("desconhecida");
        let mensagem = json!({"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "apagar_tudo"}});
        let resposta = responder(&mensagem, &raiz, agora_de_teste()).unwrap();
        assert_eq!(resposta["error"]["code"], PARAMETROS_INVALIDOS);
    }

    #[test]
    fn falha_da_ferramenta_volta_como_resultado_com_is_error() {
        let raiz = cofre_de_teste("falha");
        let mensagem = json!({
            "jsonrpc": "2.0", "id": 5, "method": "tools/call",
            "params": {"name": "criar_tarefa", "arguments": {"titulo": "x", "prazo": "amanhã"}}
        });
        let resposta = responder(&mensagem, &raiz, agora_de_teste()).unwrap();
        assert_eq!(resposta["result"]["isError"], true);
        assert!(resposta["result"]["content"][0]["text"].as_str().unwrap().contains("prazo inválido"));
    }

    #[test]
    fn chamada_com_sucesso_devolve_texto_sem_is_error() {
        let raiz = cofre_de_teste("sucesso");
        let mensagem = json!({"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "listar_tarefas"}});
        let resposta = responder(&mensagem, &raiz, agora_de_teste()).unwrap();
        assert_eq!(resposta["result"]["isError"], false);
    }
}
