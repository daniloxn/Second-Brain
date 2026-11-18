//! Adaptador do OpenCode CLI. Usa a execução sem interface interativa e restringe suas ferramentas.

use super::provedor::ProvedorAgente;

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

impl ProvedorAgente for AdaptadorOpenCode {
    /// Nome apresentado no seletor de agente.
    fn nome(&self) -> &'static str { "OpenCode" }

    /// Executável procurado no PATH.
    fn comando(&self) -> &'static str { "opencode" }

    /// Usa a pasta atual do processo como diretório do projeto e aplica o perfil restrito.
    fn montar_argumentos(&self, _pasta_do_cofre: &str) -> Vec<String> {
        vec![
            "run".into(), "--standalone".into(), "--format".into(), "default".into(),
            "--agent".into(), "segundo-cerebro-app".into(),
        ]
    }

    /// O CLI documenta o prompt como argumento posicional do comando `run`.
    fn prompt_como_argumento(&self) -> bool { true }

    /// Aplica regras restritas à execução, sem sobrescrever a configuração pessoal persistida.
    fn configuracao_de_execucao(&self) -> Option<&'static str> { Some(CONFIGURACAO_RESTRITA) }

    /// O modo padrão imprime a resposta final em texto, sem eventos JSON intermediários.
    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let resposta = saida.trim();
        if resposta.is_empty() { Err("OpenCode terminou sem escrever resposta".into()) }
        else { Ok(resposta.to_string()) }
    }
}
