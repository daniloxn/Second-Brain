//! Adaptador do OpenCode CLI. Usa a execução sem interface interativa e restringe suas ferramentas.

use super::contrato::ModoDeExecucao;
use super::passos::rotulo_do_passo;
use super::provedor::ProvedorAgente;
use serde::Deserialize;

/// Perfil aplicado apenas ao processo do Segundo Cérebro: lê e edita o cofre, sem shell ou subagentes.
/// Há dois agentes: o de execução edita; o de planejamento só lê, para o modo `Planejar`.
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
    },
    "segundo-cerebro-planejar": {
      "mode": "primary",
      "description": "Lê o cofre e propõe um plano, sem editar nenhum arquivo.",
      "permissions": [
        { "action": "*", "resource": "*", "effect": "deny" },
        { "action": "read", "resource": "*", "effect": "allow" },
        { "action": "glob", "resource": "*", "effect": "allow" },
        { "action": "grep", "resource": "*", "effect": "allow" },
        { "action": "read", "resource": "*.env", "effect": "deny" },
        { "action": "read", "resource": "*.env.*", "effect": "deny" },
        { "action": "read", "resource": "_segredos/**", "effect": "deny" }
      ]
    }
  }
}"#;

/// Adaptador do OpenCode.
pub struct AdaptadorOpenCode;

/// Evento JSONL emitido pelo OpenCode: texto gerado, ferramenta usada ou erro.
#[derive(Deserialize)]
struct EventoDeSaidaOpenCode {
    #[serde(rename = "type")]
    tipo: String,
    part: Option<ParteDoOpenCode>,
    error: Option<serde_json::Value>,
}

/// Parte de um evento: texto incremental ou uma chamada de ferramenta.
#[derive(Deserialize)]
struct ParteDoOpenCode {
    #[serde(rename = "type")]
    tipo: String,
    text: Option<String>,
    tool: Option<String>,
    state: Option<EstadoDaFerramentaOpenCode>,
}

/// Situação de uma chamada de ferramenta. Só a conclusão vira passo, para não repetir a mesma ferramenta.
#[derive(Deserialize)]
struct EstadoDaFerramentaOpenCode {
    status: Option<String>,
}

impl ProvedorAgente for AdaptadorOpenCode {
    /// Nome apresentado no seletor de agente.
    fn nome(&self) -> &'static str { "OpenCode" }

    /// Executável procurado no PATH.
    fn comando(&self) -> &'static str { "opencode" }

    /// Usa a pasta atual do processo como diretório do projeto e escolhe o agente pelo modo.
    fn montar_argumentos(&self, _pasta_do_cofre: &str, modo: ModoDeExecucao) -> Vec<String> {
        let agente = match modo {
            ModoDeExecucao::Executar => "segundo-cerebro-app",
            // `Consultar` não chega aqui: o app recusa antes, porque o OpenCode não tem web configurada.
            ModoDeExecucao::Planejar | ModoDeExecucao::Consultar => "segundo-cerebro-planejar",
        };
        vec![
            "run".into(), "--standalone".into(), "--format".into(), "json".into(),
            "--agent".into(), agente.into(),
        ]
    }

    /// O CLI documenta o prompt como argumento posicional do comando `run`.
    fn prompt_como_argumento(&self) -> bool { true }

    /// JSONL publica as partes de texto assim que são geradas.
    fn transmite_resposta(&self) -> bool { true }

    /// Extrai somente o texto visível. Os passos de ferramenta vão por `ler_passo_de_resposta`.
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

    /// Uma ferramenta concluída vira passo ("Leu um arquivo" etc.). Eventos de outro tipo são ignorados.
    fn ler_passo_de_resposta(&self, linha: &str) -> Option<String> {
        let evento: EventoDeSaidaOpenCode = serde_json::from_str(linha.trim()).ok()?;
        let parte = evento.part.filter(|_| evento.tipo == "tool_use")?;
        let concluida = parte.state.and_then(|estado| estado.status).is_some_and(|status| status == "completed");
        if !concluida {
            return None;
        }
        parte.tool.map(|ferramenta| rotulo_do_passo(&ferramenta))
    }

    /// Aplica regras restritas à execução, sem sobrescrever a configuração pessoal persistida.
    fn configuracao_de_execucao(&self, _modo: ModoDeExecucao) -> Option<&'static str> { Some(CONFIGURACAO_RESTRITA) }

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

#[cfg(test)]
mod testes {
    use super::*;

    /// Evento real de ferramenta concluída, copiado da saída do OpenCode 2.3.2.
    const LINHA_DE_FERRAMENTA_CONCLUIDA: &str = r#"{"type":"tool_use","part":{"type":"tool","tool":"read","state":{"status":"completed","input":{"path":"nota.md"}}}}"#;
    const LINHA_DE_FERRAMENTA_EM_ANDAMENTO: &str = r#"{"type":"tool_use","part":{"type":"tool","tool":"read","state":{"status":"running"}}}"#;

    #[test]
    fn ferramenta_concluida_vira_passo() {
        assert_eq!(AdaptadorOpenCode.ler_passo_de_resposta(LINHA_DE_FERRAMENTA_CONCLUIDA), Some("Leu um arquivo".into()));
    }

    #[test]
    fn ferramenta_ainda_rodando_nao_vira_passo_duplicado() {
        assert_eq!(AdaptadorOpenCode.ler_passo_de_resposta(LINHA_DE_FERRAMENTA_EM_ANDAMENTO), None);
    }

    #[test]
    fn modo_planejar_usa_agente_sem_edicao() {
        let argumentos = AdaptadorOpenCode.montar_argumentos("C:\\cofre", ModoDeExecucao::Planejar);
        assert!(argumentos.contains(&"segundo-cerebro-planejar".to_string()));
        assert!(CONFIGURACAO_RESTRITA.contains("segundo-cerebro-planejar"));
    }
}
