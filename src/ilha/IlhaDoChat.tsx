import { useCallback, useEffect, useRef, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { ArrowUpRight, ChevronDown, ChevronUp, MessageCircle, Power, Settings2, X } from "lucide-react";
import { cofreAtual } from "../cofre/chamadasCofre";
import { useAgente } from "../agente/useAgente";
import { ConversaDoAgente } from "../agente/ConversaDoAgente";
import { FormularioDoPedido } from "../agente/FormularioDoPedido";
import { HistoricoDoAgente } from "../agente/HistoricoDoAgente";
import { abrirJanelaPrincipal, dimensionarIlha, encerrarAplicativo } from "./chamadasIlha";

export function IlhaDoChat() {
  const [expandida, setExpandida] = useState(false);
  const [cofre, setCofre] = useState<string | null>(null);
  const [carregando, setCarregando] = useState(true);
  const [erro, setErro] = useState<string | null>(null);
  const [pedido, setPedido] = useState<string | null>(null);
  const [ocupado, setOcupado] = useState(false);
  // Acende quando o agente termina com a ilha recolhida, para a pessoa notar a resposta pronta.
  const [respostaNova, setRespostaNova] = useState(false);
  const expandidaAtual = useRef(false);
  const ocupadoAntes = useRef(false);
  const transicao = useRef(false);
  const botaoAbrir = useRef<HTMLButtonElement>(null);

  // Ignora o primeiro valor: só uma rodada que estava em andamento pode terminar com resposta nova.
  const aoOcuparChat = useCallback((valor: boolean) => {
    if (ocupadoAntes.current && !valor && !expandidaAtual.current) setRespostaNova(true);
    ocupadoAntes.current = valor;
    setOcupado(valor);
  }, []);

  const atualizarCofre = useCallback(() => {
    cofreAtual().then(setCofre).catch(e => setErro(String(e))).finally(() => setCarregando(false));
  }, []);

  const alternar = useCallback(async (abrir: boolean) => {
    if (transicao.current) return;
    transicao.current = true;
    try {
      await dimensionarIlha(abrir);
      setExpandida(abrir);
      setErro(null);
      if (abrir) atualizarCofre();
      else requestAnimationFrame(() => botaoAbrir.current?.focus());
    } catch (e) { setErro(String(e)); }
    finally { transicao.current = false; }
  }, [atualizarCofre]);

  useEffect(() => {
    expandidaAtual.current = expandida;
    if (expandida) setRespostaNova(false);
  }, [expandida]);

  useEffect(() => {
    atualizarCofre();
    void dimensionarIlha(false).catch(e => setErro(String(e)));
    const escutas = [
      listen<string | null>("ilha-abrir", ({ payload }) => { setPedido(payload); void alternar(true); }),
      listen("ilha-recolher", () => setExpandida(false)),
      listen("cofre-alterado", () => atualizarCofre()),
    ];
    return () => { escutas.forEach(p => { void p.then(parar => parar()).catch(() => {}); }); };
  }, [alternar, atualizarCofre]);

  function abrirCofre() {
    void abrirJanelaPrincipal().then(() => alternar(false)).catch(e => setErro(String(e)));
  }

  return (
    <main className={`ilha ${expandida ? "ilha-expandida" : "ilha-recolhida"}`}
      onKeyDown={e => { if (e.key === "Escape") { e.preventDefault(); void alternar(false); } }}>
      <button ref={botaoAbrir} type="button" className="ilha-pilula" hidden={expandida}
        aria-label={ocupado ? "Abrir chat — agente respondendo" : respostaNova ? "Abrir chat — resposta pronta" : "Abrir chat do Segundo Cérebro"}
        aria-expanded={expandida} aria-controls="ilha-painel" onClick={() => void alternar(true)}>
        <MessageCircle size={18} aria-hidden="true" />
        <span>{ocupado ? "Pensando…" : respostaNova ? "Resposta pronta" : "Segundo Cérebro"}</span>
        <span className={`ilha-sinal${ocupado ? " ilha-sinal-ocupado" : respostaNova ? " ilha-sinal-nova" : ""}`} aria-hidden="true" />
        <ChevronDown size={14} aria-hidden="true" />
      </button>
      <div id="ilha-painel" className="ilha-painel" hidden={!expandida}>
        <header className="ilha-cabecalho">
          <MessageCircle size={21} className="ilha-marca" aria-hidden="true" />
          <div className="ilha-titulo"><h1>Segundo Cérebro</h1><span>{ocupado ? "Preparando sua resposta…" : "Uma conversa, tudo no lugar."}</span></div>
          <button className="ilha-icone" onClick={abrirCofre} title="Abrir cofre" aria-label="Abrir cofre"><ArrowUpRight size={19} /></button>
          <button className="ilha-icone" onClick={() => void alternar(false)} title="Recolher · Esc" aria-label="Recolher chat"><ChevronUp size={19} /></button>
        </header>
        {erro && <p role="alert" className="ilha-erro">{erro}</p>}
        {cofre ? <ChatDaIlha key={cofre} cofre={cofre} expandida={expandida} pedido={pedido} consumirPedido={() => setPedido(null)} aoOcupar={aoOcuparChat} /> :
          <div className="ilha-sem-cofre"><MessageCircle size={34} aria-hidden="true" /><h2>{carregando ? "Abrindo seu cofre…" : "Vamos começar?"}</h2>
            <p>Escolha seu cofre para conversar sobre suas notas, tarefas e ideias.</p>
            <button className="botao botao-primario" onClick={abrirCofre}>Escolher cofre <ArrowUpRight size={16} /></button>
          </div>}
        <footer className="ilha-rodape"><span>{cofre ? cofre.split(/[\\/]/).filter(Boolean).pop() : "Seu espaço pessoal"}</span>
          <button className="ilha-sair" onClick={() => void encerrarAplicativo().catch(e => setErro(String(e)))} title="Encerrar aplicativo"><Power size={12} /> Sair</button>
        </footer>
      </div>
    </main>
  );
}

function ChatDaIlha({ cofre, expandida, pedido, consumirPedido, aoOcupar }: {
  cofre: string; expandida: boolean; pedido: string | null; consumirPedido: () => void; aoOcupar: (valor: boolean) => void;
}) {
  const agente = useAgente();
  const [opcoes, setOpcoes] = useState(false);
  const conversa = useRef<HTMLDivElement>(null);
  const acompanhar = useRef(true);
  const raiz = useRef<HTMLDivElement>(null);
  useEffect(() => { aoOcupar(agente.enviando); }, [agente.enviando, aoOcupar]);
  useEffect(() => {
    if (expandida && !agente.enviando) raiz.current?.querySelector("textarea")?.focus();
  }, [expandida, agente.enviando, agente.estadoDoProvedor?.instalado]);
  useEffect(() => {
    if (acompanhar.current && conversa.current) conversa.current.scrollTop = conversa.current.scrollHeight;
  }, [agente.mensagens, agente.respostaParcial, expandida]);
  const pronto = agente.estadoDoProvedor?.instalado === true;

  return <div className="ilha-chat" ref={raiz}>
    <div className="ilha-contexto"><span><span className={`ilha-sinal${agente.enviando ? " ilha-sinal-ocupado" : ""}`} />{agente.estadoDoProvedor?.nome ?? "Conectando…"}</span>
      <button className="ilha-opcoes-botao" onClick={() => setOpcoes(!opcoes)} aria-expanded={opcoes}>{opcoes ? <X size={14} /> : <Settings2 size={14} />} {opcoes ? "Voltar à conversa" : "Opções"}</button>
    </div>
    {agente.mensagemDeErro && <p className="ilha-erro" role="alert">{agente.mensagemDeErro}</p>}
    {agente.estadoDoProvedor && !pronto && <p className="ilha-erro" role="alert">{agente.estadoDoProvedor.nome} não foi encontrado. Escolha outro provedor em Opções.</p>}
    {agente.aquecimento?.situacao === "falhou" && <div className="ilha-aviso">Não consegui preparar a saudação. Você pode enviar uma mensagem.<button onClick={() => void agente.tentarDeNovoAquecimento()}>Tentar novamente</button></div>}
    <div className="ilha-opcoes" hidden={!opcoes}>
      <label htmlFor="ilha-provedor">Conversar com</label>
      <select id="ilha-provedor" value={agente.estadoDoProvedor?.id ?? "opencode"} disabled={agente.enviando} onChange={e => void agente.trocarProvedor(e.target.value)}>
        <option value="opencode">OpenCode</option><option value="codex">Codex</option><option value="claude">Claude Code</option><option value="antigravity">Antigravity</option><option value="ollama">Ollama · somente conversa</option>
      </select>
      <h2>Ações no cofre</h2><HistoricoDoAgente registros={agente.historico} bloqueado={agente.enviando} aoDesfazer={agente.desfazerPedido} />
    </div>
    <div className="ilha-conversa" ref={conversa} hidden={opcoes} onScroll={() => { const el = conversa.current; if (el) acompanhar.current = el.scrollHeight - el.scrollTop - el.clientHeight < 70; }}>
      {agente.mensagens.length === 0 && !agente.enviando ? <div className="ilha-boas-vindas"><h2>O que está na sua cabeça?</h2><p>Guarde uma ideia, organize o dia<br />ou encontre algo no seu cofre.</p></div> :
        <ConversaDoAgente mensagens={agente.mensagens} enviando={agente.enviando} respostaParcial={agente.respostaParcial} passos={agente.passos} aoAprovarPlano={agente.aprovarPlano} />}
    </div>
    <div className="ilha-compositor" hidden={opcoes}>
      <FormularioDoPedido cofre={cofre} desativado={agente.enviando || !pronto} aoEnviar={texto => { acompanhar.current = true; void agente.enviarPedido(texto); }} pedidoInicial={pedido} aoConsumirPedidoInicial={consumirPedido} modo={agente.modoDoPedido} aoMudarModo={agente.definirModoDoPedido} compacto />
      <span className="ilha-dica">Enter envia <span aria-hidden="true">·</span> Shift + Enter quebra a linha</span>
    </div>
  </div>;
}
