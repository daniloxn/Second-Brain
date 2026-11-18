// Hook da tela do agente: estado do provedor, histórico, conversa da sessão e ações
// de enviar pedido e desfazer. Guarda a conversa só na memória da tela; o histórico vem do log.

import { useEffect, useRef, useState } from "react";
import { aquecerAgente, definirProvedorDoAgente, desfazerAcaoDoAgente, enviarPedidoAoAgente, listarHistoricoDoAgente, ouvirRespostaParcialDoAgente } from "./chamadasAgente";
import type { EstadoDoProvedor, FalaParaOAgente, RegistroDoAgente } from "./contratoAgente";
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
  const [respostaParcial, setRespostaParcial] = useState("");
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
    const provedorAnterior = window.localStorage.getItem("segundo-cerebro-provedor");
    const provedorSalvo = provedorAnterior === "antigravity" ? "opencode" : provedorAnterior ?? "opencode";
    definirProvedorDoAgente(provedorSalvo)
      .then(async (estado) => {
        if (!aindaMontada) return;
        window.localStorage.setItem("segundo-cerebro-provedor", estado.id);
        setEstadoDoProvedor(estado);
        setHistorico(await listarHistoricoDoAgente());
        await aquecerAgente();
      })
      .catch((erro) => {
        if (aindaMontada) setMensagemDeErro(String(erro));
      });
    return () => {
      aindaMontada = false;
    };
  }, []);

  /** Troca o provedor, guarda a escolha localmente e inicia o aquecimento correspondente. */
  async function trocarProvedor(id: string) {
    setMensagemDeErro(null);
    try {
      const estado = await definirProvedorDoAgente(id);
      window.localStorage.setItem("segundo-cerebro-provedor", id);
      setEstadoDoProvedor(estado);
      await aquecerAgente();
    } catch (erro) {
      setMensagemDeErro(String(erro));
    }
  }

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
    setRespostaParcial("");
    setMensagemDeErro(null);
    // Conversa até aqui, sem os erros: o agente precisa dela para entender "isso" e "sim".
    const conversaAnterior: FalaParaOAgente[] = mensagens.flatMap((mensagem) =>
      mensagem.papel === "erro" ? [] : [{ papel: mensagem.papel, texto: mensagem.texto }],
    );
    acrescentarMensagem("voce", pedido, null);
    const idDaRodada = crypto.randomUUID();
    let pararDeOuvir: (() => void) | null = null;
    try {
      pararDeOuvir = await ouvirRespostaParcialDoAgente((trecho) => {
        if (trecho.idDaRodada === idDaRodada) {
          setRespostaParcial((textoAnterior) => textoAnterior + trecho.texto);
        }
      });
      const registro = await enviarPedidoAoAgente(pedido, conversaAnterior, idDaRodada);
      acrescentarMensagem("agente", registro.resposta, registro);
    } catch (erro) {
      acrescentarMensagem("erro", String(erro), null);
    } finally {
      pararDeOuvir?.();
      setRespostaParcial("");
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
    trocarProvedor,
    aquecimento,
    tentarDeNovoAquecimento,
    historico,
    mensagens,
    enviando,
    respostaParcial,
    mensagemDeErro,
    enviarPedido,
    desfazerPedido,
  };
}
