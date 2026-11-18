// Chamadas da funcionalidade projetos. Os componentes não acessam invoke diretamente.

import { invoke } from "@tauri-apps/api/core";
import type { ProjetoDoCofre } from "./contratoProjetos";

/** Lista projetos e lê PROJETO.md, MAPA.md e LOG.md de cada pasta. */
export const listarProjetos = () => invoke<ProjetoDoCofre[]>("listar_projetos");

/** Cria um projeto com os documentos iniciais de contexto do agente. */
export const criarProjeto = (nome: string) => invoke<ProjetoDoCofre>("criar_projeto", { nome });
