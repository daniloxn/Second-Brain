// Chamadas ao backend dos atalhos da tela de configurações. Componentes não usam invoke direto.

import { invoke } from "@tauri-apps/api/core";
import type { AcaoDeAtalho, AtalhosDoApp } from "./contratoConfiguracoes";

/** Combinações atuais dos atalhos globais. */
export const atalhosDoApp = () => invoke<AtalhosDoApp>("atalhos_do_app");

/** Troca a combinação de uma ação. Devolve as combinações já atualizadas. Erro se a combinação for recusada. */
export const definirAtalho = (acao: AcaoDeAtalho, combinacao: string) =>
  invoke<AtalhosDoApp>("definir_atalho", { acao, combinacao });
