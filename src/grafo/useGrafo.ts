// Estado da tela do grafo: carrega o grafo do backend, guarda os filtros e calcula a visão.
// Usa só o contrato e as regras de filtro; não sabe desenhar nada.

import { useCallback, useEffect, useMemo, useState } from "react";
import { buscarGrafoDoCofre } from "./chamadasGrafo";
import type { GrafoDoCofre } from "./contratoGrafo";
import { FILTROS_INICIAIS_DO_GRAFO, montarVisaoDoGrafo, type FiltrosDoGrafo } from "./filtrosDoGrafo";

export function useGrafo() {
  const [grafo, setGrafo] = useState<GrafoDoCofre | null>(null);
  const [estaCarregando, setEstaCarregando] = useState(true);
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const [filtros, setFiltros] = useState<FiltrosDoGrafo>(FILTROS_INICIAIS_DO_GRAFO);

  useEffect(() => {
    let aindaMontada = true;
    buscarGrafoDoCofre()
      .then((grafoRecebido) => {
        if (aindaMontada) setGrafo(grafoRecebido);
      })
      .catch((erro) => {
        if (aindaMontada) setMensagemDeErro(String(erro));
      })
      .finally(() => {
        if (aindaMontada) setEstaCarregando(false);
      });
    return () => {
      aindaMontada = false;
    };
  }, []);

  /** Troca só os filtros indicados e mantém os outros. */
  const mudarFiltros = useCallback((mudanca: Partial<FiltrosDoGrafo>) => {
    setFiltros((filtrosAnteriores) => ({ ...filtrosAnteriores, ...mudanca }));
  }, []);

  const visao = useMemo(() => (grafo ? montarVisaoDoGrafo(grafo, filtros) : null), [grafo, filtros]);

  return { grafo, visao, filtros, mudarFiltros, estaCarregando, mensagemDeErro };
}
