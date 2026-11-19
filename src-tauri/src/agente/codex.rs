//! Adaptador do Codex CLI em modo não interativo e restrito à pasta do cofre.

use super::contrato::ModoDeExecucao;
use super::provedor::ProvedorAgente;

/// Adaptador do Codex CLI.
pub struct AdaptadorCodex;

impl ProvedorAgente for AdaptadorCodex {
    fn nome(&self) -> &'static str { "Codex" }

    fn comando(&self) -> &'static str { "codex" }

    /// Executar usa o sandbox workspace-write. Planejar usa read-only, que impede qualquer escrita.
    /// O prompt entra pela entrada padrão.
    fn montar_argumentos(&self, _pasta_do_cofre: &str, modo: ModoDeExecucao) -> Vec<String> {
        let sandbox = match modo {
            ModoDeExecucao::Executar => "workspace-write",
            // `Consultar` não chega aqui: o app recusa antes, porque o Codex não tem web configurada.
            ModoDeExecucao::Planejar | ModoDeExecucao::Consultar => "read-only",
        };
        vec!["exec".into(), "--sandbox".into(), sandbox.into(), "--ask-for-approval".into(), "never".into(), "-".into()]
    }

    /// `codex exec` imprime a resposta final em stdout.
    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let resposta = saida.trim();
        if resposta.is_empty() { Err("Codex terminou sem escrever resposta".into()) }
        else { Ok(resposta.to_string()) }
    }
}
