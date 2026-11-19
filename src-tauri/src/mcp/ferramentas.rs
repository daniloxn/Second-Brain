//! Ferramentas que o agente enxerga: nome, descrição, esquema dos argumentos e a chamada de cada uma.
//! Cada chamada só delega para as funções de tarefas e notas e transforma o resultado em texto.
//! Nenhuma regra de negócio mora aqui.

use crate::notas::consulta::{buscar_notas_da_raiz, ler_nota_da_raiz};
use crate::tarefas::contrato::NovaTarefaDto;
use crate::tarefas::servico::{criar_tarefa_no_cofre, listar_tarefas_do_cofre};
use chrono::NaiveDateTime;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::Path;

/// Por que a chamada de uma ferramenta não deu certo.
#[derive(Debug)]
pub(crate) enum ErroDeFerramenta {
    /// Nome que este servidor não conhece. Vira erro de protocolo.
    Desconhecida,
    /// A ferramenta existe, mas não conseguiu: a mensagem vai para o agente ler.
    Falhou(String),
}

/// Argumentos de `buscar_notas`.
#[derive(Deserialize)]
struct ArgumentosDeBuscarNotas {
    termo: String,
}

/// Argumentos de `ler_nota`.
#[derive(Deserialize)]
struct ArgumentosDeLerNota {
    caminho: String,
}

/// Lista de ferramentas no formato do MCP: nome, descrição e JSON Schema dos argumentos.
/// A descrição de `criar_tarefa` diz que só se cria depois da confirmação, porque o agente lê isso.
pub(crate) fn lista_de_ferramentas() -> Value {
    json!([
        {
            "name": "listar_tarefas",
            "description": "Lista as tarefas do cofre, da mais urgente para a menos urgente. Cada uma traz status, prioridade, prazo e situação (atrasada, hoje, proxima, sem_prazo, concluida).",
            "inputSchema": {"type": "object", "properties": {}, "additionalProperties": false}
        },
        {
            "name": "criar_tarefa",
            "description": "Cria uma tarefa em tarefas/. Chame só depois de o usuário confirmar o título, o prazo e a prioridade.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "titulo": {"type": "string", "description": "Título curto da tarefa."},
                    "descricao": {"type": "string", "description": "Texto livre em Markdown."},
                    "prioridade": {"type": "string", "enum": ["alta", "media", "baixa"]},
                    "prazo": {"type": "string", "description": "AAAA-MM-DD ou AAAA-MM-DD HH:MM."},
                    "origem": {"type": "string"},
                    "tags": {"type": "array", "items": {"type": "string"}},
                    "recorrencia": {"type": "string", "enum": ["nenhuma", "diaria"]}
                },
                "required": ["titulo"]
            }
        },
        {
            "name": "buscar_notas",
            "description": "Procura um texto no nome e no conteúdo dos arquivos .md do cofre (notas, tarefas e projetos). Devolve até 20 resultados, com caminho e a linha que casou. Não lê _sistema/ nem _segredos/.",
            "inputSchema": {
                "type": "object",
                "properties": {"termo": {"type": "string", "description": "Texto a procurar, sem diferenciar maiúsculas."}},
                "required": ["termo"]
            }
        },
        {
            "name": "ler_nota",
            "description": "Devolve o texto completo de uma nota .md do cofre.",
            "inputSchema": {
                "type": "object",
                "properties": {"caminho": {"type": "string", "description": "Caminho relativo à raiz do cofre, como o devolvido por buscar_notas. Ex.: notas/ideias/horta.md"}},
                "required": ["caminho"]
            }
        }
    ])
}

/// Executa a ferramenta `nome` sobre o cofre em `raiz` e devolve o texto da resposta.
/// `agora` vem de fora para as ferramentas de tarefa calcularem a situação sem depender do relógio.
pub(crate) fn chamar_ferramenta(raiz: &Path, nome: &str, argumentos: Value, agora: NaiveDateTime) -> Result<String, ErroDeFerramenta> {
    match nome {
        "listar_tarefas" => {
            let lista = listar_tarefas_do_cofre(raiz, agora).map_err(ErroDeFerramenta::Falhou)?;
            texto_em_json(&lista)
        }
        "criar_tarefa" => {
            let entrada: NovaTarefaDto = ler_argumentos(argumentos)?;
            let tarefa = criar_tarefa_no_cofre(raiz, entrada, agora).map_err(ErroDeFerramenta::Falhou)?;
            texto_em_json(&tarefa)
        }
        "buscar_notas" => {
            let argumentos_da_busca: ArgumentosDeBuscarNotas = ler_argumentos(argumentos)?;
            let resultados = buscar_notas_da_raiz(raiz, &argumentos_da_busca.termo).map_err(ErroDeFerramenta::Falhou)?;
            texto_em_json(&resultados)
        }
        "ler_nota" => {
            let argumentos_da_leitura: ArgumentosDeLerNota = ler_argumentos(argumentos)?;
            ler_nota_da_raiz(raiz, &argumentos_da_leitura.caminho).map_err(ErroDeFerramenta::Falhou)
        }
        _ => Err(ErroDeFerramenta::Desconhecida),
    }
}

