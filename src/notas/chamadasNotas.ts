// Chamadas ao backend da funcionalidade notas: ler, salvar, criar, listar e backlinks.
// Os tipos abaixo espelham as structs de src-tauri/src/notas/.

import { invoke } from "@tauri-apps/api/core";

/** Uma nota do índice: nome sem extensão e caminho completo. */
export interface NotaIndice {
  nome: string;
  caminho: string;
}

/** Uma linha de outra nota que aponta para a nota aberta. */
export interface Backlink {
  nome: string;
  caminho: string;
  linha: number;
  trecho: string;
}

/** Pasta, relativa à raiz do cofre, onde as notas novas são criadas (PLANO.md, seção 3). */
export const PASTA_RELATIVA_DAS_NOVAS_NOTAS = "notas/anotacoes";

/** Todas as notas do cofre, em ordem alfabética. */
export const listarNotas = () => invoke<NotaIndice[]>("listar_notas");

/** Texto de uma nota e a versão do arquivo no momento da leitura (data de modificação em ms). */
export interface NotaLida {
  texto: string;
  versao: number;
}

/** Resultado de um salvamento. `gravado: false` quer dizer que a nota mudou no disco depois da versão conhecida. */
export interface ResultadoDoSalvamento {
  gravado: boolean;
  versao: number;
}

/** Texto completo de uma nota, com a versão do arquivo. */
export const lerNota = (caminhoDaNota: string) =>
  invoke<NotaLida>("ler_nota", { caminho: caminhoDaNota });

/** Grava o texto inteiro da nota só se ela ainda estiver na `versaoConhecida`. */
export const salvarNota = (caminhoDaNota: string, textoDaNota: string, versaoConhecida: number) =>
  invoke<ResultadoDoSalvamento>("salvar_nota", {
    caminho: caminhoDaNota,
    conteudo: textoDaNota,
    versaoConhecida,
  });

/** Guarda um texto que não pôde ser salvo, em `_sistema/conflitos/`. Devolve o caminho da cópia. */
export const guardarCopiaDoConflito = (caminhoDaNota: string, textoDaNota: string) =>
  invoke<string>("guardar_copia_do_conflito", { caminho: caminhoDaNota, conteudo: textoDaNota });

/** Cria uma nota com título inicial na pasta indicada e devolve o caminho do arquivo. */
export const criarNota = (pastaDeDestino: string, nomeDaNota: string) =>
  invoke<string>("criar_nota", { pasta: pastaDeDestino, nome: nomeDaNota });

/** Notas que contêm wikilinks para a nota indicada. */
export const backlinksDaNota = (caminhoDaNota: string) =>
  invoke<Backlink[]>("backlinks", { caminho: caminhoDaNota });
