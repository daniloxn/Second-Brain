// Histórico das ações do agente: cada pedido com as mudanças no cofre, antes e depois,
// e o botão para desfazer. Desfazer pede confirmação, porque pode apagar notas e pastas.

import { Undo2 } from "lucide-react";
import type { MudancaNoCofre, RegistroDoAgente } from "./contratoAgente";
import {
  formatarQuandoDoRegistro,
  podeDesfazerRegistro,
  resumoDasMudancas,
  rotuloDoTipoDeMudanca,
  textoOuTracoVazio,
} from "./formatacaoAgente";

interface Props {
  registros: RegistroDoAgente[];
  /** Enquanto o agente responde, o desfazer fica desligado. */
  bloqueado: boolean;
  aoDesfazer: (id: string) => void;
}

const MENSAGEM_DE_CONFIRMACAO_DO_DESFAZER =
  "Desfazer esta ação? Notas e pastas que ela criou serão apagadas, e as notas que ela mudou voltam ao texto anterior.";

export function HistoricoDoAgente({ registros, bloqueado, aoDesfazer }: Props) {
  if (registros.length === 0) {
    return <p className="suave">Nenhuma ação ainda. Cada mudança que o agente fizer no cofre aparece aqui.</p>;
  }

  return (
    <ul className="lista-historico">
      {registros.map((registro) => (
        <li key={registro.id} className="item-historico">
          <div className="cabecalho-historico">
            <span className="suave">
              {formatarQuandoDoRegistro(registro.quando)} · {registro.provedor}
            </span>
            {registro.desfeito && <span className="marca-desfeito">Desfeito</span>}
          </div>
          <p className="pedido-historico">{registro.pedido}</p>
          <span className="suave">{resumoDasMudancas(registro.mudancas)}</span>

          {registro.mudancas.length > 0 && (
            <details>
              <summary>Ver mudanças</summary>
              <ul className="lista-de-mudancas">
                {registro.mudancas.map((mudanca) => (
                  <MudancaDoHistorico key={`${mudanca.tipo}-${mudanca.caminho}`} mudanca={mudanca} />
                ))}
              </ul>
            </details>
          )}

          {podeDesfazerRegistro(registro) && (
            <button
              type="button"
              className="botao"
              disabled={bloqueado}
              onClick={() => {
                if (window.confirm(MENSAGEM_DE_CONFIRMACAO_DO_DESFAZER)) aoDesfazer(registro.id);
              }}
            >
              <Undo2 size={14} aria-hidden="true" />
              <span className="texto-botao">Desfazer</span>
            </button>
          )}
        </li>
      ))}
    </ul>
  );
}

interface PropsDaMudanca {
  mudanca: MudancaNoCofre;
}

/** Uma mudança do pedido. Notas mostram o texto de antes e de depois; pastas não têm texto. */
function MudancaDoHistorico({ mudanca }: PropsDaMudanca) {
  const temTexto = mudanca.antes !== null || mudanca.depois !== null;
  return (
    <li className="mudanca">
      <strong>{rotuloDoTipoDeMudanca(mudanca.tipo)}</strong> <code>{mudanca.caminho}</code>
      {temTexto && (
        <details>
          <summary>Antes e depois</summary>
          <h4 className="titulo-do-texto">Antes</h4>
          <pre>{textoOuTracoVazio(mudanca.antes)}</pre>
          <h4 className="titulo-do-texto">Depois</h4>
          <pre>{textoOuTracoVazio(mudanca.depois)}</pre>
        </details>
      )}
    </li>
  );
}
