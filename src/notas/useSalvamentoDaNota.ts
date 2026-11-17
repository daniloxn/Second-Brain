// Hook do salvamento de uma nota: carrega o texto, salva sozinho 700 ms depois da última
// mudança e ao sair, e detecta quando a nota mudou no disco por fora (ex.: um agente editou).
// Nesse caso não sobrescreve: a tela deixa escolher entre a versão do disco e a sua.

import { useEffect, useRef, useState } from "react";
import { guardarCopiaDoConflito, lerNota, salvarNota } from "./chamadasNotas";

/** Estado do salvamento, mostrado no topo da nota. */
export type EstadoDoSalvamento = "carregando" | "salvo" | "editando" | "salvando" | "conflito" | "erro";

/** Tempo sem digitar até o salvamento automático disparar. */
const ATRASO_DO_SALVAMENTO_EM_MS = 700;

export function useSalvamentoDaNota(caminhoDaNota: string) {
  // Texto que a tela mostra. Muda a cada tecla, então a leitura também atualiza.
  const [textoDaTela, setTextoDaTela] = useState<string | null>(null);
  const [estadoDoSalvamento, setEstadoDoSalvamento] = useState<EstadoDoSalvamento>("carregando");
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  // Sobe quando o texto é trocado pela versão do disco. O editor usa como `key` para remontar.
  const [versaoDoTexto, setVersaoDoTexto] = useState(0);

  // Refs guardam o texto e as pendências sem disparar renderização a cada tecla.
  const textoNaMemoria = useRef("");
  // Versão do arquivo que a tela conhece. O salvamento só grava se o disco ainda estiver nela.
  const versaoConhecida = useRef(0);
  const existeAlteracaoPendente = useRef(false);
  const temporizadorDoSalvamento = useRef<number | undefined>(undefined);

  // Carrega o texto e a versão do arquivo quando a nota abre.
  useEffect(() => {
    let aindaMontada = true;
    lerNota(caminhoDaNota)
      .then((notaLida) => {
        if (!aindaMontada) return;
        textoNaMemoria.current = notaLida.texto;
        versaoConhecida.current = notaLida.versao;
        setTextoDaTela(notaLida.texto);
        setEstadoDoSalvamento("salvo");
      })
      .catch((erro) => {
        if (!aindaMontada) return;
        setMensagemDeErro(String(erro));
        setEstadoDoSalvamento("erro");
      });
    return () => {
      aindaMontada = false;
    };
  }, [caminhoDaNota]);

  // Ao sair da nota, grava o que ficou pendente. Se o disco mudou, guarda uma cópia do texto
  // em `_sistema/conflitos/` em vez de sobrescrever.
  useEffect(() => {
    return () => {
      window.clearTimeout(temporizadorDoSalvamento.current);
      if (!existeAlteracaoPendente.current) return;
      existeAlteracaoPendente.current = false;
      const textoDeSaida = textoNaMemoria.current;
      salvarNota(caminhoDaNota, textoDeSaida, versaoConhecida.current)
        .then((resultado) => {
          if (!resultado.gravado) return guardarCopiaDoConflito(caminhoDaNota, textoDeSaida);
        })
        .catch(() => {
          // Sem tela para mostrar ao sair. A próxima abertura carrega o que estiver no disco.
        });
    };
  }, [caminhoDaNota]);

  /** Grava o texto agora, se houver alteração pendente. */
  async function salvarAgora() {
    window.clearTimeout(temporizadorDoSalvamento.current);
    if (!existeAlteracaoPendente.current) return;
    existeAlteracaoPendente.current = false;
    setEstadoDoSalvamento("salvando");
    try {
      const resultado = await salvarNota(caminhoDaNota, textoNaMemoria.current, versaoConhecida.current);
      if (!resultado.gravado) {
        // O texto digitado continua pendente. A tela decide se mantém o seu ou usa o do disco.
        existeAlteracaoPendente.current = true;
        setEstadoDoSalvamento("conflito");
        return;
      }
      versaoConhecida.current = resultado.versao;
      // Se o usuário digitou durante o salvamento, o estado continua "editando".
      if (!existeAlteracaoPendente.current) setEstadoDoSalvamento("salvo");
      setMensagemDeErro(null);
    } catch (erro) {
      existeAlteracaoPendente.current = true;
      setMensagemDeErro(String(erro));
      setEstadoDoSalvamento("erro");
    }
  }

  /** Registra uma tecla digitada e agenda o salvamento automático. Um conflito continua visível. */
  function aoMudarTexto(textoAtualizado: string) {
    textoNaMemoria.current = textoAtualizado;
    existeAlteracaoPendente.current = true;
    setTextoDaTela(textoAtualizado);
    setEstadoDoSalvamento((estadoAnterior) => (estadoAnterior === "conflito" ? "conflito" : "editando"));
    window.clearTimeout(temporizadorDoSalvamento.current);
    temporizadorDoSalvamento.current = window.setTimeout(salvarAgora, ATRASO_DO_SALVAMENTO_EM_MS);
  }

  /** Descarta o texto digitado e carrega o do disco. O texto digitado vai antes para uma cópia. */
  async function usarVersaoDoDisco() {
    try {
      await guardarCopiaDoConflito(caminhoDaNota, textoNaMemoria.current);
      const notaLida = await lerNota(caminhoDaNota);
      textoNaMemoria.current = notaLida.texto;
      versaoConhecida.current = notaLida.versao;
      existeAlteracaoPendente.current = false;
      setTextoDaTela(notaLida.texto);
      setVersaoDoTexto((versaoAnterior) => versaoAnterior + 1);
      setEstadoDoSalvamento("salvo");
      setMensagemDeErro(null);
    } catch (erro) {
      setMensagemDeErro(String(erro));
      setEstadoDoSalvamento("erro");
    }
  }

  /** Mantém o texto digitado. A versão do disco vai antes para uma cópia, e então sobrescreve. */
  async function manterMinhaVersao() {
    try {
      const notaNoDisco = await lerNota(caminhoDaNota);
      await guardarCopiaDoConflito(caminhoDaNota, notaNoDisco.texto);
      versaoConhecida.current = notaNoDisco.versao;
      existeAlteracaoPendente.current = true;
      await salvarAgora();
    } catch (erro) {
      setMensagemDeErro(String(erro));
      setEstadoDoSalvamento("erro");
    }
  }

  return {
    textoDaTela,
    estadoDoSalvamento,
    mensagemDeErro,
    versaoDoTexto,
    aoMudarTexto,
    salvarAgora,
    usarVersaoDoDisco,
    manterMinhaVersao,
  };
}
