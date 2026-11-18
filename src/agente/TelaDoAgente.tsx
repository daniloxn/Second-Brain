// Tela do agente: conversa com o provedor à esquerda e o histórico das ações à direita.
// A lógica fica no hook useAgente; aqui só se monta o layout e a linha de situação.

import { ConversaDoAgente } from "./ConversaDoAgente";
import { FormularioDoPedido } from "./FormularioDoPedido";
import { HistoricoDoAgente } from "./HistoricoDoAgente";
import { textoDaSituacaoDoAquecimento } from "./formatacaoAgente";
import { useAgente } from "./useAgente";

interface Props {
  pedidoInicial?: string | null;
  aoConsumirPedidoInicial?: () => void;
}

/** Mostra a conversa e permite receber um pedido inicial já preenchido pela importação. */
export function TelaDoAgente({ pedidoInicial, aoConsumirPedidoInicial }: Props) {
  const {
    estadoDoProvedor,
    trocarProvedor,
    aquecimento,
    tentarDeNovoAquecimento,
    historico,
    mensagens,
    enviando,
    mensagemDeErro,
    enviarPedido,
    desfazerPedido,
  } = useAgente();
  const nomeDoProvedor = estadoDoProvedor?.nome ?? "Agente";
  const provedorPronto = estadoDoProvedor?.instalado === true;
  const textoDaSituacao = estadoDoProvedor && !estadoDoProvedor.instalado
    ? `${nomeDoProvedor}: o comando "${estadoDoProvedor.comando}" não foi encontrado no PATH.`
    : textoDaSituacaoDoAquecimento(aquecimento, nomeDoProvedor);

  return (
    <div className="area-do-agente">
      <div className="tela-agente">
        <section className="coluna-conversa" aria-label="Conversa com o agente">
          <div className="linha-da-situacao">
            <div className="seletor-provedor">
              <label htmlFor="seletor-provedor-agente">Provedor</label>
              <select
                id="seletor-provedor-agente"
                value={estadoDoProvedor?.id ?? "opencode"}
                disabled={enviando}
                onChange={(evento) => void trocarProvedor(evento.target.value)}
              >
                <option value="opencode">OpenCode</option>
                <option value="codex">Codex</option>
                <option value="claude">Claude Code</option>
                <option value="ollama">Ollama local · somente conversa</option>
              </select>
            </div>
            <p className="suave situacao-do-provedor">{textoDaSituacao}</p>
            {aquecimento?.situacao === "falhou" && (
              <button type="button" className="botao" onClick={tentarDeNovoAquecimento}>
                Tentar de novo
              </button>
            )}
          </div>
          {mensagemDeErro && <p role="alert" className="erro faixa">{mensagemDeErro}</p>}
          <div className="area-da-conversa">
            <ConversaDoAgente mensagens={mensagens} enviando={enviando} />
          </div>
          <FormularioDoPedido
            desativado={enviando || !provedorPronto}
            aoEnviar={enviarPedido}
            pedidoInicial={pedidoInicial}
            aoConsumirPedidoInicial={aoConsumirPedidoInicial}
          />
        </section>

        <aside className="coluna-historico" aria-label="Histórico das ações do agente">
          <h2 className="rotulo">Ações no cofre</h2>
          <HistoricoDoAgente registros={historico} bloqueado={enviando} aoDesfazer={desfazerPedido} />
        </aside>
      </div>
    </div>
  );
}
