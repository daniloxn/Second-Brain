//! Formato das mensagens JSON-RPC 2.0 do MCP: respostas, erros e o resultado de uma ferramenta.
//! Funções puras: montam os valores sem ler nem escrever nada.

use serde_json::{json, Value};

/// Versões do protocolo MCP que este servidor fala, da mais nova para a mais antiga.
const VERSOES_DO_PROTOCOLO: &[&str] = &["2025-06-18", "2025-03-26", "2024-11-05"];

/// Código JSON-RPC para mensagem que não é JSON.
pub(crate) const ERRO_DE_FORMATO: i64 = -32700;
/// Código JSON-RPC para método que este servidor não conhece.
pub(crate) const METODO_NAO_ENCONTRADO: i64 = -32601;
/// Código JSON-RPC para parâmetros inválidos (inclui ferramenta desconhecida).
pub(crate) const PARAMETROS_INVALIDOS: i64 = -32602;

/// Resposta de sucesso ao pedido `id`.
pub(crate) fn resposta_de_sucesso(id: &Value, resultado: Value) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "result": resultado})
}

/// Resposta de erro ao pedido `id`. Usa `null` como id quando a mensagem nem chegou a ser lida.
pub(crate) fn resposta_de_erro(id: &Value, codigo: i64, mensagem: &str) -> Value {
    json!({"jsonrpc": "2.0", "id": id, "error": {"code": codigo, "message": mensagem}})
}

/// Versão do protocolo a usar: a pedida pelo cliente, se o servidor a conhece; senão a mais nova.
/// O cliente decide se aceita a versão devolvida, então não há como negociar além dessa escolha.
pub(crate) fn versao_negociada(versao_pedida: Option<&str>) -> &'static str {
    versao_pedida
        .and_then(|pedida| VERSOES_DO_PROTOCOLO.iter().find(|versao| **versao == pedida))
        .copied()
        .unwrap_or(VERSOES_DO_PROTOCOLO[0])
}

/// Resultado de `tools/call`. Texto de erro da ferramenta também volta aqui, com `isError: true`:
/// o agente lê a mensagem e pode corrigir o pedido. Erro de protocolo é outra coisa (ver `resposta_de_erro`).
pub(crate) fn resultado_de_texto(texto: &str, houve_erro: bool) -> Value {
    json!({"content": [{"type": "text", "text": texto}], "isError": houve_erro})
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn versao_pedida_conhecida_e_devolvida() {
        assert_eq!(versao_negociada(Some("2024-11-05")), "2024-11-05");
    }

    #[test]
    fn versao_desconhecida_cai_na_mais_nova() {
        assert_eq!(versao_negociada(Some("1999-01-01")), "2025-06-18");
        assert_eq!(versao_negociada(None), "2025-06-18");
    }

    #[test]
    fn resultado_de_erro_de_ferramenta_marca_is_error() {
        let resultado = resultado_de_texto("tarefa não encontrada", true);
        assert_eq!(resultado["isError"], true);
        assert_eq!(resultado["content"][0]["text"], "tarefa não encontrada");
    }
}
