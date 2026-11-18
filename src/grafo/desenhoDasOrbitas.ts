// Órbitas ao redor da galáxia: círculos finos e, em cada um, um cometa com cauda que percorre a órbita.
// É só ambientação, sem dado nenhum do cofre.

import {
  AdditiveBlending,
  BufferAttribute,
  BufferGeometry,
  LineBasicMaterial,
  LineLoop,
  Object3D,
  Points,
  PointsMaterial,
} from "three";
import type { VolumeDaGalaxia } from "./contratoGrafo";
import type { PecaDoDesenho } from "./pecaDoDesenho";
import { criarTexturaDeBrilhoRadial } from "./texturaDeBrilhoRadial";

/** Cada órbita: raio como fração do raio da galáxia, inclinação (radianos), velocidade (rad/s) e ângulo de partida do cometa. */
const ORBITAS = [
  { fracaoDoRaio: 0.62, inclinacao: 0.12, velocidade: 0.11, anguloInicial: 0.4 },
  { fracaoDoRaio: 0.97, inclinacao: -0.18, velocidade: 0.07, anguloInicial: 2.6 },
  { fracaoDoRaio: 1.32, inclinacao: 0.26, velocidade: 0.045, anguloInicial: 4.5 },
];

const PONTOS_POR_CIRCULO = 160;
const OPACIDADE_DOS_CIRCULOS = 0.09;
const PONTOS_NA_CAUDA_DO_COMETA = 14;
/** Distância angular entre dois pontos da cauda, em radianos. */
const PASSO_DA_CAUDA = 0.022;
const TAMANHO_DOS_PONTOS_DO_COMETA_EM_PIXELS = 7;

/** Pontos de um círculo no plano x-z de raio 1, para escalar depois. */
function pontosDoCirculo(): Float32Array {
  const posicoes = new Float32Array(PONTOS_POR_CIRCULO * 3);
  for (let ponto = 0; ponto < PONTOS_POR_CIRCULO; ponto++) {
    const angulo = (ponto / PONTOS_POR_CIRCULO) * Math.PI * 2;
    posicoes.set([Math.cos(angulo), 0, Math.sin(angulo)], ponto * 3);
  }
  return posicoes;
}

export function criarDesenhoDasOrbitas(volume: VolumeDaGalaxia): PecaDoDesenho {
  const geometriaDoCirculo = new BufferGeometry();
  geometriaDoCirculo.setAttribute("position", new BufferAttribute(pontosDoCirculo(), 3));
  const materialDosCirculos = new LineBasicMaterial({
    color: 0xb8c8e8,
    transparent: true,
    opacity: OPACIDADE_DOS_CIRCULOS,
    blending: AdditiveBlending,
    depthWrite: false,
  });

  const texturaDoCometa = criarTexturaDeBrilhoRadial("rgba(255, 235, 200, 1)", 0.1);
  const materialDosCometas = new PointsMaterial({
    map: texturaDoCometa,
    size: TAMANHO_DOS_PONTOS_DO_COMETA_EM_PIXELS,
    sizeAttenuation: false,
    vertexColors: true,
    transparent: true,
    depthWrite: false,
    blending: AdditiveBlending,
  });

  const grupos = ORBITAS.map((orbita) => {
    const raio = volume.raio * orbita.fracaoDoRaio;
    const plano = new Object3D();
    plano.rotation.x = orbita.inclinacao;
    const circulo = new LineLoop(geometriaDoCirculo, materialDosCirculos);
    circulo.scale.setScalar(raio);
    circulo.frustumCulled = false;

    // A cauda esmaece da cabeça para o fim: a cor vai a zero e a mistura aditiva faz o resto.
    const posicoesDaCauda = new Float32Array(PONTOS_NA_CAUDA_DO_COMETA * 3);
    const coresDaCauda = new Float32Array(PONTOS_NA_CAUDA_DO_COMETA * 3);
    for (let ponto = 0; ponto < PONTOS_NA_CAUDA_DO_COMETA; ponto++) {
      const intensidade = (1 - ponto / PONTOS_NA_CAUDA_DO_COMETA) ** 2;
      coresDaCauda.set([intensidade, intensidade * 0.9, intensidade * 0.72], ponto * 3);
    }
    const atributoDasPosicoes = new BufferAttribute(posicoesDaCauda, 3);
    const geometriaDoCometa = new BufferGeometry();
    geometriaDoCometa.setAttribute("position", atributoDasPosicoes);
    geometriaDoCometa.setAttribute("color", new BufferAttribute(coresDaCauda, 3));
    const cometa = new Points(geometriaDoCometa, materialDosCometas);
    cometa.frustumCulled = false;

    plano.add(circulo, cometa);
    return { plano, raio, orbita, atributoDasPosicoes, posicoesDaCauda, geometriaDoCometa };
  });

  return {
    objetos: grupos.map((grupo) => grupo.plano),
    animar(tempoEmSegundos) {
      for (const { raio, orbita, atributoDasPosicoes, posicoesDaCauda } of grupos) {
        const anguloDaCabeca = orbita.anguloInicial + tempoEmSegundos * orbita.velocidade;
        for (let ponto = 0; ponto < PONTOS_NA_CAUDA_DO_COMETA; ponto++) {
          const angulo = anguloDaCabeca - ponto * PASSO_DA_CAUDA;
          posicoesDaCauda.set([Math.cos(angulo) * raio, 0, Math.sin(angulo) * raio], ponto * 3);
        }
        atributoDasPosicoes.needsUpdate = true;
      }
    },
    destruir() {
      geometriaDoCirculo.dispose();
      materialDosCirculos.dispose();
      materialDosCometas.dispose();
      texturaDoCometa.dispose();
      grupos.forEach((grupo) => grupo.geometriaDoCometa.dispose());
    },
  };
}
