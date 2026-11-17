// Contrato de tarefas no lado do frontend. Espelha src-tauri/src/tarefas/contrato.rs.
// Este é o único arquivo do frontend que conhece o formato das tarefas: telas e hooks
// usam os tipos e as funções daqui, nunca `invoke` direto nem o formato do arquivo.

import { invoke } from "@tauri-apps/api/core";

/** Versão do contrato que este frontend sabe ler. Se o backend mandar outra, a lista é recusada. */
export const VERSAO_DO_CONTRATO_ESPERADA = 1;

export type StatusDaTarefa = "aberta" | "fazendo" | "feita" | "arquivada";
export type PrioridadeDaTarefa = "alta" | "media" | "baixa";
export type SituacaoDaTarefa = "atrasada" | "hoje" | "proxima" | "sem_prazo" | "concluida";

/** Uma tarefa como o backend a envia. `situacao` já vem calculada. */
export interface TarefaDoContrato {
  id: string;
  titulo: string;
  descricao: string;
  status: StatusDaTarefa;
  prioridade: PrioridadeDaTarefa;
  /** "AAAA-MM-DD" ou "AAAA-MM-DD HH:MM". Nulo quando não há prazo. */
  prazo: string | null;
  origem: string;
  tags: string[];
  situacao: SituacaoDaTarefa;
}

/** Resposta de `listar_tarefas`. */
export interface ListaDeTarefasDoContrato {
  versao: number;
  tarefas: TarefaDoContrato[];
}

/** Campos para criar uma tarefa. Só o título é obrigatório. */
export interface NovaTarefaParaCriar {
  titulo: string;
  descricao?: string;
  prioridade?: PrioridadeDaTarefa;
  /** "AAAA-MM-DD" ou "AAAA-MM-DD HH:MM". Vazio ou ausente = sem prazo. */
  prazo?: string;
  origem?: string;
  tags?: string[];
}

/** Alterações parciais. Campo ausente não muda; `prazo: ""` remove o prazo. */
export interface AlteracaoDaTarefa {
  titulo?: string;
  descricao?: string;
  status?: StatusDaTarefa;
  prioridade?: PrioridadeDaTarefa;
  prazo?: string;
  origem?: string;
  tags?: string[];
}

/** Lista as tarefas. Lança erro se a versão do contrato não for a esperada. */
export async function listarTarefasDoCofre(): Promise<TarefaDoContrato[]> {
  const resposta = await invoke<ListaDeTarefasDoContrato>("listar_tarefas");
  if (resposta.versao !== VERSAO_DO_CONTRATO_ESPERADA) {
    throw new Error(
      `O backend usa o contrato de tarefas v${resposta.versao}, e o frontend espera v${VERSAO_DO_CONTRATO_ESPERADA}.`,
    );
  }
  return resposta.tarefas;
}

/** Cria uma tarefa e devolve a tarefa criada. */
export const criarTarefaNoCofre = (entrada: NovaTarefaParaCriar) =>
  invoke<TarefaDoContrato>("criar_tarefa", { entrada });

/** Altera uma tarefa pelo id e devolve a tarefa já atualizada. */
export const atualizarTarefaNoCofre = (idDaTarefa: string, alteracao: AlteracaoDaTarefa) =>
  invoke<TarefaDoContrato>("atualizar_tarefa", { id: idDaTarefa, alteracao });
