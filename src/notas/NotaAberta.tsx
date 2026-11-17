// Tela de uma nota aberta: barra de ações, editor ou leitura, e painel de backlinks.
// Quem usa deve passar `key={caminhoDaNota}`, para que cada nota comece do zero.

import { useState } from "react";
import { ArrowLeft, BookOpen, Pencil } from "lucide-react";
import { useSalvamentoDaNota, type EstadoDoSalvamento } from "./useSalvamentoDaNota";
import { EditorMarkdown } from "./EditorMarkdown";
import { LeituraMarkdown } from "./LeituraMarkdown";
import { PainelBacklinks } from "./PainelBacklinks";
import { FaixaDeConflito } from "./FaixaDeConflito";
import { caminhoRelativoAoCofre, nomeDaNotaPeloCaminho } from "./nomesDeNota";

/** Rótulo mostrado ao lado dos botões, para cada estado do salvamento. */
const ROTULO_DO_ESTADO: Record<EstadoDoSalvamento, string> = {
  carregando: "Carregando…",
  salvo: "Salvo",
  editando: "Editando…",
  salvando: "Salvando…",
  conflito: "Conflito com o disco",
  erro: "Não foi possível salvar",
};

type ModoDeVisualizacao = "editar" | "ler";

interface Props {
  caminhoDaNota: string;
  pastaDoCofre: string;
  aoClicarNoWikilink: (alvoDoLink: string) => void;
  aoClicarNoBacklink: (caminhoDaNota: string) => void;
  aoVoltarParaLista: () => void;
}

export function NotaAberta({
  caminhoDaNota,
  pastaDoCofre,
  aoClicarNoWikilink,
  aoClicarNoBacklink,
  aoVoltarParaLista,
}: Props) {
  const {
    textoDaTela,
    estadoDoSalvamento,
    mensagemDeErro,
    versaoDoTexto,
    aoMudarTexto,
    salvarAgora,
    usarVersaoDoDisco,
    manterMinhaVersao,
  } = useSalvamentoDaNota(caminhoDaNota);
  const [modoDeVisualizacao, setModoDeVisualizacao] = useState<ModoDeVisualizacao>("editar");

  const nomeDaNota = nomeDaNotaPeloCaminho(caminhoDaNota);
  const caminhoRelativo = caminhoRelativoAoCofre(caminhoDaNota, pastaDoCofre);

  return (
    <div className="nota">
      <header className="barra-nota">
        <button type="button" className="botao" onClick={aoVoltarParaLista} aria-label="Voltar para a lista de notas">
          <ArrowLeft size={15} aria-hidden="true" />
          <span className="texto-botao">Notas</span>
        </button>

        <div className="segmentado" role="group" aria-label="Modo de visualização">
          <button type="button" aria-pressed={modoDeVisualizacao === "editar"} onClick={() => setModoDeVisualizacao("editar")}>
            <Pencil size={14} aria-hidden="true" />
            Editar
          </button>
          <button type="button" aria-pressed={modoDeVisualizacao === "ler"} onClick={() => setModoDeVisualizacao("ler")}>
            <BookOpen size={14} aria-hidden="true" />
            Ler
          </button>
        </div>

        <span className={`status status-${estadoDoSalvamento}`} aria-live="polite">
          {ROTULO_DO_ESTADO[estadoDoSalvamento]}
        </span>
      </header>

      {mensagemDeErro && <p role="alert" className="erro faixa">{mensagemDeErro}</p>}
      {estadoDoSalvamento === "conflito" && (
        <FaixaDeConflito aoUsarDoDisco={usarVersaoDoDisco} aoManterMinha={manterMinhaVersao} />
      )}

      <div className="nota-corpo">
        <div className="nota-area">
          <div className="nota-cabecalho">
            <h1 className="titulo-nota">{nomeDaNota}</h1>
            <span className="caminho" title={caminhoDaNota}>{caminhoRelativo}</span>
          </div>

          {textoDaTela === null ? (
            <p className="suave">Carregando nota…</p>
          ) : modoDeVisualizacao === "editar" ? (
            <EditorMarkdown
              key={versaoDoTexto}
              textoInicial={textoDaTela}
              aoMudarTexto={aoMudarTexto}
              aoSalvarAgora={salvarAgora}
            />
          ) : (
            <div className="leitura">
              <LeituraMarkdown textoDaNota={textoDaTela} aoClicarNoWikilink={aoClicarNoWikilink} />
            </div>
          )}
        </div>

        <PainelBacklinks
          nomeDaNota={nomeDaNota}
          caminhoDaNota={caminhoDaNota}
          aoClicarNoBacklink={aoClicarNoBacklink}
        />
      </div>
    </div>
  );
}
