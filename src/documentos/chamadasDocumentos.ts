// Camada de chamada Tauri da importação; os componentes não usam invoke diretamente.

import { invoke } from "@tauri-apps/api/core";
import type { DocumentoImportado } from "./contratoDocumentos";

/** Copia os caminhos recebidos pelo evento nativo de soltar arquivos para `documentos/`. */
export const importarDocumentos = (caminhosDosArquivos: string[]) =>
  invoke<DocumentoImportado[]>("importar_documentos", { caminhosDosArquivos });
