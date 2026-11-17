// Barra do topo da área principal: título da seção, caminho do cofre e botões gerais.

import { RefreshCw, FolderOpen } from "lucide-react";

interface Props {
  tituloDaSecao: string;
  caminhoDoCofre: string;
  aoAtualizar: () => void;
  aoTrocarCofre: () => void;
}

export function CabecalhoDaSecao({ tituloDaSecao, caminhoDoCofre, aoAtualizar, aoTrocarCofre }: Props) {
  return (
    <header className="topo">
      <div className="topo-texto">
        <h1 className="titulo-secao">{tituloDaSecao}</h1>
        <span className="caminho" title={caminhoDoCofre}>{caminhoDoCofre}</span>
      </div>
      <div className="acoes">
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
