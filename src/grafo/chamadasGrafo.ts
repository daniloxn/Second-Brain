// Chamadas ao backend da funcionalidade grafo. Componentes passam por aqui, não usam invoke direto.

import { invoke } from "@tauri-apps/api/core";
import type { GrafoDoCofre } from "./contratoGrafo";

/** Notas e tarefas do cofre ativo, com ligações e posições 3D. Pode levar um instante em cofres grandes. */
export const buscarGrafoDoCofre = () => invoke<GrafoDoCofre>("grafo_do_cofre");
