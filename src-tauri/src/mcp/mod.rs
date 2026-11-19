//! Servidor MCP do Segundo Cérebro. Deixa o Claude Code, o Codex e o OpenCode usarem as tarefas
//! e as notas do cofre como ferramentas, por JSON-RPC na entrada e saída padrão.
//! Roda como processo próprio, sem janela e sem o Tauri aberto: lê o cofre direto do disco.
//!
//! Camadas, da mais interna para a mais externa:
//! - `protocolo`: formato das mensagens JSON-RPC (funções puras).
//! - `ferramentas`: nome, descrição e chamada de cada ferramenta, delegando ao domínio.
//! - `cofre_ativo`: qual pasta é o cofre (`--cofre` ou o config.json do app).
//! - `servidor`: laço de leitura e saída, e o despacho de cada método.

mod cofre_ativo;
mod ferramentas;
mod protocolo;
mod servidor;

pub use servidor::executar_servidor;
