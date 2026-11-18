// Painel flutuante do grafo: busca com resultados clicáveis, filtros e as pastas com a cor de cada uma.
// Fica por cima do universo e pode ser fechado pelo botão da tela.

import { useMemo } from "react";
import { Search } from "lucide-react";
import { corDaPastaEmCss, criarPaletaDasPastas } from "./aparenciaDoGrafo";
import type { GrafoDoCofre } from "./contratoGrafo";
import {
  listarPastasDoGrafo,
  nomeDaPastaParaExibir,
  resultadosDaBusca,
  type FiltrosDoGrafo,
  type VisaoDoGrafo,
} from "./filtrosDoGrafo";

interface Props {
  id: string;
  grafo: GrafoDoCofre;
  visao: VisaoDoGrafo;
  filtros: FiltrosDoGrafo;
  aoMudarFiltros: (mudanca: Partial<FiltrosDoGrafo>) => void;
  aoAbrirNota: (caminhoDaNota: string) => void;
}

export function PainelDeFiltrosDoGrafo({ id, grafo, visao, filtros, aoMudarFiltros, aoAbrirNota }: Props) {
  // A mesma lista de pastas dá a mesma paleta do desenho, então as bolinhas combinam com as estrelas.
  const pastas = useMemo(() => listarPastasDoGrafo(grafo), [grafo]);
  const paletaDasPastas = useMemo(() => criarPaletaDasPastas(pastas), [pastas]);
  const resultados = resultadosDaBusca(visao);
  const buscaPreenchida = filtros.busca.trim() !== "";

  return (
    <aside id={id} className="grafo-painel" aria-label="Filtros do grafo">
      {grafo.truncado && (
        <p className="suave">O cofre tem mais notas do que o grafo mostra. Só as primeiras em ordem alfabética aparecem.</p>
      )}

      <label className="grafo-busca">
        <Search size={15} aria-hidden="true" />
        <input
          type="search"
          className="campo"
          placeholder="Buscar nota"
          value={filtros.busca}
          onChange={(evento) => aoMudarFiltros({ busca: evento.target.value })}
        />
      </label>

      {buscaPreenchida && (
        <ul className="grafo-resultados" aria-label="Resultados da busca">
          {resultados.length === 0 && <li className="suave">Nenhuma nota encontrada.</li>}
          {resultados.map((posicao) => (
            <li key={grafo.nos[posicao].caminho}>
              <button type="button" className="grafo-resultado" onClick={() => aoAbrirNota(grafo.nos[posicao].caminho)}>
                {grafo.nos[posicao].nome}
              </button>
            </li>
          ))}
        </ul>
      )}

      <fieldset className="grafo-grupo">
        <legend className="rotulo">Mostrar</legend>
        <label className="grafo-opcao">
          <input
            type="checkbox"
            checked={filtros.mostrarTarefas}
            onChange={(evento) => aoMudarFiltros({ mostrarTarefas: evento.target.checked })}
          />
          Tarefas
        </label>
        <label className="grafo-opcao">
          <input
            type="checkbox"
            checked={filtros.mostrarIsoladas}
            onChange={(evento) => aoMudarFiltros({ mostrarIsoladas: evento.target.checked })}
          />
          Notas sem ligações
        </label>
      </fieldset>

      <section className="grafo-grupo" aria-label="Constelações">
        <h3 className="rotulo">Constelação</h3>
        <ul className="grafo-pastas">
          <li>
            <button
              type="button"
              className="grafo-pasta"
              aria-pressed={filtros.pastaEscolhida === null}
              onClick={() => aoMudarFiltros({ pastaEscolhida: null })}
            >
              Todas
            </button>
          </li>
          {pastas.map((pasta) => (
            <li key={pasta}>
              <button
                type="button"
                className="grafo-pasta"
                aria-pressed={filtros.pastaEscolhida === pasta}
                onClick={() => aoMudarFiltros({ pastaEscolhida: pasta })}
              >
                <span className="grafo-pasta-cor" style={{ background: corDaPastaEmCss(pasta, paletaDasPastas) }} aria-hidden="true" />
                {nomeDaPastaParaExibir(pasta)}
              </button>
            </li>
          ))}
        </ul>
        <p className="suave">Arraste para girar, role para aproximar e clique em uma estrela para abrir a nota. Tarefas são laranja.</p>
      </section>
    </aside>
  );
}
