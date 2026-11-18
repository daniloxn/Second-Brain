// Painel de projetos: seleciona uma pasta e mostra o resumo, mapa técnico ou log do agente.

import { useEffect, useState } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { criarProjeto, listarProjetos } from "./chamadasProjetos";
import type { ProjetoDoCofre } from "./contratoProjetos";
import { FormularioNovoProjeto } from "./FormularioNovoProjeto";

type DocumentoDoProjeto = "projeto" | "mapa" | "log";

const ROTULOS_DOS_DOCUMENTOS: Record<DocumentoDoProjeto, string> = {
  projeto: "Projeto",
  mapa: "MAPA.md",
  log: "LOG.md",
};

/** Busca projetos ao abrir e permite criar ou consultar seus documentos de contexto. */
export function TelaDeProjetos() {
  const [projetos, setProjetos] = useState<ProjetoDoCofre[]>([]);
  const [projetoSelecionado, setProjetoSelecionado] = useState<string | null>(null);
  const [documentoSelecionado, setDocumentoSelecionado] = useState<DocumentoDoProjeto>("projeto");
  const [estaCarregando, setEstaCarregando] = useState(true);
  const [mensagemDeErro, setMensagemDeErro] = useState<string | null>(null);

  useEffect(() => {
    let telaMontada = true;
    listarProjetos()
      .then((projetosLidos) => {
        if (!telaMontada) return;
        setProjetos(projetosLidos);
        setProjetoSelecionado(projetosLidos[0]?.nome ?? null);
      })
      .catch((erro) => {
        if (telaMontada) setMensagemDeErro(String(erro));
      })
      .finally(() => {
        if (telaMontada) setEstaCarregando(false);
      });
    return () => {
      telaMontada = false;
    };
  }, []);

  /** Cria o projeto e o seleciona imediatamente para revisão dos modelos. */
  async function criarProjetoNoCofre(nome: string) {
    const projetoCriado = await criarProjeto(nome);
    setProjetos((projetosAtuais) => [...projetosAtuais, projetoCriado].sort((a, b) => a.nome.localeCompare(b.nome)));
    setProjetoSelecionado(projetoCriado.nome);
    setDocumentoSelecionado("projeto");
    setMensagemDeErro(null);
  }

  const projetoAtual = projetos.find((projeto) => projeto.nome === projetoSelecionado) ?? null;
  const conteudoDoDocumento = projetoAtual?.[documentoSelecionado] ?? "";

  return (
    <div className="tela-projetos">
      <div className="topo-projetos">
        <div>
          <h2 className="rotulo">Projetos dos agentes</h2>
          <p className="suave">Cada projeto tem um resumo, um mapa técnico e um histórico de trabalho.</p>
        </div>
      </div>

      <FormularioNovoProjeto aoCriar={criarProjetoNoCofre} />
      {mensagemDeErro && <p role="alert" className="erro">{mensagemDeErro}</p>}
      {estaCarregando && <p className="suave">Carregando projetos…</p>}

      {!estaCarregando && projetos.length === 0 && (
        <p className="suave">Ainda não há projetos. Crie um para reservar um espaço de trabalho e contexto para os agentes.</p>
      )}

      {projetos.length > 0 && (
        <div className="painel-projetos">
          <nav className="lista-projetos" aria-label="Projetos">
            {projetos.map((projeto) => (
              <button
                key={projeto.nome}
                type="button"
                className="botao-projeto"
                aria-current={projeto.nome === projetoSelecionado ? "true" : undefined}
                onClick={() => setProjetoSelecionado(projeto.nome)}
              >
                <strong>{projeto.nome}</strong>
                <span>{projeto.caminhoRelativo}</span>
              </button>
            ))}
          </nav>

          {projetoAtual && (
            <section className="detalhe-projeto" aria-label={`Projeto ${projetoAtual.nome}`}>
              <header className="cabecalho-projeto">
                <div>
                  <h3>{projetoAtual.nome}</h3>
                  <span className="suave">{projetoAtual.caminhoRelativo}</span>
                </div>
                <div className="abas-documentos-projeto" role="tablist" aria-label="Documentos do projeto">
                  {(Object.keys(ROTULOS_DOS_DOCUMENTOS) as DocumentoDoProjeto[]).map((documento) => (
                    <button
                      key={documento}
                      type="button"
                      role="tab"
                      aria-selected={documento === documentoSelecionado}
                      className="botao"
                      onClick={() => setDocumentoSelecionado(documento)}
                    >
                      {ROTULOS_DOS_DOCUMENTOS[documento]}
                    </button>
                  ))}
                </div>
              </header>
              <article className="markdown-projeto">
                {conteudoDoDocumento
                  ? <ReactMarkdown remarkPlugins={[remarkGfm]}>{conteudoDoDocumento}</ReactMarkdown>
                  : <p className="suave">O agente ainda não preencheu este documento.</p>}
              </article>
            </section>
          )}
        </div>
      )}
    </div>
  );
}
