// Tela da seção Tarefas: lista agrupada por situação, com o formulário de nova tarefa.
// Os grupos e a ordem vêm prontos do backend; aqui só se organiza a exibição.

import { useState } from "react";
import { Plus } from "lucide-react";
import { useTarefas } from "./useTarefas";
import { agruparPorSituacao } from "./formatacaoTarefas";
import { ItemDeTarefa } from "./ItemDeTarefa";
import { FormularioNovaTarefa } from "./FormularioNovaTarefa";
import type { NovaTarefaParaCriar } from "./contratoTarefas";

export function TelaDeTarefas() {
  const { tarefas, estaCarregando, mensagemDeErro, criarTarefa, mudarStatusDaTarefa } = useTarefas();
  const [estaCriandoTarefa, setEstaCriandoTarefa] = useState(false);

  /** Cria a tarefa e fecha o formulário. Erros sobem para o formulário. */
  async function criarEFechar(entrada: NovaTarefaParaCriar) {
    await criarTarefa(entrada);
    setEstaCriandoTarefa(false);
  }

  const gruposDeTarefas = agruparPorSituacao(tarefas);

  return (
    <div className="tela-tarefas">
      <div className="linha-topo">
        <h2 className="rotulo">{tarefas.length === 1 ? "1 tarefa" : `${tarefas.length} tarefas`}</h2>
        {!estaCriandoTarefa && (
          <button type="button" className="botao botao-primario" onClick={() => setEstaCriandoTarefa(true)}>
            <Plus size={15} aria-hidden="true" />
            Nova tarefa
          </button>
        )}
      </div>

      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}

      {estaCriandoTarefa && (
        <FormularioNovaTarefa aoEnviar={criarEFechar} aoCancelar={() => setEstaCriandoTarefa(false)} />
      )}

      {estaCarregando && <p className="suave">Carregando tarefas…</p>}

      {!estaCarregando && tarefas.length === 0 && (
        <p className="suave">Nenhuma tarefa ainda. Crie a primeira ou peça para o agente registrar uma.</p>
      )}

      {gruposDeTarefas.map((grupo) => (
        <section key={grupo.situacao} className="grupo-tarefas" aria-label={grupo.rotulo}>
          <h3 className="rotulo">{grupo.rotulo} · {grupo.tarefas.length}</h3>
          <ul className="lista-tarefas">
            {grupo.tarefas.map((tarefa) => (
              <ItemDeTarefa key={tarefa.id} tarefa={tarefa} aoMudarStatus={mudarStatusDaTarefa} />
            ))}
          </ul>
        </section>
      ))}
    </div>
  );
}
