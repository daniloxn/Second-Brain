// Tela do agente: conversa com o provedor à esquerda e o histórico das ações à direita.
// A lógica fica no hook useAgente; aqui só se monta o layout e a linha de situação.

import { ConversaDoAgente } from "./ConversaDoAgente";
import { FormularioDoPedido } from "./FormularioDoPedido";
import { HistoricoDoAgente } from "./HistoricoDoAgente";
import { textoDaSituacaoDoAquecimento } from "./formatacaoAgente";
import { useAgente } from "./useAgente";

export function TelaDoAgente() {
  const {
    estadoDoProvedor,
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
          <FormularioDoPedido desativado={enviando || !provedorPronto} aoEnviar={enviarPedido} />
        </section>

        <aside className="coluna-historico" aria-label="Histórico das ações do agente">
          <h2 className="rotulo">Ações no cofre</h2>
          <HistoricoDoAgente registros={historico} bloqueado={enviando} aoDesfazer={desfazerPedido} />
        </aside>
      </div>
    </div>
  );
}