/// Converte os argumentos recebidos no tipo esperado. Argumento errado vira erro para o agente.
fn ler_argumentos<T: DeserializeOwned>(argumentos: Value) -> Result<T, ErroDeFerramenta> {
    serde_json::from_value(argumentos).map_err(|erro| ErroDeFerramenta::Falhou(format!("argumentos inválidos: {erro}")))
}

/// Texto JSON com indentação, que o agente lê melhor que uma linha só.
fn texto_em_json<T: Serialize>(valor: &T) -> Result<String, ErroDeFerramenta> {
    serde_json::to_string_pretty(valor).map_err(|erro| ErroDeFerramenta::Falhou(erro.to_string()))
}

#[cfg(test)]
mod testes {
    use super::*;
    use chrono::NaiveDate;
    use std::fs;
    use std::path::PathBuf;

    fn cofre_de_teste(nome: &str) -> PathBuf {
        let raiz = std::env::temp_dir().join(format!("segundo-cerebro-mcp-{nome}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&raiz);
        fs::create_dir_all(raiz.join("_sistema")).unwrap();
        raiz.canonicalize().unwrap()
    }

    fn agora_de_teste() -> NaiveDateTime {
        NaiveDate::from_ymd_opt(2026, 11, 17).unwrap().and_hms_opt(8, 0, 0).unwrap()
    }

    #[test]
    fn servidor_expoe_as_quatro_ferramentas_com_nomes_do_plano() {
        let nomes: Vec<String> = lista_de_ferramentas()
            .as_array()
            .unwrap()
            .iter()
            .map(|ferramenta| ferramenta["name"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(nomes, vec!["listar_tarefas", "criar_tarefa", "buscar_notas", "ler_nota"]);
    }

    #[test]
    fn nome_desconhecido_nao_e_executado() {
        let raiz = cofre_de_teste("desconhecida");
        let resultado = chamar_ferramenta(&raiz, "apagar_tudo", json!({}), agora_de_teste());
        assert!(matches!(resultado, Err(ErroDeFerramenta::Desconhecida)));
    }

    #[test]
    fn criar_tarefa_com_prioridade_invalida_devolve_erro_para_o_agente() {
        let raiz = cofre_de_teste("prioridade");
        let argumentos = json!({"titulo": "Teste", "prioridade": "urgentíssima"});
        match chamar_ferramenta(&raiz, "criar_tarefa", argumentos, agora_de_teste()) {
            Err(ErroDeFerramenta::Falhou(mensagem)) => assert!(mensagem.contains("prioridade inválida")),
            _ => panic!("esperava erro de prioridade"),
        }
    }

    #[test]
    fn tarefa_criada_pela_ferramenta_aparece_em_listar_tarefas() {
        let raiz = cofre_de_teste("ida-e-volta");
        let argumentos = json!({"titulo": "Pagar conta", "prazo": "2026-11-18", "prioridade": "alta"});
        chamar_ferramenta(&raiz, "criar_tarefa", argumentos, agora_de_teste()).unwrap();

        let texto = chamar_ferramenta(&raiz, "listar_tarefas", json!({}), agora_de_teste()).unwrap();
        let lista: Value = serde_json::from_str(&texto).unwrap();

        assert_eq!(lista["tarefas"][0]["titulo"], "Pagar conta");
        assert_eq!(lista["tarefas"][0]["situacao"], "proxima");
    }

    #[test]
    fn argumento_faltando_devolve_erro_claro() {
        let raiz = cofre_de_teste("sem-termo");
        match chamar_ferramenta(&raiz, "buscar_notas", json!({}), agora_de_teste()) {
            Err(ErroDeFerramenta::Falhou(mensagem)) => assert!(mensagem.contains("argumentos inválidos")),
            _ => panic!("esperava erro de argumentos"),
        }
    }
}
