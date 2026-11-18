// Cores e tamanhos das estrelas do grafo. Cada pasta (constelação) tem uma cor de luz de estrela;
// tarefas usam a cor de ação do app. Não desenha nada: só responde "de que cor e de que tamanho".

import { Color } from "three";
import type { NoDoGrafo } from "./contratoGrafo";

/** Cor das tarefas: laranja vivo, parecido com o --color-accent do app. */
const COR_DAS_TAREFAS = "#ff8a3d";

/** Cor da luz que une as estrelas e dos pontos sem pasta conhecida. */
export const COR_DA_LUZ_DAS_LIGACOES = "#9db4d6";

/** Cores de luz de estrela, em ordem: dourada, azul, violeta, turquesa, rosa, creme, verde-limão. */
const CORES_DAS_CONSTELACOES = ["#ffd9a0", "#a9c7ff", "#d6b8ff", "#9ff0d8", "#ff9fb2", "#fff1c9", "#d2f59a"];

/** Diâmetro mínimo de uma estrela e quanto ele cresce com a raiz do número de conexões. */
const TAMANHO_MINIMO_DA_ESTRELA = 5;
const CRESCIMENTO_DO_TAMANHO_POR_CONEXAO = 2.2;

/** Cor de cada pasta do grafo. */
export type PaletaDasPastas = Map<string, Color>;

/** Verdadeiro para `tarefas` e suas subpastas, a mesma regra que o backend usa para o tipo do ponto. */
function ePastaDeTarefas(pasta: string): boolean {
  return pasta === "tarefas" || pasta.startsWith("tarefas/");
}

/** Monta a paleta a partir das pastas em ordem alfabética: a cor depende da posição na lista. */
export function criarPaletaDasPastas(pastasEmOrdem: string[]): PaletaDasPastas {
  const paleta: PaletaDasPastas = new Map();
  pastasEmOrdem.forEach((pasta, posicao) => {
    const cor = ePastaDeTarefas(pasta) ? COR_DAS_TAREFAS : CORES_DAS_CONSTELACOES[posicao % CORES_DAS_CONSTELACOES.length];
    paleta.set(pasta, new Color(cor));
  });
  return paleta;
}

/** Cor de uma estrela: a da pasta dela. */
export function corDoNo(no: NoDoGrafo, paleta: PaletaDasPastas): Color {
  return paleta.get(no.pasta)?.clone() ?? new Color(COR_DA_LUZ_DAS_LIGACOES);
}

/** Cor da pasta como texto CSS, para as bolinhas do painel combinarem com o desenho. */
export function corDaPastaEmCss(pasta: string, paleta: PaletaDasPastas): string {
  return `#${(paleta.get(pasta) ?? new Color(COR_DA_LUZ_DAS_LIGACOES)).getHexString()}`;
}

/** Estrelas com mais ligações brilham maiores, para os nós centrais se destacarem. */
export function tamanhoDaEstrela(no: NoDoGrafo): number {
  return TAMANHO_MINIMO_DA_ESTRELA + Math.sqrt(no.conexoes) * CRESCIMENTO_DO_TAMANHO_POR_CONEXAO;
}
