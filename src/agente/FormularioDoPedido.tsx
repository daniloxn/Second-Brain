// Campo para escrever o pedido ao agente. Enter envia; Shift+Enter quebra a linha.
// Também leva os anexos e a pasta de destino, que entram no texto enviado ao agente.

import { useEffect, useState, type FormEvent, type KeyboardEvent } from "react";
import { Send } from "lucide-react";
import { AnexosDoPedido } from "./anexos/AnexosDoPedido";
import type { ModoDeExecucao } from "./contratoAgente";
import { montarPedidoComAnexos, type AnexoDoPedido } from "./anexos/regrasDosAnexos";

interface Props {
  cofre: string;
  compacto?: boolean;
  /** Desliga o campo enquanto o agente responde ou quando o provedor não está instalado. */
  desativado: boolean;
  aoEnviar: (pedido: string) => void;
  pedidoInicial?: string | null;
  aoConsumirPedidoInicial?: () => void;
  /** Como o próximo pedido roda: executar, só planejar para aprovação, ou consultar a web sem editar. */
  modo: ModoDeExecucao;
  aoMudarModo: (modo: ModoDeExecucao) => void;
}

/** Rótulos do seletor de modo. A ordem é a ordem em que aparecem na lista. */
const ROTULOS_DOS_MODOS: Array<{ valor: ModoDeExecucao; rotulo: string }> = [
  { valor: "executar", rotulo: "Executar" },
  { valor: "planejar", rotulo: "Planejar antes" },
  { valor: "consultar", rotulo: "Consultar na web (sem editar)" },
];

export function FormularioDoPedido({ cofre, desativado, aoEnviar, pedidoInicial, aoConsumirPedidoInicial, modo, aoMudarModo, compacto = false }: Props) {
  const [pedidoDigitado, setPedidoDigitado] = useState(pedidoInicial ?? "");
  const [anexos, setAnexos] = useState<AnexoDoPedido[]>([]);
  const [pastaDeDestino, setPastaDeDestino] = useState<string | null>(null);
  const pedidoPronto = pedidoDigitado.trim() !== "" || anexos.length > 0;

  useEffect(() => {
    if (!pedidoInicial) return;
    setPedidoDigitado(pedidoInicial);
    aoConsumirPedidoInicial?.();
  }, [pedidoInicial, aoConsumirPedidoInicial]);

  /** Junta os anexos aos novos, sem repetir o mesmo arquivo. */
  function acrescentarAnexos(novos: AnexoDoPedido[]) {
    setAnexos((anteriores) => {
      const caminhosJaUsados = new Set(anteriores.map((anexo) => anexo.caminhoRelativo));
      return [...anteriores, ...novos.filter((anexo) => !caminhosJaUsados.has(anexo.caminhoRelativo))];
    });
  }

  /** Envia o texto com os anexos e limpa o campo. A pasta de destino continua até ser removida. */
  function enviarPedidoDigitado() {
    if (!pedidoPronto || desativado) return;
    const pedido = montarPedidoComAnexos(pedidoDigitado, anexos, pastaDeDestino);
    setPedidoDigitado("");
    setAnexos([]);
    aoEnviar(pedido);
  }

  function aoEnviarFormulario(evento: FormEvent) {
    evento.preventDefault();
    enviarPedidoDigitado();
  }

  function aoPressionarTecla(evento: KeyboardEvent<HTMLTextAreaElement>) {
    if (evento.key === "Enter" && !evento.shiftKey && !evento.nativeEvent.isComposing) {
      evento.preventDefault();
      enviarPedidoDigitado();
    }
  }

  return (
    <>
      <label className="planejar-antes">
        <span>Modo</span>
        <select value={modo} disabled={desativado} onChange={(evento) => aoMudarModo(evento.target.value as ModoDeExecucao)}>
          {ROTULOS_DOS_MODOS.map((item) => <option key={item.valor} value={item.valor}>{item.rotulo}</option>)}
        </select>
      </label>
      <AnexosDoPedido
        cofre={cofre}
        anexos={anexos}
        pastaDeDestino={pastaDeDestino}
        desativado={desativado}
        aoAnexar={acrescentarAnexos}
        aoRemoverAnexo={(caminho) => setAnexos((anteriores) => anteriores.filter((anexo) => anexo.caminhoRelativo !== caminho))}
        aoDefinirPastaDeDestino={setPastaDeDestino}
      />
      <form className="formulario-pedido" onSubmit={aoEnviarFormulario}>
        <textarea
          className="campo"
          aria-label="Pedido para o agente"
          placeholder={compacto ? "Escreva uma mensagem…" : "Ex.: me lembre de entregar o relatório amanhã às 14h, prioridade alta"}
          rows={compacto ? 2 : 3}
          value={pedidoDigitado}
          disabled={desativado}
          onChange={(evento) => setPedidoDigitado(evento.target.value)}
          onKeyDown={aoPressionarTecla}
        />
        <button type="submit" aria-label="Enviar mensagem" className="botao botao-primario" disabled={desativado || !pedidoPronto}>
          <Send size={15} aria-hidden="true" />
          {!compacto && <span className="texto-botao">Enviar</span>}
        </button>
      </form>
    </>
  );
}
