// Conversa da sessão: pedidos do usuário, respostas do agente e erros. Cada resposta
// mostra quantas mudanças o agente fez no cofre.

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
}

export function ConversaDoAgente({ mensagens, enviando, respostaParcial }: Props) {
  if (mensagens.length === 0 && !enviando) {
    return (
      <p className="suave">
        Escreva o que quer em linguagem natural. O agente cria tarefas, notas e pastas no seu cofre, e cada
        mudança aparece no histórico ao lado, com opção de desfazer.
      </p>
    );
  }

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
    </ol>
  );
}
