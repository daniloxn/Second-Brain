// Uma linha da lista de tarefas: caixa para concluir, título, prioridade, prazo e tags.

import { Circle, CircleCheck } from "lucide-react";
import type { StatusDaTarefa, TarefaDoContrato } from "./contratoTarefas";
import { formatarPrazo, rotuloDaPrioridade } from "./formatacaoTarefas";

interface Props {
  tarefa: TarefaDoContrato;
  aoMudarStatus: (idDaTarefa: string, novoStatus: StatusDaTarefa) => void;
}

export function ItemDeTarefa({ tarefa, aoMudarStatus }: Props) {
  const estaConcluida = tarefa.status === "feita" || tarefa.status === "arquivada";
  const textoDoPrazo = formatarPrazo(tarefa.prazo);

  return (
    <li className={`item-tarefa item-tarefa-${tarefa.situacao}`}>
      <button
        type="button"
        className="caixa-tarefa"
        role="checkbox"
        aria-checked={estaConcluida}
        aria-label={estaConcluida ? `Reabrir ${tarefa.titulo}` : `Marcar ${tarefa.titulo} como feita`}
        onClick={() => aoMudarStatus(tarefa.id, estaConcluida ? "aberta" : "feita")}
      >
        {estaConcluida ? <CircleCheck size={18} aria-hidden="true" /> : <Circle size={18} aria-hidden="true" />}
      </button>

      <div className="corpo-tarefa">
        <span className="titulo-tarefa">{tarefa.titulo}</span>
        <div className="detalhes-tarefa">
          <span className={`etiqueta-prioridade prioridade-${tarefa.prioridade}`}>
            {rotuloDaPrioridade(tarefa.prioridade)}
          </span>
          {textoDoPrazo && <span className="prazo-tarefa">{textoDoPrazo}</span>}
          {tarefa.tags.map((tag) => (
            <span key={tag} className="tag-tarefa">#{tag}</span>
          ))}
        </div>
      </div>
    </li>
  );
}
