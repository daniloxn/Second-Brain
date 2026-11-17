// Painel lateral com o nome do cofre e a árvore de arquivos inteira.

import type { NoArvore } from "../cofre/chamadasCofre";
import { ArvoreArquivos } from "./ArvoreArquivos";

interface Props {
  nomeDoCofre: string;
  arvoreDoCofre: NoArvore;
  caminhoDaNotaAberta: string | null;
  aoAbrirNota: (caminhoDaNota: string) => void;
}

export function ExploradorDeArquivos({ nomeDoCofre, arvoreDoCofre, caminhoDaNotaAberta, aoAbrirNota }: Props) {
  return (
    <aside className="explorador" aria-label="Arquivos do cofre">
      <div className="cabecalho-painel">
        <span className="rotulo">{nomeDoCofre}</span>
      </div>
      <div className="arvore">
        {arvoreDoCofre.filhos.map((item) => (
          <ArvoreArquivos
            key={item.caminho}
            no={item}
            caminhoDaNotaAberta={caminhoDaNotaAberta}
            aoAbrirNota={aoAbrirNota}
          />
        ))}
      </div>
    </aside>
  );
}
