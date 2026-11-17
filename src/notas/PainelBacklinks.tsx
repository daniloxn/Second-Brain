// Painel lateral com as notas que apontam por wikilink para a nota aberta.

import { useEffect, useState } from "react";
import { backlinksDaNota, type Backlink } from "./chamadasNotas";

interface Props {
  nomeDaNota: string;
  caminhoDaNota: string;
  aoClicarNoBacklink: (caminhoDeOutraNota: string) => void;
}

export function PainelBacklinks({ nomeDaNota, caminhoDaNota, aoClicarNoBacklink }: Props) {
  const [backlinksEncontrados, setBacklinksEncontrados] = useState<Backlink[]>([]);

  // Carrega os links sempre que a nota muda. Ignora respostas de notas que já saíram da tela.
  useEffect(() => {
    let aindaMontado = true;
    backlinksDaNota(caminhoDaNota)
      .then((lista) => {
        if (aindaMontado) setBacklinksEncontrados(lista);
      })
      .catch(() => {
        if (aindaMontado) setBacklinksEncontrados([]);
      });
    return () => {
      aindaMontado = false;
    };
  }, [caminhoDaNota]);

  return (
    <aside className="backlinks" aria-label="Links para esta nota">
      <h2 className="rotulo">Links para esta nota · {backlinksEncontrados.length}</h2>
      {backlinksEncontrados.length === 0 ? (
        <p className="suave pequeno">
          Nenhuma nota aponta para esta ainda. Use [[{nomeDaNota}]] em outra nota para criar a conexão.
        </p>
      ) : (
        <ul className="lista-backlinks">
          {backlinksEncontrados.map((backlink) => (
            <li key={`${backlink.caminho}:${backlink.linha}`}>
              <button type="button" className="cartao-backlink" onClick={() => aoClicarNoBacklink(backlink.caminho)}>
                <strong>{backlink.nome}</strong>
                <span className="suave pequeno">linha {backlink.linha}</span>
                <span className="trecho">{backlink.trecho}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </aside>
  );
}
