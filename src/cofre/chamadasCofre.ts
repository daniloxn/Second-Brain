// Chamadas ao backend da funcionalidade cofre: abrir, criar e montar a árvore de arquivos,
// além das janelas do sistema para escolher pastas. Nenhum componente chama `invoke` direto.

import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { documentDir } from "@tauri-apps/api/path";

/** Um item da árvore de arquivos. Espelha `NoArvore` em src-tauri/src/cofre/arvore.rs. */
export interface NoArvore {
  nome: string;
  caminho: string;
  pasta: boolean;
  filhos: NoArvore[];
}

/**
 * Resposta de `abrir_cofre`. Se `precisaConfirmar` for verdadeiro, a pasta não é um cofre do app
 * e nada foi feito: o usuário precisa confirmar antes de o app criar as pastas dele ali.
 */
export interface AberturaDoCofre {
  caminho: string | null;
  precisaConfirmar: boolean;
}

/** Caminho do cofre ativo, ou null se não há cofre válido salvo. */
export const cofreAtual = () => invoke<string | null>("cofre_atual");

/** Abre uma pasta como cofre. Com `converterPasta` falso, pastas comuns são recusadas e nada é criado. */
export const abrirCofre = (caminhoDaPasta: string, converterPasta: boolean) =>
  invoke<AberturaDoCofre>("abrir_cofre", { caminho: caminhoDaPasta, converterPasta });

/** Cria um cofre novo com a estrutura padrão dentro da pasta indicada e devolve o caminho do cofre. */
export const criarCofre = (pastaPai: string, nomeDoCofre: string) =>
  invoke<string>("criar_cofre", { pastaPai, nome: nomeDoCofre });

/** Cria uma pasta dentro do cofre ativo. Aceita vários níveis, como `projetos/app`. */
export const criarPastaNoCofre = (caminhoRelativo: string) =>
  invoke<string>("criar_pasta_no_cofre", { caminhoRelativo });

/** Árvore de arquivos de uma pasta dentro do cofre. */
export const listarArvore = (caminhoDaPasta: string) =>
  invoke<NoArvore>("listar_arvore", { caminho: caminhoDaPasta });

/** Pasta Documentos do usuário, usada como local sugerido para criar um cofre. */
export const pastaDeDocumentos = () => documentDir();

/** Abre a janela do sistema para escolher uma pasta. Devolve null se o usuário cancelar. */
export async function escolherPastaDoCofre(tituloDaJanela: string): Promise<string | null> {
  const pastaEscolhida = await open({
    directory: true,
    multiple: false,
    title: tituloDaJanela,
  });
  return typeof pastaEscolhida === "string" ? pastaEscolhida : null;
}
