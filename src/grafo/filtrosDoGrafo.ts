// Regras dos filtros do grafo: quais pontos e ligações aparecem e quais ficam em destaque na busca.
// Funções puras, sem React nem three: recebem o grafo e os filtros e devolvem a visão.

import type { GrafoDoCofre } from "./contratoGrafo";

/** Escolhas do usuário no painel de filtros. */
export interface FiltrosDoGrafo {
  mostrarTarefas: boolean;
  mostrarIsoladas: boolean;
  /** Pasta (relativa ao cofre) que fica sozinha na tela, ou null para todas. */
  pastaEscolhida: string | null;
  busca: string;
}

export const FILTROS_INICIAIS_DO_GRAFO: FiltrosDoGrafo = {
  mostrarTarefas: true,
  mostrarIsoladas: true,
  pastaEscolhida: null,
  busca: "",
};

/** O que desenhar. Todas as listas seguem a ordem de `nos` e `ligacoes` do grafo. */
export interface VisaoDoGrafo {
  nosVisiveis: boolean[];
  /** Pontos que casam com a busca. Null quando não há busca: nada fica apagado. */
  nosEmDestaque: boolean[] | null;
  ligacoesVisiveis: boolean[];
}

/** Quantas notas a lista de resultados da busca mostra. */
export const LIMITE_DE_RESULTADOS_DA_BUSCA = 8;

/** Texto da busca pronto para comparar: sem espaços nas pontas e em minúsculas. */
function normalizarBusca(busca: string): string {
  return busca.trim().toLowerCase();
}

/** Filtros escondem pontos (tarefas, isolados, outras pastas); a busca só apaga os que não casam. */
export function montarVisaoDoGrafo(grafo: GrafoDoCofre, filtros: FiltrosDoGrafo): VisaoDoGrafo {
  const nosVisiveis = grafo.nos.map((no) => {
    if (no.tipo === "tarefa" && !filtros.mostrarTarefas) return false;
    if (no.conexoes === 0 && !filtros.mostrarIsoladas) return false;
    return filtros.pastaEscolhida === null || no.pasta === filtros.pastaEscolhida;
  });

  const buscaNormalizada = normalizarBusca(filtros.busca);
  const nosEmDestaque =
    buscaNormalizada === "" ? null : grafo.nos.map((no) => no.nome.toLowerCase().includes(buscaNormalizada));

  const ligacoesVisiveis = grafo.ligacoes.map((ligacao) => nosVisiveis[ligacao.origem] && nosVisiveis[ligacao.destino]);
  return { nosVisiveis, nosEmDestaque, ligacoesVisiveis };
}

/** Posições dos pontos visíveis que casam com a busca, até o limite. Vazio quando não há busca. */
export function resultadosDaBusca(visao: VisaoDoGrafo): number[] {
  if (visao.nosEmDestaque === null) return [];
  const posicoesEncontradas: number[] = [];
  for (let posicao = 0; posicao < visao.nosVisiveis.length; posicao++) {
    if (visao.nosVisiveis[posicao] && visao.nosEmDestaque[posicao]) posicoesEncontradas.push(posicao);
    if (posicoesEncontradas.length >= LIMITE_DE_RESULTADOS_DA_BUSCA) break;
  }
  return posicoesEncontradas;
}

/** Pastas que têm algo no grafo, em ordem alfabética. A raiz do cofre aparece como texto vazio. */
export function listarPastasDoGrafo(grafo: GrafoDoCofre): string[] {
  return [...new Set(grafo.nos.map((no) => no.pasta))].sort((primeira, segunda) => primeira.localeCompare(segunda));
}

/** Nome da pasta para mostrar na tela. */
export function nomeDaPastaParaExibir(pasta: string): string {
  return pasta === "" ? "(raiz do cofre)" : pasta;
}

/** Quantos pontos e ligações estão visíveis, para o resumo do painel. */
export function contarVisiveis(visao: VisaoDoGrafo): { nos: number; ligacoes: number } {
  return {
    nos: visao.nosVisiveis.filter(Boolean).length,
    ligacoes: visao.ligacoesVisiveis.filter(Boolean).length,
  };
}
