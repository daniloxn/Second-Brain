//! Funcionalidade "tarefas": cada tarefa é um arquivo `.md` em `tarefas/`, com prazo,
//! prioridade e status no frontmatter.
//!
//! Camadas, da mais interna para a mais externa:
//! - `dominio` e `regras`: modelo e regras puras, sem arquivo nem relógio.
//! - `frontmatter`: leitura e escrita do bloco `---`.
//! - `repositorio`: único módulo que mexe em arquivos.
//! - `contrato`: formato que o frontend recebe e envia. Versionado.
//! - `comandos` e `avisos`: a porta de entrada (comandos do Tauri) e o agendador de notificações.

pub mod avisos;
pub mod comandos;
mod contrato;
mod entrada_do_contrato;
mod situacao;
mod validacao;
mod valores;
mod dominio;
mod frontmatter;
mod regras;
mod repositorio;
