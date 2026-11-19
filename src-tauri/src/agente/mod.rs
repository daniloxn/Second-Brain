//! Funcionalidade "agente": conversa com um CLI de IA (OpenCode por padrão),
//! com a pasta do cofre como área de trabalho, e registro de tudo o que ele muda.
//! Este arquivo só declara as partes. Cada uma cuida de uma coisa.

pub mod aquecimento;
mod antigravity;
mod aviso;
pub mod comandos;
mod claude;
mod codex;
pub(crate) mod contrato;
mod desfazer;
mod diferencas;
mod executar;
mod foto;
pub(crate) mod historico;
mod passos;
mod prompt;
mod provedor;
mod ollama;
mod opencode;
