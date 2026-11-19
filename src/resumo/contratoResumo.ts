// Tipo do resumo semanal, espelhado de src-tauri/src/resumo/contrato.rs. Ao mudar um campo lá, mude aqui na mesma alteração.

/** Números de uma semana (sete dias, até hoje). Os contadores de agente só contam o que o agente fez. */
export interface ResumoDaSemana {
  /** Primeiro e último dia da janela, no formato `AAAA-MM-DD`. */
  inicio: string;
  fim: string;
  pedidosDoAgente: number;
  pedidosComTempoMedido: number;
  tempoDoAgenteEmMs: number;
  linhasAlteradasPeloAgente: number;
  notasCriadasPeloAgente: number;
  notasAlteradasPeloAgente: number;
  tarefasCriadasPeloAgente: number;
  tarefasConcluidas: number;
}
