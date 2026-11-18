//! Adaptador do Claude Code em modo de impressão, sem ferramenta de terminal.

use super::provedor::ProvedorAgente;
use serde::Deserialize;

/// Adaptador do Claude Code.
pub struct AdaptadorClaude;

/// Campos da resposta JSON do Claude Code.
#[derive(Deserialize)]
struct SaidaClaude { result: Option<String>, is_error: Option<bool> }

impl ProvedorAgente for AdaptadorClaude {
    fn nome(&self) -> &'static str { "Claude Code" }

    fn comando(&self) -> &'static str { "claude" }

    /// Permite leitura e edição de arquivos, limita turnos e bloqueia Bash.
    fn montar_argumentos(&self, pasta_do_cofre: &str) -> Vec<String> {
        vec!["--print".into(), "--output-format".into(), "json".into(), "--permission-mode".into(), "acceptEdits".into(), "--disallowedTools".into(), "Bash".into(), "--max-turns".into(), "8".into(), "--add-dir".into(), pasta_do_cofre.into()]
    }

    /// Extrai o texto final do envelope JSON.
    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let envelope: SaidaClaude = serde_json::from_str(saida.trim()).map_err(|erro| format!("resposta inesperada do Claude Code: {erro}"))?;
        if envelope.is_error.unwrap_or(false) { return Err(envelope.result.unwrap_or_else(|| "Claude Code informou uma falha".into())); }
        envelope.result.filter(|resposta| !resposta.trim().is_empty()).ok_or_else(|| "Claude Code terminou sem escrever resposta".into())
    }
}
