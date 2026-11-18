// Tela da seção Grafo: o universo em tela cheia, com o resumo no canto e o painel de filtros flutuando por cima.
// Trata os estados de carregamento, erro e cofre sem notas; o resto fica no quadro e no painel.

import { useMemo, useState } from "react";
import { SlidersHorizontal } from "lucide-react";
import { montarConstelacoes } from "./constelacoesDoGrafo";
import { contarVisiveis } from "./filtrosDoGrafo";
import { useGrafo } from "./useGrafo";
import { QuadroDoGrafo } from "./QuadroDoGrafo";
import { PainelDeFiltrosDoGrafo } from "./PainelDeFiltrosDoGrafo";

interface Props {
  aoAbrirNota: (caminhoDaNota: string) => void;
}

/** Abaixo desta largura o painel começa fechado, para não cobrir o universo. */
const LARGURA_MINIMA_PARA_PAINEL_ABERTO = 900;

const ID_DO_PAINEL_DE_FILTROS = "painel-de-filtros-do-grafo";

export function TelaDoGrafo({ aoAbrirNota }: Props) {
  const { grafo, visao, filtros, mudarFiltros, estaCarregando, mensagemDeErro } = useGrafo();
  const [painelAberto, setPainelAberto] = useState(() => window.innerWidth > LARGURA_MINIMA_PARA_PAINEL_ABERTO);
  const constelacoes = useMemo(() => (grafo && visao ? montarConstelacoes(grafo, visao) : []), [grafo, visao]);

  if (estaCarregando) return <p className="suave grafo-aviso">Abrindo o universo…</p>;
  if (mensagemDeErro) return <p role="alert" className="erro grafo-aviso">{mensagemDeErro}</p>;
  if (!grafo || !visao || grafo.nos.length === 0) {
    return <p className="suave grafo-aviso">Ainda não há notas para mostrar. Crie notas e ligue-as com [[wikilinks]].</p>;
  }

  const totais = contarVisiveis(visao);
  return (
    <div className="tela-grafo">
      <QuadroDoGrafo grafo={grafo} visao={visao} aoAbrirNota={aoAbrirNota} />

      <div className="grafo-resumo">
        <h2>Seu universo</h2>
        <p>
          {totais.nos === 1 ? "1 estrela" : `${totais.nos} estrelas`} · {totais.ligacoes === 1 ? "1 ligação" : `${totais.ligacoes} ligações`} ·{" "}
          {constelacoes.length === 1 ? "1 constelação" : `${constelacoes.length} constelações`}
        </p>
      </div>

      <button
        type="button"
        className="botao grafo-alternar-painel"
        aria-expanded={painelAberto}
        aria-controls={ID_DO_PAINEL_DE_FILTROS}
        onClick={() => setPainelAberto((aberto) => !aberto)}
      >
        <SlidersHorizontal size={15} aria-hidden="true" />
        Filtros
      </button>
      {painelAberto && (
        <PainelDeFiltrosDoGrafo
          id={ID_DO_PAINEL_DE_FILTROS}
          grafo={grafo}
          visao={visao}
          filtros={filtros}
          aoMudarFiltros={mudarFiltros}
          aoAbrirNota={aoAbrirNota}
        />
      )}
    </div>
  );
}
