// Linha de um segredo desbloqueado. Exibe metadados e delega cópia ou exclusão.

import { Copy, Trash2 } from "lucide-react";
import type { ResumoDeSegredo } from "./contratoSegredos";

interface Props {
  segredo: ResumoDeSegredo;
  aoCopiar: (segredo: ResumoDeSegredo) => void;
  aoApagar: (segredo: ResumoDeSegredo) => void;
}

/** Mostra os detalhes sem revelar a senha na interface. */
export function ItemDeSegredo({ segredo, aoCopiar, aoApagar }: Props) {
  return (
    <article className="item-segredo">
      <div className="corpo-segredo">
        <h3>{segredo.titulo}</h3>
        {segredo.usuario && <p className="suave">{segredo.usuario}</p>}
        {segredo.anotacoes && <p className="anotacoes-segredo">{segredo.anotacoes}</p>}
      </div>
      <div className="acoes-segredo">
        <button type="button" className="botao" onClick={() => aoCopiar(segredo)}>
          <Copy size={15} aria-hidden="true" /> Copiar senha
        </button>
        <button type="button" className="botao botao-apagar-segredo" aria-label={`Apagar ${segredo.titulo}`} title="Apagar segredo" onClick={() => aoApagar(segredo)}>
          <Trash2 size={15} aria-hidden="true" />
        </button>
      </div>
    </article>
  );
}
