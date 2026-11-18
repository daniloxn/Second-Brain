// Seções do menu lateral. Cada seção diz se já tem tela pronta (fase de entrega null)
// ou se ainda mostra um aviso de que chega numa fase futura.

import type { LucideIcon } from "lucide-react";
import { Bot, Brain, CheckSquare, Folder, FolderGit2, FileText, KeyRound } from "lucide-react";

/** Identificador de cada seção. Usado para decidir o que mostrar na área principal. */
export type IdDaSecao = "arquivos" | "notas" | "tarefas" | "grafo" | "segredos" | "agente" | "projetos";

export interface SecaoDoMenu {
  id: IdDaSecao;
  titulo: string;
  icone: LucideIcon;
  /** Fase em que a seção fica pronta. Null quando ela já tem tela de verdade. */
  faseDeEntrega: string | null;
  /** Frase mostrada na tela de aviso, enquanto a seção não existe. */
  descricaoDoAviso: string;
}

/** Seção aberta quando o app inicia. */
export const SECAO_INICIAL: IdDaSecao = "arquivos";

/** Todas as seções, na ordem do menu lateral. */
export const SECOES_DO_MENU: SecaoDoMenu[] = [
  { id: "arquivos", titulo: "Arquivos", icone: Folder, faseDeEntrega: null, descricaoDoAviso: "" },
  { id: "notas", titulo: "Notas", icone: FileText, faseDeEntrega: null, descricaoDoAviso: "" },
  { id: "tarefas", titulo: "Tarefas", icone: CheckSquare, faseDeEntrega: null, descricaoDoAviso: "" },
  { id: "grafo", titulo: "Grafo", icone: Brain, faseDeEntrega: null, descricaoDoAviso: "" },
  {
    id: "segredos",
    titulo: "Segredos",
    icone: KeyRound,
    faseDeEntrega: "fase 4",
    descricaoDoAviso: "Senhas guardadas criptografadas, fora dos arquivos de texto.",
  },
  { id: "agente", titulo: "Agente", icone: Bot, faseDeEntrega: null, descricaoDoAviso: "" },
  {
    id: "projetos",
    titulo: "Projetos",
    icone: FolderGit2,
    faseDeEntrega: "fase 7",
    descricaoDoAviso: "Estado dos projetos de programação, registrado pelos agentes.",
  },
];

/** Busca a seção pelo id. Se não achar, devolve a primeira do menu. */
export function acharSecaoPeloId(idProcurado: IdDaSecao): SecaoDoMenu {
  return SECOES_DO_MENU.find((secao) => secao.id === idProcurado) ?? SECOES_DO_MENU[0];
}
