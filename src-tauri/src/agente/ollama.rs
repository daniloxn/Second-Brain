//! Adaptador de conversa local pelo Ollama. Não concede ferramentas para alterar arquivos.

use super::contrato::ModoDeExecucao;
use super::provedor::ProvedorAgente;

/// Adaptador de modelo local do Ollama.
pub struct AdaptadorOllama;

impl ProvedorAgente for AdaptadorOllama {
    fn nome(&self) -> &'static str { "Ollama local" }

    fn comando(&self) -> &'static str { "ollama" }

    /// Executa o modelo local padrão sem habilitar ferramentas de sistema de arquivos.
    /// Não tem ferramentas, então o modo não muda nada.
    fn montar_argumentos(&self, _pasta_do_cofre: &str, _modo: ModoDeExecucao) -> Vec<String> {
        vec!["run".into(), "llama3.2".into()]
    }

    /// Ollama devolve texto simples no modo `run`.
    fn ler_resposta(&self, saida: &str) -> Result<String, String> {
        let resposta = saida.trim();
        if resposta.is_empty() { Err("Ollama terminou sem resposta. Confira se o modelo llama3.2 está baixado".into()) }
        else { Ok(resposta.to_string()) }
    }
}
