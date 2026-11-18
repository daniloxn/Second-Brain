//! Adaptador do Codex CLI em modo não interativo e restrito à pasta do cofre.

use super::provedor::ProvedorAgente;

/// Adaptador do Codex CLI.
pub struct AdaptadorCodex;

impl ProvedorAgente for AdaptadorCodex {
    fn nome(&self) -> &'static str { "Codex" }

    fn comando(&self) -> &'static str { "codex" }

    /// Usa o sandbox workspace-write e envia o prompt pela entrada padrão.
    fn montar_argumentos(&self, _pasta_do_cofre: &str) -> Vec<String> {
        vec!["exec".into(), "--sandbox".into(), "workspace-write".into(), "--ask-for-approval".into(), "never".into(), "-".into()]
    }

    /// `codex exec` imprime a resposta final em stdout.
    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let resposta = saida.trim();
        if resposta.is_empty() { Err("Codex terminou sem escrever resposta".into()) }
        else { Ok(resposta.to_string()) }
    }
}
