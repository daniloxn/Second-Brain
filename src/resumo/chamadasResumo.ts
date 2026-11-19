// Chamada ao backend do resumo semanal. Componentes não usam invoke direto.

import { invoke } from "@tauri-apps/api/core";
import type { ResumoDaSemana } from "./contratoResumo";

/** Números dos últimos sete dias, calculados no backend a partir do histórico e das tarefas. */
export const resumoDaSemana = () => invoke<ResumoDaSemana>("resumo_da_semana");
