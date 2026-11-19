// Janelas do sistema para anexar arquivos e escolher a pasta de destino. Os componentes não abrem
// o diálogo direto; a cópia dos arquivos passa pela importação de documentos, que já existe.

import { open } from "@tauri-apps/plugin-dialog";
import { importarDocumentos } from "../../documentos/chamadasDocumentos";
import type { AnexoDoPedido } from "./regrasDosAnexos";

/** Abre a janela de escolha de arquivos e copia os escolhidos para `documentos/`. Cancelar devolve lista vazia. */
export async function escolherEImportarAnexos(): Promise<AnexoDoPedido[]> {
  const escolhidos = await open({ multiple: true, directory: false });
  if (!escolhidos) return [];
  const caminhos = Array.isArray(escolhidos) ? escolhidos : [escolhidos];
  const importados = await importarDocumentos(caminhos);
  return importados.map((documento) => ({ nome: documento.nome, caminhoRelativo: documento.caminhoRelativo }));
}

/** Abre a janela de escolha de pasta, começando no cofre. Devolve o caminho absoluto, ou null se cancelou. */
export async function escolherPastaDeDestino(cofre: string): Promise<string | null> {
  const escolhida = await open({ directory: true, multiple: false, defaultPath: cofre });
  return typeof escolhida === "string" ? escolhida : null;
}
