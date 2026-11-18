//! Cofre de segredos: credenciais cifradas em arquivo separado dos Markdown.
//! A senha mestra só chega em comandos explícitos e nunca é salva.

pub mod comandos;
mod contrato;
mod criptografia;
mod repositorio;
