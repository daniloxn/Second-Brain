//! Adaptador do Antigravity, chamado pelo comando `agy`. Usa o modo de impressão (`-p`),
//! que roda um pedido e sai. Opções conferidas em `agy --help`. A saída em JSON traz
//! `status` (SUCCESS quando deu certo) e `response` (o texto da resposta).

use super::provedor::ProvedorAgente;
use serde::Deserialize;

/// Tempo máximo de uma rodada do agente, no formato que o `agy --print-timeout` aceita.
const TEMPO_MAXIMO_DA_RODADA: &str = "300s";

/// Status que o `agy` devolve quando a rodada termina bem.
const STATUS_DE_SUCESSO: &str = "SUCCESS";

/// Adaptador do Antigravity.
pub struct AdaptadorAgy;

/// Formato da saída do `agy --output-format json`. Campos que o app não usa são ignorados.
/// `error` só vem preenchido quando a rodada falha (ex.: serviço sem capacidade no momento).
#[derive(Deserialize)]
struct SaidaDoAgy {
    status: String,
    response: String,
    error: Option<String>,
}

impl ProvedorAgente for AdaptadorAgy {
    fn nome(&self) -> &'static str {
        "Antigravity"
    }

    fn comando(&self) -> &'static str {
        "agy"
    }

    /// Sem `-p`: o `agy` lê o prompt da entrada padrão. `accept-edits` aceita as edições
    /// de arquivo sem perguntar. `--sandbox` restringe os comandos de terminal.
    /// A pasta do cofre entra em `--add-dir` para o agente enxergar o cofre.
    fn montar_argumentos(&self, pasta_do_cofre: &str) -> Vec<String> {
        vec![
            "--output-format".to_string(),
            "json".to_string(),
            "--mode".to_string(),
            "accept-edits".to_string(),
            "--sandbox".to_string(),
            "--add-dir".to_string(),
            pasta_do_cofre.to_string(),
            "--print-timeout".to_string(),
            TEMPO_MAXIMO_DA_RODADA.to_string(),
        ]
    }

    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let saida_lida: SaidaDoAgy = serde_json::from_str(saida.trim())
            .map_err(|erro| format!("resposta inesperada do agy: {erro}"))?;
        if saida_lida.status != STATUS_DE_SUCESSO {
            let motivo = saida_lida.error.unwrap_or_default();
            return Err(format!("o agente terminou com status {} {motivo}", saida_lida.status).trim_end().to_string());
        }
        Ok(saida_lida.response.trim().to_string())
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn argumentos_liberam_so_a_pasta_do_cofre_e_nao_levam_o_prompt() {
        let argumentos = AdaptadorAgy.montar_argumentos("C:\\cofre");
        assert!(!argumentos.iter().any(|argumento| argumento == "-p"));
        let posicao_da_pasta = argumentos.iter().position(|argumento| argumento == "--add-dir").unwrap();
        assert_eq!(argumentos[posicao_da_pasta + 1], "C:\\cofre");
    }

    #[test]
    fn resposta_com_sucesso_devolve_o_texto() {
        let saida = r#"{"conversation_id":"x","status":"SUCCESS","response":"Tarefa criada.\n"}"#;
        assert_eq!(AdaptadorAgy.ler_resposta(saida).unwrap(), "Tarefa criada.");
    }

    #[test]
    fn resposta_com_erro_de_status_falha() {
        let saida = r#"{"status":"ERROR","response":""}"#;
        assert!(AdaptadorAgy.ler_resposta(saida).is_err());
    }

    #[test]
    fn saida_que_nao_e_json_falha() {
        assert!(AdaptadorAgy.ler_resposta("texto solto").is_err());
    }
}
