//! Adaptador do OpenCode CLI. Usa a execução sem interface interativa e restringe suas ferramentas.

use super::provedor::ProvedorAgente;
use serde::Deserialize;

/// Perfil aplicado apenas ao processo do Segundo Cérebro: lê e edita o cofre, sem shell ou subagentes.
const CONFIGURACAO_RESTRITA: &str = r#"{
  "default_agent": "segundo-cerebro-app",
  "permissions": [
    { "action": "external_directory", "resource": "*", "effect": "deny" }
  ],
  "agents": {
    "segundo-cerebro-app": {
      "mode": "primary",
      "description": "Trabalha apenas com arquivos do cofre do Segundo Cérebro.",
      "permissions": [
        { "action": "*", "resource": "*", "effect": "deny" },
        { "action": "read", "resource": "*", "effect": "allow" },
        { "action": "edit", "resource": "*", "effect": "allow" },
        { "action": "glob", "resource": "*", "effect": "allow" },
        { "action": "grep", "resource": "*", "effect": "allow" },
        { "action": "read", "resource": "*.env", "effect": "deny" },
        { "action": "read", "resource": "*.env.*", "effect": "deny" },
        { "action": "read", "resource": "_segredos/**", "effect": "deny" },
        { "action": "edit", "resource": "_segredos/**", "effect": "deny" }
      ]
    }
  }
}"#;

/// Adaptador do OpenCode.
pub struct AdaptadorOpenCode;

/// Evento JSONL emitido quando OpenCode publica texto gerado.
#[derive(Deserialize)]
struct EventoDeSaidaOpenCode {
    #[serde(rename = "type")]
    tipo: String,
    part: Option<ParteDeTextoOpenCode>,
    error: Option<serde_json::Value>,
}

/// Parte de texto incremental da resposta do modelo.
#[derive(Deserialize)]
struct ParteDeTextoOpenCode {
    #[serde(rename = "type")]
    tipo: String,
    text: Option<String>,
}

impl ProvedorAgente for AdaptadorOpenCode {
    /// Nome apresentado no seletor de agente.
    fn nome(&self) -> &'static str { "OpenCode" }

    /// Executável procurado no PATH.
    fn comando(&self) -> &'static str { "opencode" }

    /// Usa a pasta atual do processo como diretório do projeto e aplica o perfil restrito.
    fn montar_argumentos(&self, _pasta_do_cofre: &str) -> Vec<String> {
        vec![
            "run".into(), "--standalone".into(), "--format".into(), "json".into(),
            "--agent".into(), "segundo-cerebro-app".into(),
        ]
    }

    /// O CLI documenta o prompt como argumento posicional do comando `run`.
    fn prompt_como_argumento(&self) -> bool { true }

    /// JSONL publica as partes de texto assim que são geradas.
    fn transmite_resposta(&self) -> bool { true }

    /// Extrai somente o texto visível, descartando eventos de passos e ferramentas.
    fn ler_trecho_de_resposta(&self, linha: &str) -> Result<Option<String>, String> {
        let evento: EventoDeSaidaOpenCode = serde_json::from_str(linha.trim())
            .map_err(|erro| format!("evento JSON inesperado do OpenCode: {erro}"))?;
        if evento.tipo == "error" {
            return Err(evento.error.map(|erro| erro.to_string()).unwrap_or_else(|| "OpenCode retornou um erro".into()));
        }
        Ok(evento.part
            .filter(|parte| evento.tipo == "text" && parte.tipo == "text")
            .and_then(|parte| parte.text)
            .filter(|texto| !texto.is_empty()))
    }

    /// Aplica regras restritas à execução, sem sobrescrever a configuração pessoal persistida.
    fn configuracao_de_execucao(&self) -> Option<&'static str> { Some(CONFIGURACAO_RESTRITA) }

    /// Reúne os eventos JSONL para gerar a mesma resposta final registrada no histórico.
    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let mut resposta = String::new();
        for linha in saida.lines().filter(|linha| !linha.trim().is_empty()) {
            if let Some(trecho) = self.ler_trecho_de_resposta(linha)? {
                resposta.push_str(&trecho);
            }
        }
        if resposta.trim().is_empty() { Err("OpenCode terminou sem escrever resposta".into()) }
        else { Ok(resposta.trim().to_string()) }
    }
}
