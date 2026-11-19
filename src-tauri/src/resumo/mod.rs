//! Resumo semanal: quanto o agente trabalhou e o que mudou no cofre nos últimos sete dias.
//! `calculo` faz a conta sobre dados prontos; `comandos` lê o histórico e as tarefas e entrega o resultado.

pub mod comandos;
mod calculo;
pub(crate) mod contrato;
