// Tipos dos projetos e dos documentos de contexto que o backend leu do cofre.

/** Resumo de um projeto e os três documentos Markdown que o descrevem. */
export interface ProjetoDoCofre {
  nome: string;
  caminhoRelativo: string;
  projeto: string;
  mapa: string;
  log: string;
}
