// Constelações do grafo: cada pasta com estrelas visíveis vira uma constelação, com nome,
// quantidade e o centro onde o rótulo aparece. Função pura, sem three.

import type { GrafoDoCofre } from "./contratoGrafo";
import type { VisaoDoGrafo } from "./filtrosDoGrafo";

export interface ConstelacaoDoGrafo {
  pasta: string;
  /** Nome curto para o rótulo: o último nível da pasta. */
  rotulo: string;
  /** Quantas estrelas visíveis a constelação tem. */
  quantidade: number;
  /** Centro das estrelas visíveis da pasta, em `[x, y, z]`. */
  centro: [number, number, number];
}

/** Nome do último nível da pasta. A raiz do cofre aparece como "Início". */
export function rotuloDaPasta(pasta: string): string {
  if (pasta === "") return "Início";
  return pasta.split("/").pop() ?? pasta;
}

/** Constelações com pelo menos uma estrela visível, em ordem alfabética da pasta. */
export function montarConstelacoes(grafo: GrafoDoCofre, visao: VisaoDoGrafo): ConstelacaoDoGrafo[] {
  const somaDasPosicoesPorPasta = new Map<string, { soma: [number, number, number]; quantidade: number }>();
  grafo.nos.forEach((no, posicao) => {
    if (!visao.nosVisiveis[posicao]) return;
    const acumulado = somaDasPosicoesPorPasta.get(no.pasta) ?? { soma: [0, 0, 0], quantidade: 0 };
    acumulado.soma = [acumulado.soma[0] + no.posicao[0], acumulado.soma[1] + no.posicao[1], acumulado.soma[2] + no.posicao[2]];
    acumulado.quantidade++;
    somaDasPosicoesPorPasta.set(no.pasta, acumulado);
  });

  return [...somaDasPosicoesPorPasta.entries()]
    .sort(([primeiraPasta], [segundaPasta]) => primeiraPasta.localeCompare(segundaPasta))
    .map(([pasta, { soma, quantidade }]) => ({
      pasta,
      rotulo: rotuloDaPasta(pasta),
      quantidade,
      centro: [soma[0] / quantidade, soma[1] / quantidade, soma[2] / quantidade],
    }));
}
