// Campo para escrever o pedido ao agente. Enter envia; Shift+Enter quebra a linha.

import { useState, type FormEvent, type KeyboardEvent } from "react";
import { Send } from "lucide-react";

interface Props {
  /** Desliga o campo enquanto o agente responde ou quando o provedor não está instalado. */
  desativado: boolean;
  aoEnviar: (pedido: string) => void;
}

export function FormularioDoPedido({ desativado, aoEnviar }: Props) {
  const [pedidoDigitado, setPedidoDigitado] = useState("");
  const pedidoPronto = pedidoDigitado.trim() !== "";

  /** Envia o texto digitado e limpa o campo. Ignora pedido vazio ou com o campo desligado. */
  function enviarPedidoDigitado() {
    const pedido = pedidoDigitado.trim();
    if (!pedido || desativado) return;
    setPedidoDigitado("");
    aoEnviar(pedido);
  }

  function aoEnviarFormulario(evento: FormEvent) {
    evento.preventDefault();
    enviarPedidoDigitado();
  }

  function aoPressionarTecla(evento: KeyboardEvent<HTMLTextAreaElement>) {
    if (evento.key === "Enter" && !evento.shiftKey) {
      evento.preventDefault();
      enviarPedidoDigitado();
    }
  }

  return (
    <form className="formulario-pedido" onSubmit={aoEnviarFormulario}>
      <textarea
        className="campo"
        aria-label="Pedido para o agente"
        placeholder="Ex.: me lembre de entregar o relatório amanhã às 14h, prioridade alta"
        rows={3}
        value={pedidoDigitado}
        disabled={desativado}
        onChange={(evento) => setPedidoDigitado(evento.target.value)}
        onKeyDown={aoPressionarTecla}
      />
      <button type="submit" className="botao botao-primario" disabled={desativado || !pedidoPronto}>
        <Send size={15} aria-hidden="true" />
        <span className="texto-botao">Enviar</span>
      </button>
    </form>
  );
}
