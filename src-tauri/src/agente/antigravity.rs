//! Adaptador do Antigravity, pelo comando `agy`, em modo de impressão. O prompt vai colado em `--print=`.
//! A saída é um JSON com `status` e `response`. Sem `status` SUCCESS, ou sem texto, a rodada falha.
//! No modo headless o CLI nega comandos de terminal; quando isso acontece a resposta sai vazia e a
//! mensagem de erro diz quais ações foram negadas.

use super::contrato::ModoDeExecucao;
use super::provedor::ProvedorAgente;
use serde::Deserialize;

/// Adaptador do Antigravity.
pub struct AdaptadorAntigravity;

/// Envelope JSON que o `agy --output-format json` imprime ao final da rodada.
#[derive(Deserialize)]
struct SaidaAntigravity {
    status: Option<String>,
    response: Option<String>,
    denied_actions: Option<Vec<AcaoNegada>>,
}

/// Ação que o modo headless negou. Só o nome aparece na mensagem de erro.
#[derive(Deserialize)]
struct AcaoNegada {
    display_name: Option<String>,
}

impl ProvedorAgente for AdaptadorAntigravity {
    fn nome(&self) -> &'static str { "Antigravity" }

    fn comando(&self) -> &'static str { "agy" }

    /// Executar aceita edições. Planejar usa o modo `plan`, que não edita.
    fn montar_argumentos(&self, pasta_do_cofre: &str, modo: ModoDeExecucao) -> Vec<String> {
        let modo_do_agy = match modo {
            ModoDeExecucao::Executar => "accept-edits",
            // `Consultar` não chega aqui: o app recusa antes, porque o Antigravity não tem web configurada.
            ModoDeExecucao::Planejar | ModoDeExecucao::Consultar => "plan",
        };
        vec![
            "--output-format".into(), "json".into(),
            "--mode".into(), modo_do_agy.into(),
            "--add-dir".into(), pasta_do_cofre.into(),
        ]
    }

    /// O prompt é argumento, precedido de `--print=`. Sem isso o CLI pega a próxima opção como prompt.
    fn prompt_como_argumento(&self) -> bool { true }

    fn prefixo_do_argumento_do_prompt(&self) -> &'static str { "--print=" }

    /// Texto da resposta. Erro se o status não for SUCCESS, se a resposta vier vazia ou se houve ação negada.
    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let envelope: SaidaAntigravity = serde_json::from_str(saida.trim())
            .map_err(|erro| format!("resposta inesperada do Antigravity: {erro}"))?;
        let status = envelope.status.unwrap_or_else(|| "desconhecido".into());
        if status != "SUCCESS" {
            return Err(format!("Antigravity terminou com status {status}"));
        }
        let resposta = envelope.response.unwrap_or_default();
        if !resposta.trim().is_empty() {
            return Ok(resposta.trim().to_string());
        }
        let negadas: Vec<String> = envelope
            .denied_actions
            .unwrap_or_default()
            .into_iter()
            .filter_map(|acao| acao.display_name)
            .collect();
        if negadas.is_empty() {
            Err("Antigravity terminou sem escrever resposta".into())
        } else {
            Err(format!(
                "Antigravity terminou sem resposta porque o modo headless negou: {}",
                negadas.join(", ")
            ))
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn resposta_de_sucesso_devolve_o_texto() {
        let saida = r#"{"status":"SUCCESS","response":"nota.md","denied_actions":[]}"#;
        assert_eq!(AdaptadorAntigravity.ler_resposta(saida), Ok("nota.md".into()));
    }

    #[test]
    fn resposta_vazia_por_acao_negada_diz_qual() {
        let saida = r#"{"status":"SUCCESS","response":"","denied_actions":[{"action":"command","display_name":"Bash"}]}"#;
        let erro = AdaptadorAntigravity.ler_resposta(saida).unwrap_err();
        assert!(erro.contains("Bash"));
    }

    #[test]
    fn status_diferente_de_sucesso_falha() {
        let saida = r#"{"status":"ERROR","response":"x"}"#;
        assert!(AdaptadorAntigravity.ler_resposta(saida).is_err());
    }

    /// Validação opt-in com o `agy` instalado, pelo mesmo caminho usado pelo app (com prompt colado em `--print=`).
    #[test]
    #[ignore = "requer o Antigravity instalado e autenticado; faz uma chamada ao modelo"]
    fn antigravity_real_responde_pelo_caminho_do_app() {
        let pasta = std::env::temp_dir().join(format!("segundo-cerebro-agy-{}", std::process::id()));
        std::fs::create_dir_all(&pasta).unwrap();
        let resposta = super::super::executar::executar_provedor(
            &AdaptadorAntigravity,
            "Responda apenas a palavra OK. Não use ferramentas.",
            pasta.to_str().unwrap(),
            ModoDeExecucao::Planejar,
            |_| {},
            |_| {},
        );
        let _ = std::fs::remove_dir(&pasta);
        assert!(resposta.unwrap().to_uppercase().contains("OK"));
    }

    #[test]
    fn prompt_vai_colado_na_opcao_print() {
        assert_eq!(AdaptadorAntigravity.prefixo_do_argumento_do_prompt(), "--print=");
    }
}
