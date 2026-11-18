// Estado da tela de tarefas: carrega a lista, cria tarefas e muda o status.
// Usa só o contrato; não sabe como o backend guarda as tarefas.

import { useCallback, useEffect, useState } from "react";
import {
  atualizarTarefaNoCofre,
  apagarTarefaDoCofre,
  criarTarefaNoCofre,
  listarTarefasDoCofre,
  type NovaTarefaParaCriar,
  type AlteracaoDaTarefa,
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

  /** Salva as alterações de uma tarefa e recarrega a lista. */
  const editarTarefa = useCallback(
    async (idDaTarefa: string, alteracao: AlteracaoDaTarefa) => {
      await atualizarTarefaNoCofre(idDaTarefa, alteracao);
      await recarregarTarefas();
    },
    [recarregarTarefas],
  );

  /** Apaga uma tarefa e atualiza a lista, mostrando erros no painel. */
  const apagarTarefa = useCallback(
    async (idDaTarefa: string) => {
      try {
        await apagarTarefaDoCofre(idDaTarefa);
        await recarregarTarefas();
        return true;
      } catch (erro) {
        setMensagemDeErro(String(erro));
        return false;
      }
    },
    [recarregarTarefas],
  );

  return { tarefas, estaCarregando, mensagemDeErro, criarTarefa, editarTarefa, apagarTarefa, mudarStatusDaTarefa };
}
