// Conversa da sessão: pedidos do usuário, respostas do agente e erros. Cada resposta
// mostra quantas mudanças o agente fez no cofre. Enquanto o agente trabalha, lista os passos
// (ferramentas usadas). Uma resposta do modo planejar traz o botão para aprovar o plano.

import type { MensagemDaConversa } from "./useAgente";
import { resumoDasMudancas } from "./formatacaoAgente";

const ROTULO_DA_FALA: Record<MensagemDaConversa["papel"], string> = {
  voce: "Você",
  agente: "Agente",
  erro: "Erro",
};

interface Props {
  mensagens: MensagemDaConversa[];
  enviando: boolean;
  respostaParcial: string;
  passos: string[];
  aoAprovarPlano: (plano: string) => void;
}

export function ConversaDoAgente({ mensagens, enviando, respostaParcial, passos, aoAprovarPlano }: Props) {
  if (mensagens.length === 0 && !enviando) {
    return (
      <p className="suave">
        Escreva o que quer em linguagem natural. O agente cria tarefas, notas e pastas no seu cofre, e cada
        mudança aparece no histórico ao lado, com opção de desfazer.
      </p>
    );
  }

  // Só a última resposta de planejamento oferece aprovação: aprovar uma resposta antiga não faz sentido.
  const ultima = mensagens[mensagens.length - 1];
  const planoParaAprovar = !enviando && ultima?.papel === "agente" && ultima.registro?.modo === "planejar" ? ultima.texto : null;

  return (
    <ol className="conversa-do-agente" aria-live="polite">
      {mensagens.map((mensagem) => (
        <li key={mensagem.id} className={`fala fala-${mensagem.papel}`}>
          <span className="rotulo-da-fala">{ROTULO_DA_FALA[mensagem.papel]}</span>
          <p>{mensagem.texto}</p>
          {mensagem.registro && <span className="suave">{resumoDasMudancas(mensagem.registro.mudancas)}</span>}
        </li>
      ))}
      {enviando && respostaParcial && (
        <li className="fala fala-agente fala-em-transmissao">
          <span className="rotulo-da-fala">Agente</span>
          <p>{respostaParcial}<span className="cursor-de-transmissao" aria-hidden="true" /></p>
        </li>
      )}
      {enviando && !respostaParcial && (
        <li className="fala fala-agente">
          <span className="rotulo-da-fala">Agente</span>
          <p className="suave">Pensando…</p>
        </li>
      )}
      {enviando && passos.length > 0 && (
        <li className="passos-do-agente" aria-label="Passos do agente">
          <ul>
            {passos.map((passo, indice) => <li key={indice}>{passo}</li>)}
          </ul>
        </li>
      )}
      {planoParaAprovar && (
        <li className="aprovacao-do-plano">
          <button className="botao botao-primario" onClick={() => aoAprovarPlano(planoParaAprovar)}>
            Aprovar plano e executar
          </button>
        </li>
      )}
    </ol>
  );
}
