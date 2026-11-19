// A conversa vive na ilha. Esta tela mantém o acesso ao histórico do cofre.
import { useCallback, useEffect, useState } from "react";
import { ArrowUpRight, MessageCircle } from "lucide-react";
import { abrirChatNaIlha } from "../ilha/chamadasIlha";
import { listarHistoricoDoAgente } from "./chamadasAgente";
import { HistoricoDoAgente } from "./HistoricoDoAgente";
import type { RegistroDoAgente } from "./contratoAgente";

interface Props { pedidoInicial?: string | null; aoConsumirPedidoInicial?: () => void; }

export function TelaDoAgente({ pedidoInicial, aoConsumirPedidoInicial }: Props) {
  const [historico, setHistorico] = useState<RegistroDoAgente[]>([]);
  const [erro, setErro] = useState<string | null>(null);
  const atualizar = useCallback(() => {
    void listarHistoricoDoAgente().then(setHistorico).catch(e => setErro(String(e)));
  }, []);
  useEffect(() => {
    atualizar();
    window.addEventListener("focus", atualizar);
    return () => window.removeEventListener("focus", atualizar);
  }, [atualizar]);

  async function abrirConversa() {
    try { await abrirChatNaIlha(pedidoInicial ?? null); aoConsumirPedidoInicial?.(); setErro(null); }
    catch (e) { setErro(String(e)); }
  }

  return <div className="agente-acesso-ilha">
    <section className="agente-convite-ilha">
      <MessageCircle size={28} aria-hidden="true" />
      <h2>Seu chat está no topo da tela</h2>
      <p>Converse sem sair do que está fazendo. Notas, tarefas e ideias continuam conectadas ao seu cofre.</p>
      <button className="botao botao-primario" onClick={() => void abrirConversa()}>{pedidoInicial ? "Levar documentos para o chat" : "Abrir conversa"}<ArrowUpRight size={16} /></button>
      {erro && <p className="erro" role="alert">{erro}</p>}
    </section>
    <section aria-label="Histórico do agente"><h2>Ações no cofre</h2><p className="suave">Para desfazer uma ação, abra Opções na ilha.</p>
      <HistoricoDoAgente registros={historico} bloqueado aoDesfazer={() => {}} />
    </section>
  </div>;
}
