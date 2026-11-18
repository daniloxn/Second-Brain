// Junta as peças do desenho do universo (fundo, órbitas, ligações, buraco negro e estrelas) numa
// interface só para a cena. Não cria a cena nem trata o mouse: isso é de cenaDoGrafo.ts.

import type { Camera, Object3D, Points } from "three";
import type { GrafoDoCofre } from "./contratoGrafo";
import { criarDesenhoDasEstrelas } from "./desenhoDasEstrelas";
import { criarDesenhoDasLigacoes } from "./desenhoDasLigacoes";
import { criarDesenhoDasOrbitas } from "./desenhoDasOrbitas";
import { criarDesenhoDoBuracoNegro } from "./desenhoDoBuracoNegro";
import { criarDesenhoDoFundoEstrelado } from "./desenhoDoFundoEstrelado";
import type { VisaoDoGrafo } from "./filtrosDoGrafo";

export interface DesenhoDoGrafo {
  /** Objetos para pôr na cena, do fundo para a frente. */
  objetos: Object3D[];
  /** Objeto de pontos das estrelas; o `index` de um acerto do mouse é a posição em `grafo.nos`. */
  pontosDasEstrelas: Points;
  aplicarVisao(visao: VisaoDoGrafo): void;
  ajustarEscalaDaTela(alturaEmPixels: number, campoDeVisaoVerticalEmGraus: number): void;
  animar(tempoEmSegundos: number, camera: Camera): void;
  destruir(): void;
}

export function criarDesenhoDoGrafo(grafo: GrafoDoCofre): DesenhoDoGrafo {
  const fundo = criarDesenhoDoFundoEstrelado(grafo.volume);
  const orbitas = criarDesenhoDasOrbitas(grafo.volume);
  const ligacoes = criarDesenhoDasLigacoes(grafo);
  const buracoNegro = criarDesenhoDoBuracoNegro(grafo.volume);
  const estrelas = criarDesenhoDasEstrelas(grafo);
  const pecas = [fundo, orbitas, ligacoes, buracoNegro, estrelas];

  return {
    objetos: pecas.flatMap((peca) => peca.objetos),
    pontosDasEstrelas: estrelas.pontos,
    aplicarVisao(visao) {
      estrelas.aplicarVisao(visao);
      ligacoes.aplicarVisao(visao);
    },
    ajustarEscalaDaTela: estrelas.ajustarEscalaDaTela,
    animar(tempoEmSegundos, camera) {
      pecas.forEach((peca) => peca.animar?.(tempoEmSegundos, camera));
    },
    destruir() {
      pecas.forEach((peca) => peca.destruir());
    },
  };
}
