// Situação do aquecimento do agente para a tela: lê o estado guardado no backend ao montar
// e acompanha o evento de mudança. Assim, se a saudação já terminou, a tela mostra logo.

import { useEffect, useState } from "react";
import { aquecerAgente, estadoDoAquecimento, ouvirAquecimentoDoAgente } from "./chamadasAgente";
import type { AquecimentoDoAgente } from "./contratoAgente";

export function useAquecimentoDoAgente() {
  // Nulo até a primeira leitura. A tela trata nulo como "preparando".
  const [aquecimento, setAquecimento] = useState<AquecimentoDoAgente | null>(null);

  useEffect(() => {
    let aindaMontada = true;
    let cancelarEscuta: (() => void) | null = null;

    ouvirAquecimentoDoAgente((novoAquecimento) => {
      if (aindaMontada) setAquecimento(novoAquecimento);
    }).then((cancelar) => {
      // Se a tela fechou antes da escuta começar, cancela na hora.
      if (aindaMontada) cancelarEscuta = cancelar;
      else cancelar();
    });

    estadoDoAquecimento()
      .then((estadoAtual) => {
        if (aindaMontada) setAquecimento(estadoAtual);
      })
      .catch(() => {
        // Sem leitura, a tela continua em "preparando" até chegar o evento.
      });

    return () => {
      aindaMontada = false;
      cancelarEscuta?.();
    };
  }, []);

  return { aquecimento, tentarDeNovo: aquecerAgente };
}
