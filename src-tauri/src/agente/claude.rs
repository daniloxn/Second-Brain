//! Adaptador do Claude Code em modo de impressão, sem ferramenta de terminal.
//! Usa `stream-json`: cada linha é um evento. Mensagens do assistente trazem texto e passos,
//! e o evento `result` no fim traz a resposta final.

use super::contrato::ModoDeExecucao;
use super::passos::rotulo_do_passo;
use super::provedor::ProvedorAgente;
use serde_json::Value;

/// Adaptador do Claude Code.
pub struct AdaptadorClaude;

/// Blocos de conteúdo de uma mensagem do assistente: o campo `type` diz se é texto, ferramenta ou outro.
fn blocos_da_mensagem(evento: &Value) -> &[Value] {
    evento["message"]["content"].as_array().map(Vec::as_slice).unwrap_or(&[])
}

impl ProvedorAgente for AdaptadorClaude {
    fn nome(&self) -> &'static str { "Claude Code" }

    fn comando(&self) -> &'static str { "claude" }

    /// Execução permite editar com `acceptEdits`. Planejar e Consultar usam o modo `plan`, que não edita.
    /// Consultar libera as ferramentas de web e leitura. Em todos, Bash fica bloqueado e os turnos são limitados.
    fn montar_argumentos(&self, pasta_do_cofre: &str, modo: ModoDeExecucao) -> Vec<String> {
        let permissao = match modo {
            ModoDeExecucao::Executar => "acceptEdits",
            ModoDeExecucao::Planejar | ModoDeExecucao::Consultar => "plan",
        };
        let mut argumentos: Vec<String> = vec![
            "--print".into(), "--verbose".into(), "--output-format".into(), "stream-json".into(),
            "--permission-mode".into(), permissao.into(),
            "--disallowedTools".into(), "Bash".into(),
        ];
        if modo == ModoDeExecucao::Consultar {
            argumentos.extend(["--allowedTools", "WebSearch", "WebFetch", "Read", "Glob", "Grep"].map(String::from));
        }
        argumentos.extend(["--max-turns".into(), "8".into(), "--add-dir".into(), pasta_do_cofre.into()]);
        argumentos
    }

    /// O Claude Code pesquisa na web com `WebSearch` e `WebFetch`, liberadas só no modo `Consultar`.
    fn suporta_consulta_com_web(&self) -> bool { true }

    /// O stream publica cada mensagem do assistente assim que termina.
    fn transmite_resposta(&self) -> bool { true }

    /// Texto de uma mensagem do assistente. Termina com uma linha em branco para separar
    /// o texto de uma mensagem do da seguinte.
    fn ler_trecho_de_resposta(&self, linha: &str) -> Result<Option<String>, String> {
        let Ok(evento) = serde_json::from_str::<Value>(linha.trim()) else { return Ok(None) };
        if evento["type"] != "assistant" {
            return Ok(None);
        }
        let texto: String = blocos_da_mensagem(&evento)
            .iter()
            .filter(|bloco| bloco["type"] == "text")
            .filter_map(|bloco| bloco["text"].as_str())
            .collect();
        Ok(if texto.is_empty() { None } else { Some(format!("{texto}\n\n")) })
    }

    /// Primeira ferramenta usada em uma mensagem do assistente vira passo.
    fn ler_passo_de_resposta(&self, linha: &str) -> Option<String> {
        let evento: Value = serde_json::from_str(linha.trim()).ok()?;
        if evento["type"] != "assistant" {
            return None;
        }
        blocos_da_mensagem(&evento)
            .iter()
            .find(|bloco| bloco["type"] == "tool_use")
            .and_then(|bloco| bloco["name"].as_str())
            .map(rotulo_do_passo)
    }

    /// Usa o texto do evento `result`. Se ele não existir, junta o texto das mensagens do assistente.
    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let mut resposta_final = String::new();
        let mut texto_das_mensagens = String::new();
        for linha in saida.lines() {
            if let Ok(Some(trecho)) = self.ler_trecho_de_resposta(linha) {
                texto_das_mensagens.push_str(&trecho);
            }
            let Ok(evento) = serde_json::from_str::<Value>(linha.trim()) else { continue };
            if evento["type"] == "result" {
                if evento["is_error"].as_bool().unwrap_or(false) {
                    let motivo = evento["result"].as_str().unwrap_or("").trim();
                    return Err(if motivo.is_empty() { "Claude Code informou uma falha".into() } else { motivo.to_string() });
                }
                resposta_final = evento["result"].as_str().unwrap_or("").to_string();
            }
        }
        let escolhida = if resposta_final.trim().is_empty() { texto_das_mensagens } else { resposta_final };
        let escolhida = escolhida.trim();
        if escolhida.is_empty() { Err("Claude Code terminou sem escrever resposta".into()) }
        else { Ok(escolhida.to_string()) }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    const LINHA_DE_FERRAMENTA: &str = r#"{"type":"assistant","message":{"content":[{"type":"tool_use","id":"t1","name":"Read","input":{}}]}}"#;
    const LINHA_DE_TEXTO: &str = r#"{"type":"assistant","message":{"content":[{"type":"text","text":"Achei um arquivo."}]}}"#;

    #[test]
    fn ferramenta_usada_vira_passo_com_rotulo() {
        assert_eq!(AdaptadorClaude.ler_passo_de_resposta(LINHA_DE_FERRAMENTA), Some("Leu um arquivo".into()));
        assert_eq!(AdaptadorClaude.ler_passo_de_resposta(LINHA_DE_TEXTO), None);
    }

    #[test]
    fn texto_da_mensagem_entra_no_trecho() {
        let trecho = AdaptadorClaude.ler_trecho_de_resposta(LINHA_DE_TEXTO).unwrap();
        assert_eq!(trecho, Some("Achei um arquivo.\n\n".into()));
    }

    #[test]
    fn resposta_vem_do_evento_result_e_ignora_eventos_de_hook() {
        let saida = format!(
            "{}\n{}\n{}",
            r#"{"type":"system","subtype":"hook_started"}"#,
            LINHA_DE_TEXTO,
            r#"{"type":"result","is_error":false,"result":"Resposta final."}"#,
        );
        assert_eq!(AdaptadorClaude.ler_resposta(&saida), Ok("Resposta final.".into()));
    }

    #[test]
    fn erro_do_evento_result_vira_falha() {
        let saida = r#"{"type":"result","is_error":true,"result":"Limite de uso atingido"}"#;
        assert_eq!(AdaptadorClaude.ler_resposta(saida), Err("Limite de uso atingido".into()));
    }

    #[test]
    fn modo_consultar_libera_web_e_so_nele() {
        let consulta = AdaptadorClaude.montar_argumentos("C:\\cofre", ModoDeExecucao::Consultar);
        assert!(consulta.contains(&"WebSearch".to_string()));
        let planejar = AdaptadorClaude.montar_argumentos("C:\\cofre", ModoDeExecucao::Planejar);
        assert!(!planejar.contains(&"WebSearch".to_string()));
    }

    #[test]
    fn modo_planejar_nao_permite_editar() {
        let argumentos = AdaptadorClaude.montar_argumentos("C:\\cofre", ModoDeExecucao::Planejar);
        assert!(argumentos.windows(2).any(|par| par[0] == "--permission-mode" && par[1] == "plan"));
    }
}
