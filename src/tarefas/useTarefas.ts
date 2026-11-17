// Estado da tela de tarefas: carrega a lista, cria tarefas e muda o status.
// Usa só o contrato; não sabe como o backend guarda as tarefas.

import { useCallback, useEffect, useState } from "react";
import {
  atualizarTarefaNoCofre,
  criarTarefaNoCofre,
  listarTarefasDoCofre,
  type NovaTarefaParaCriar,
  type StatusDaTarefa,
  type TarefaDoContrato,
} from "./contratoTarefas";

export function useTarefas() {
  const [tarefas, setTarefas] = useState<TarefaDoContrato[]>([]);
  const [estaCarregando, setEstaCarregando] = useState(true);
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);

  /** Busca a lista atual no backend. */
  const recarregarTarefas = useCallback(async () => {
    try {
      setTarefas(await listarTarefasDoCofre());
      setMensagemDeErro(null);
    } catch (erro) {
      setMensagemDeErro(String(erro));
    } finally {
      setEstaCarregando(false);
    }
  }, []);

  useEffect(() => {
    recarregarTarefas();
  }, [recarregarTarefas]);

  /** Cria uma tarefa e atualiza a lista. Lança erro para o formulário mostrar. */
  const criarTarefa = useCallback(
    async (entrada: NovaTarefaParaCriar) => {
      await criarTarefaNoCofre(entrada);
      await recarregarTarefas();
    },
    [recarregarTarefas],
  );

  /** Muda o status de uma tarefa (ex.: marcar como feita) e atualiza a lista. */
  const mudarStatusDaTarefa = useCallback(
    async (idDaTarefa: string, novoStatus: StatusDaTarefa) => {
      try {
        await atualizarTarefaNoCofre(idDaTarefa, { status: novoStatus });
        await recarregarTarefas();
      } catch (erro) {
        setMensagemDeErro(String(erro));
      }
    },
    [recarregarTarefas],
  );

  return { tarefas, estaCarregando, mensagemDeErro, criarTarefa, mudarStatusDaTarefa };
}
