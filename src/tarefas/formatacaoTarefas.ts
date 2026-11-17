// Textos e agrupamentos de tela para tarefas. Funções puras: só transformam dados
// que o backend já calculou (situação, ordem). Trocar um rótulo aqui não afeta o backend.

import type { PrioridadeDaTarefa, SituacaoDaTarefa, StatusDaTarefa, TarefaDoContrato } from "./contratoTarefas";

/** Ordem em que os grupos aparecem na tela. */
const ORDEM_DOS_GRUPOS: SituacaoDaTarefa[] = ["atrasada", "hoje", "proxima", "sem_prazo", "concluida"];

const ROTULO_DA_SITUACAO: Record<SituacaoDaTarefa, string> = {
  atrasada: "Atrasadas",
  hoje: "Hoje",
  proxima: "Próximas",
  sem_prazo: "Sem prazo",
  concluida: "Concluídas",
};

const ROTULO_DA_PRIORIDADE: Record<PrioridadeDaTarefa, string> = {
  alta: "Alta",
  media: "Média",
  baixa: "Baixa",
};

const ROTULO_DO_STATUS: Record<StatusDaTarefa, string> = {
  aberta: "Aberta",
  fazendo: "Fazendo",
  feita: "Feita",
  arquivada: "Arquivada",
};

/** Abreviações dos meses em português, indexadas de 0 a 11. */
const MESES_ABREVIADOS = ["jan", "fev", "mar", "abr", "mai", "jun", "jul", "ago", "set", "out", "nov", "dez"];

/** Grupo da lista: uma situação e as tarefas dela, na ordem que o backend mandou. */
export interface GrupoDeTarefas {
  situacao: SituacaoDaTarefa;
  rotulo: string;
  tarefas: TarefaDoContrato[];
}

export function rotuloDaPrioridade(prioridade: PrioridadeDaTarefa): string {
  return ROTULO_DA_PRIORIDADE[prioridade];
}

export function rotuloDoStatus(status: StatusDaTarefa): string {
  return ROTULO_DO_STATUS[status];
}

/**
 * Prazo em texto curto. Ex.: `2026-11-18` vira `18 nov`; `2026-11-18 14:00` vira `18 nov · 14:00`.
 * Sem prazo, devolve texto vazio.
 */
export function formatarPrazo(prazoDoContrato: string | null): string {
  if (!prazoDoContrato) return "";
  const [dataTexto, horaTexto] = prazoDoContrato.split(" ");
  const [, mesTexto, diaTexto] = dataTexto.split("-");
  const rotuloDaData = `${Number(diaTexto)} ${MESES_ABREVIADOS[Number(mesTexto) - 1] ?? mesTexto}`;
  return horaTexto ? `${rotuloDaData} · ${horaTexto}` : rotuloDaData;
}

/**
 * Separa as tarefas em grupos pela situação calculada pelo backend.
 * Grupos vazios não aparecem. A ordem dentro de cada grupo é a que o backend mandou.
 */
export function agruparPorSituacao(tarefas: TarefaDoContrato[]): GrupoDeTarefas[] {
  return ORDEM_DOS_GRUPOS.map((situacao) => ({
    situacao,
    rotulo: ROTULO_DA_SITUACAO[situacao],
    tarefas: tarefas.filter((tarefa) => tarefa.situacao === situacao),
  })).filter((grupo) => grupo.tarefas.length > 0);
}
