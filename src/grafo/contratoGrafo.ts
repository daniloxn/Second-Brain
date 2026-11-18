// Tipos do grafo, espelhados de src-tauri/src/grafo/contrato.rs. Ao mudar um campo lá,
// mude aqui na mesma alteração.

/** Tipo de um ponto do grafo: tarefas ficam em `tarefas/`, o resto é nota. */
export type TipoDoNo = "nota" | "tarefa";

/** Uma nota ou tarefa, com a posição 3D já calculada pelo backend. */
export interface NoDoGrafo {
  /** Caminho completo do arquivo, o mesmo que a tela usa para abrir a nota. */
  caminho: string;
  /** Nome sem extensão. */
  nome: string;
  /** Pasta relativa ao cofre, com `/`. Vazio quando a nota está na raiz. */
  pasta: string;
  tipo: TipoDoNo;
  /** Quantidade de ligações que entram ou saem deste ponto. */
  conexoes: number;
  /** Posição `[x, y, z]` dentro do disco da galáxia. */
  posicao: [number, number, number];
}

/** Uma ligação por wikilink. `origem` e `destino` são posições na lista `nos`. */
export interface LigacaoDoGrafo {
  origem: number;
  destino: number;
}

/** Forma da galáxia onde as notas foram posicionadas: um disco no plano x-z com um núcleo vazio no centro. */
export interface VolumeDaGalaxia {
  /** Raio do disco. */
  raio: number;
  /** Metade da espessura do disco no centro. */
  semiEspessura: number;
  /** Raio do núcleo, onde nenhuma nota entra. */
  raioDoNucleo: number;
}

/** Grafo inteiro do cofre. `truncado` avisa que havia mais notas do que o limite do grafo. */
export interface GrafoDoCofre {
  nos: NoDoGrafo[];
  ligacoes: LigacaoDoGrafo[];
  volume: VolumeDaGalaxia;
  truncado: boolean;
}
