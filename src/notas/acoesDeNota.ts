// Ações de nota que combinam backend e regras do app: criar nota na pasta padrão
// e abrir a nota de um wikilink, criando-a se ainda não existir.

import { criarNota, listarNotas, PASTA_RELATIVA_DAS_NOVAS_NOTAS } from "./chamadasNotas";
import { acharNotaPeloNome, nomeDaNotaNoAlvo } from "./nomesDeNota";

/** Caminho absoluto da pasta onde as notas novas são criadas. */
export function pastaDasNovasNotas(pastaDoCofre: string): string {
  return `${pastaDoCofre}/${PASTA_RELATIVA_DAS_NOVAS_NOTAS}`;
}

/**
 * Cria uma nota nova com o nome digitado, na pasta padrão.
 * Devolve o caminho da nota criada. Lança erro com a mensagem do backend se o nome for inválido.
 */
export function criarNovaNotaNaPastaPadrao(pastaDoCofre: string, nomeDigitado: string): Promise<string> {
  return criarNota(pastaDasNovasNotas(pastaDoCofre), nomeDigitado);
}

/**
 * Caminho da nota que um wikilink aponta. Confere o índice mais recente do backend
 * e, se a nota não existir, cria em `notas/anotacoes`.
 */
export async function abrirOuCriarNotaPeloLink(pastaDoCofre: string, alvoDoLink: string): Promise<string> {
  const notasAtuais = await listarNotas();
  const nomeDaNota = nomeDaNotaNoAlvo(alvoDoLink);
  const notaExistente = acharNotaPeloNome(notasAtuais, nomeDaNota);
  if (notaExistente) return notaExistente.caminho;
  return criarNota(pastaDasNovasNotas(pastaDoCofre), nomeDaNota);
}
