//! Funcionalidade "agente": conversa com um CLI de IA (hoje o Antigravity, pelo comando `agy`),
//! com a pasta do cofre como área de trabalho, e registro de tudo o que ele muda.
//! Este arquivo só declara as partes. Cada uma cuida de uma coisa.

pub mod aquecimento;
pub mod comandos;
mod agy;
mod contrato;
mod desfazer;
mod diferencas;
mod executar;
mod foto;
mod historico;
mod prompt;
mod provedor;
