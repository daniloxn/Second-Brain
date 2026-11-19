//! Formato do resumo semanal que a tela recebe. Em camelCase, como as outras funcionalidades.
//! Mudar um campo aqui exige mudar `src/resumo/contratoResumo.ts` na mesma alteração.

use serde::Serialize;

/// Números de uma semana (sete dias, até hoje). Os contadores de agente só contam o que o agente fez.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ResumoDaSemana {
    /// Primeiro e último dia da janela, no formato `AAAA-MM-DD`.
    pub inicio: String,
    pub fim: String,
    /// Pedidos feitos ao agente na semana (inclui os de planejamento).
    pub pedidos_do_agente: usize,
    /// Quantos desses pedidos têm tempo medido. Pedidos anteriores à medição não têm.
    pub pedidos_com_tempo_medido: usize,
    /// Soma do tempo das rodadas com tempo medido, em milissegundos.
    pub tempo_do_agente_em_ms: u64,
    /// Linhas que entraram ou saíram das notas e tarefas mudadas pelo agente (aproximação por linha).
    pub linhas_alteradas_pelo_agente: usize,
    pub notas_criadas_pelo_agente: usize,
    pub notas_alteradas_pelo_agente: usize,
    pub tarefas_criadas_pelo_agente: usize,
    /// Tarefas concluídas na semana, de qualquer origem.
    pub tarefas_concluidas: usize,
}
