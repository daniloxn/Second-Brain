// Tela de configurações: uma busca no topo filtra as opções pelo título e pelas palavras-chave.
// Hoje reúne o provedor do agente e os atalhos globais.

import { useEffect, useState } from "react";
import { definirProvedorDoAgente } from "../agente/chamadasAgente";
import { CampoDeAtalho } from "./CampoDeAtalho";
import { atalhosDoApp, definirAtalho } from "./chamadasConfiguracoes";
import type { AcaoDeAtalho, AtalhosDoApp } from "./contratoConfiguracoes";
import { opcaoCombinaComABusca } from "./regrasDeConfiguracao";

/** Provedores que aparecem na escolha. Mesmos identificadores do backend (`PROVEDORES_DISPONIVEIS`). */
const PROVEDORES = [
  { id: "opencode", nome: "OpenCode" },
  { id: "codex", nome: "Codex" },
  { id: "claude", nome: "Claude Code" },
  { id: "antigravity", nome: "Antigravity" },
  { id: "ollama", nome: "Ollama local (somente conversa)" },
];

/** Chave do localStorage com o provedor escolhido. A tela do agente e a ilha leem a mesma chave. */
const CHAVE_DO_PROVEDOR = "segundo-cerebro-provedor";

export function TelaDeConfiguracoes() {
  const [termo, setTermo] = useState("");
  const [atalhos, setAtalhos] = useState<AtalhosDoApp | null>(null);
  const [provedor, setProvedor] = useState(() => window.localStorage.getItem(CHAVE_DO_PROVEDOR) ?? "opencode");
  const [erro, setErro] = useState<string | null>(null);

  useEffect(() => {
    atalhosDoApp().then(setAtalhos).catch((falha) => setErro(String(falha)));
  }, []);

  /** Troca o provedor para os próximos pedidos e guarda a escolha, como a ilha faz. */
  async function escolherProvedor(id: string) {
    setErro(null);
    try {
      await definirProvedorDoAgente(id);
      window.localStorage.setItem(CHAVE_DO_PROVEDOR, id);
      setProvedor(id);
    } catch (falha) {
      setErro(String(falha));
    }
  }

  /** Troca uma combinação de atalho. Quando o backend recusa, a exceção sobe para o campo mostrar. */
  async function trocarAtalho(acao: AcaoDeAtalho, combinacao: string) {
    setAtalhos(await definirAtalho(acao, combinacao));
  }

  const opcoes = [
    {
      id: "provedor-do-agente",
      titulo: "Provedor do agente",
      palavrasChave: "codex claude code opencode ollama modelo ia conversa",
      controle: (
        <select aria-label="Provedor do agente" value={provedor} onChange={(evento) => void escolherProvedor(evento.target.value)}>
          {PROVEDORES.map((item) => <option key={item.id} value={item.id}>{item.nome}</option>)}
        </select>
      ),
    },
    {
      id: "atalho-da-ilha",
      titulo: "Atalho para abrir o chat",
      palavrasChave: "ilha atalho teclado global abrir conversa",
      controle: atalhos && (
        <CampoDeAtalho rotulo="Abrir o chat" combinacao={atalhos.abrirIlha} aoDefinir={(nova) => trocarAtalho("abrirIlha", nova)} />
      ),
    },
    {
      id: "atalho-da-janela",
      titulo: "Atalho para abrir a janela principal",
      palavrasChave: "janela principal cofre atalho teclado global",
      controle: atalhos && (
        <CampoDeAtalho rotulo="Abrir a janela" combinacao={atalhos.abrirJanelaPrincipal} aoDefinir={(nova) => trocarAtalho("abrirJanelaPrincipal", nova)} />
      ),
    },
  ];
  const opcoesVisiveis = opcoes.filter((opcao) => opcaoCombinaComABusca(`${opcao.titulo} ${opcao.palavrasChave}`, termo));

  return (
    <section className="configuracoes" aria-label="Configurações">
      <label className="configuracoes-busca">
        <span className="sr-only">Buscar nas configurações</span>
        <input
          type="search"
          placeholder="Buscar nas configurações"
          value={termo}
          onChange={(evento) => setTermo(evento.target.value)}
        />
      </label>
      {erro && <p role="alert" className="erro">{erro}</p>}
      {opcoesVisiveis.length === 0 ? (
        <p className="suave">Nenhuma configuração encontrada para "{termo}".</p>
      ) : (
        <ul className="configuracoes-lista">
          {opcoesVisiveis.map((opcao) => (
            <li key={opcao.id} className="configuracoes-opcao">
              <h2>{opcao.titulo}</h2>
              {opcao.controle}
            </li>
          ))}
        </ul>
      )}
    </section>
  );
}
