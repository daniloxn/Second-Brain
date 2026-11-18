// Ligações do grafo: linhas finas de luz entre as estrelas que se citam por wikilink.
// Só as ligações visíveis são gravadas, uma atrás da outra, e o resto da memória não é desenhado.

import { AdditiveBlending, BufferAttribute, BufferGeometry, LineBasicMaterial, LineSegments } from "three";
import { COR_DA_LUZ_DAS_LIGACOES } from "./aparenciaDoGrafo";
import type { GrafoDoCofre } from "./contratoGrafo";
import type { VisaoDoGrafo } from "./filtrosDoGrafo";
import type { PecaDoDesenho } from "./pecaDoDesenho";

/** Bem discretas: a luz das ligações soma com a das estrelas e não pode competir com elas. */
const OPACIDADE_DAS_LIGACOES = 0.22;

export interface DesenhoDasLigacoes extends PecaDoDesenho {
  aplicarVisao(visao: VisaoDoGrafo): void;
}

export function criarDesenhoDasLigacoes(grafo: GrafoDoCofre): DesenhoDasLigacoes {
  const posicoes = new Float32Array(grafo.ligacoes.length * 6);
  const atributoDasPosicoes = new BufferAttribute(posicoes, 3);
  const geometria = new BufferGeometry();
  geometria.setAttribute("position", atributoDasPosicoes);
  const material = new LineBasicMaterial({
    color: COR_DA_LUZ_DAS_LIGACOES,
    transparent: true,
    opacity: OPACIDADE_DAS_LIGACOES,
    blending: AdditiveBlending,
    depthWrite: false,
  });
  const linhas = new LineSegments(geometria, material);
  // A caixa de colisão é calculada uma vez e ficaria velha quando as ligações visíveis mudam.
  linhas.frustumCulled = false;

  return {
    objetos: [linhas],
    aplicarVisao(visao) {
      let ligacoesGravadas = 0;
      grafo.ligacoes.forEach((ligacao, posicao) => {
        if (!visao.ligacoesVisiveis[posicao]) return;
        posicoes.set(grafo.nos[ligacao.origem].posicao, ligacoesGravadas * 6);
        posicoes.set(grafo.nos[ligacao.destino].posicao, ligacoesGravadas * 6 + 3);
        ligacoesGravadas++;
      });
      geometria.setDrawRange(0, ligacoesGravadas * 2);
      atributoDasPosicoes.needsUpdate = true;
    },
    destruir() {
      geometria.dispose();
      material.dispose();
    },
  };
}
