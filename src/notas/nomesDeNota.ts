// Funções puras para nomes e caminhos de nota. Não tocam no disco nem no backend,
// então podem ser usadas em qualquer componente sem efeito colateral.

import type { NotaIndice } from "./chamadasNotas";

/** Nome de uma nota a partir do caminho. Ex.: `C:/cofre/ideia.md` vira `ideia`. */
export function nomeDaNotaPeloCaminho(caminhoDaNota: string): string {
  const ultimoTrechoDoCaminho = caminhoDaNota.split(/[\\/]/).pop() ?? caminhoDaNota;
  return ultimoTrechoDoCaminho.replace(/\.md$/i, "");
}

/** Caminho de uma nota relativo à pasta do cofre. Ex.: `notas/anotacoes/ideia.md`. */
export function caminhoRelativoAoCofre(caminhoDaNota: string, pastaDoCofre: string): string {
  if (!caminhoDaNota.startsWith(pastaDoCofre)) return caminhoDaNota;
  return caminhoDaNota.slice(pastaDoCofre.length).replace(/^[\\/]/, "");
}

/**
 * Nome da nota que um alvo de wikilink aponta, preservando maiúsculas.
 * Ex.: `pasta/Projeto X.md` vira `Projeto X`.
 */
export function nomeDaNotaNoAlvo(alvoDoLink: string): string {
  const ultimoTrechoDoAlvo = alvoDoLink.split(/[\\/]/).pop() ?? alvoDoLink;
  return ultimoTrechoDoAlvo.replace(/\.md$/i, "");
}

/**
 * Procura uma nota pelo nome, sem diferenciar maiúsculas.
 * Se houver mais de uma com o mesmo nome, devolve a primeira da lista.
 */
export function acharNotaPeloNome(notas: NotaIndice[], nomeProcurado: string): NotaIndice | undefined {
  const nomeEmMinusculas = nomeProcurado.toLowerCase();
  return notas.find((nota) => nota.nome.toLowerCase() === nomeEmMinusculas);
}
