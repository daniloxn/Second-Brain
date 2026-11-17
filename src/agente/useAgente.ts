// Hook da tela do agente: estado do provedor, histórico, conversa da sessão e ações
// de enviar pedido e desfazer. Guarda a conversa só na memória da tela; o histórico vem do log.

import { useEffect, useRef, useState } from "react";
import { desfazerAcaoDoAgente, enviarPedidoAoAgente, estadoDoAgente, listarHistoricoDoAgente } from "./chamadasAgente";
import type { EstadoDoProvedor, RegistroDoAgente } from "./contratoAgente";
import { useAquecimentoDoAgente } from "./useAquecimentoDoAgente";

/** Uma fala da conversa: o que o usuário pediu, a resposta do agente ou um erro. */
export interface MensagemDaConversa {
  id: number;
  papel: "voce" | "agente" | "erro";
  texto: string;
  /** Registro do pedido, quando o agente respondeu (traz as mudanças). */
  registro: RegistroDoAgente | null;
}

export function useAgente() {
  const [estadoDoProvedor, setEstadoDoProvedor] = useState<EstadoDoProvedor | null>(null);
  const [historico, setHistorico] = useState<RegistroDoAgente[]>([]);
  const [mensagens, setMensagens] = useState<MensagemDaConversa[]>([]);
  const [enviando, setEnviando] = useState(false);
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);
  const { aquecimento, tentarDeNovo: tentarDeNovoAquecimento } = useAquecimentoDoAgente();
  // Garante que a saudação entre uma vez por visita à tela, mesmo que o efeito rode de novo.
  const saudacaoJaMostrada = useRef(false);

  // Quando o aquecimento responde, a resposta vira a primeira fala da conversa.
  useEffect(() => {
    if (aquecimento?.situacao !== "pronto" || !aquecimento.mensagem || saudacaoJaMostrada.current) return;
    saudacaoJaMostrada.current = true;
    const textoDaSaudacao = aquecimento.mensagem;
    setMensagens((anteriores) => [
      { id: 0, papel: "agente", texto: textoDaSaudacao, registro: null },
      ...anteriores,
    ]);
  }, [aquecimento]);

  // Carrega o provedor e o histórico quando a tela abre.
  useEffect(() => {
    let aindaMontada = true;
    Promise.all([estadoDoAgente(), listarHistoricoDoAgente()])
      .then(([estado, historicoLido]) => {
        if (!aindaMontada) return;
        setEstadoDoProvedor(estado);
        setHistorico(historicoLido);
      })
      .catch((erro) => {
        if (aindaMontada) setMensagemDeErro(String(erro));
      });
    return () => {
      aindaMontada = false;
    };
  }, []);

  /** Relê o histórico do log. Usado depois de cada pedido e de cada desfazer. */
  async function recarregarHistorico() {
    try {
      setHistorico(await listarHistoricoDoAgente());
    } catch (erro) {
      setMensagemDeErro(String(erro));
    }
  }

  /** Acrescenta uma fala à conversa, com id crescente. */
  function acrescentarMensagem(papel: MensagemDaConversa["papel"], texto: string, registro: RegistroDoAgente | null) {
    setMensagens((anteriores) => [...anteriores, { id: anteriores.length + 1, papel, texto, registro }]);
  }

  /** Envia o pedido, mostra a resposta e atualiza o histórico, mesmo quando o agente falha. */
  async function enviarPedido(pedido: string) {
    setEnviando(true);
    setMensagemDeErro(null);
    acrescentarMensagem("voce", pedido, null);
    try {
      const registro = await enviarPedidoAoAgente(pedido);
      acrescentarMensagem("agente", registro.resposta, registro);
    } catch (erro) {
      acrescentarMensagem("erro", String(erro), null);
    } finally {
      setEnviando(false);
      await recarregarHistorico();
    }
  }

  /** Desfaz um pedido do histórico. Se o cofre mudou depois, o backend recusa e a mensagem explica. */
  async function desfazerPedido(id: string) {
    setMensagemDeErro(null);
    try {
      await desfazerAcaoDoAgente(id);
    } catch (erro) {
      setMensagemDeErro(String(erro));
    }
    await recarregarHistorico();
  }

  return {
    estadoDoProvedor,
    aquecimento,
    tentarDeNovoAquecimento,
    historico,
    mensagens,
    enviando,
    mensagemDeErro,
    enviarPedido,
    desfazerPedido,
  };
}
