// Barra do topo da área principal: título da seção, caminho do cofre e botões gerais.

import { RefreshCw, FolderOpen, FolderTree } from "lucide-react";

interface Props {
  tituloDaSecao: string;
  caminhoDoCofre: string;
  aoAtualizar: () => void;
  aoTrocarCofre: () => void;
  arvoreVisivel: boolean;
  aoAlternarArvore: () => void;
}

export function CabecalhoDaSecao({ tituloDaSecao, caminhoDoCofre, aoAtualizar, aoTrocarCofre, arvoreVisivel, aoAlternarArvore }: Props) {
  return (
    <header className="topo">
      <div className="topo-texto">
        <h1 className="titulo-secao">{tituloDaSecao}</h1>
        <span className="caminho" title={caminhoDoCofre}>{caminhoDoCofre}</span>
      </div>
      <div className="acoes">
        <button
          type="button"
          className="botao"
          aria-label={arvoreVisivel ? "Ocultar árvore de pastas" : "Mostrar árvore de pastas"}
          aria-pressed={arvoreVisivel}
          title={arvoreVisivel ? "Ocultar árvore de pastas" : "Mostrar árvore de pastas"}
          onClick={aoAlternarArvore}
        >
          <FolderTree size={15} aria-hidden="true" />
          <span className="texto-botao">Pastas</span>
        </button>
        <button type="button" className="botao" aria-label="Atualizar" onClick={aoAtualizar}>
          <RefreshCw size={15} aria-hidden="true" />
          <span className="texto-botao">Atualizar</span>
        </button>
        <button type="button" className="botao" aria-label="Trocar cofre" onClick={aoTrocarCofre}>
          <FolderOpen size={15} aria-hidden="true" />
          <span className="texto-botao">Trocar cofre</span>
        </button>
      </div>
    </header>
  );
}
