//! Funcionalidade "tarefas": cada tarefa é um arquivo `.md` em `tarefas/`, com prazo,
//! prioridade e status no frontmatter.
//!
//! Camadas, da mais interna para a mais externa:
//! - `dominio` e `regras`: modelo e regras puras, sem arquivo nem relógio.
//! - `frontmatter`: leitura e escrita do bloco `---`.
//! - `repositorio`: único módulo que mexe em arquivos.
//! - `contrato`: formato que o frontend recebe e envia. Versionado.
//! - `servico`: casos de uso (listar, criar) compartilhados pelo app e pelo servidor MCP.
//! - `comandos` e `avisos`: a porta de entrada (comandos do Tauri) e o agendador de notificações.

pub mod avisos;
pub mod comandos;
pub(crate) mod contrato;
mod entrada_do_contrato;
pub(crate) mod servico;
mod situacao;
mod validacao;
mod valores;
mod dominio;
mod frontmatter;
mod regras;
mod repositorio;
