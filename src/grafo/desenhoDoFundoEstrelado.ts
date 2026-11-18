// Fundo do universo: milhares de estrelas distantes numa casca grande ao redor da câmera e
// algumas nebulosas suaves dentro do disco da galáxia. Posições vêm de um sorteio com semente
// fixa, então o fundo é sempre o mesmo e não pisca entre uma abertura e outra.

import { AdditiveBlending, BufferAttribute, BufferGeometry, Color, Points, PointsMaterial, Sprite, SpriteMaterial } from "three";
import type { VolumeDaGalaxia } from "./contratoGrafo";
import type { PecaDoDesenho } from "./pecaDoDesenho";
import { criarTexturaDeBrilhoRadial } from "./texturaDeBrilhoRadial";

const QUANTIDADE_DE_ESTRELAS_DE_FUNDO = 1600;
/** A casca de estrelas começa e termina longe, como múltiplos do raio da galáxia. Fica além do zoom máximo. */
const RAIO_MINIMO_DA_CASCA_EM_RAIOS_DA_GALAXIA = 4;
const RAIO_MAXIMO_DA_CASCA_EM_RAIOS_DA_GALAXIA = 7;
const TAMANHO_DAS_ESTRELAS_DE_FUNDO_EM_PIXELS = 2.4;
const OPACIDADE_DAS_ESTRELAS_DE_FUNDO = 0.75;
/** Radianos por segundo: o fundo gira devagar, dando noção de profundidade. */
const VELOCIDADE_DE_ROTACAO_DO_FUNDO = 0.004;

/** Nebulosas: cor (CSS), diâmetro como fração do raio da galáxia e opacidade. As posições vêm do sorteio. */
const NEBULOSAS = [
  { cor: "rgba(255, 170, 90, 1)", fracaoDoDiametro: 1.3, opacidade: 0.1 },
  { cor: "rgba(120, 140, 255, 1)", fracaoDoDiametro: 1.6, opacidade: 0.09 },
  { cor: "rgba(200, 110, 255, 1)", fracaoDoDiametro: 1.2, opacidade: 0.08 },
  { cor: "rgba(90, 220, 200, 1)", fracaoDoDiametro: 1.1, opacidade: 0.06 },
  { cor: "rgba(255, 120, 150, 1)", fracaoDoDiametro: 1.4, opacidade: 0.06 },
];

/** Cores de luz das estrelas de fundo: brancas, azuladas e amareladas. */
const CORES_DAS_ESTRELAS_DE_FUNDO = ["#ffffff", "#cfdcff", "#fff2cf", "#ffe0c0"];

/** Gerador Mulberry32: devolve uma função que dá números entre 0 e 1 sempre na mesma ordem para a mesma semente. */
function criarSorteioComSemente(semente: number): () => number {
  let estado = semente;
  return () => {
    estado = (estado + 0x6d2b79f5) | 0;
    let misturado = Math.imul(estado ^ (estado >>> 15), 1 | estado);
    misturado = (misturado + Math.imul(misturado ^ (misturado >>> 7), 61 | misturado)) ^ misturado;
    return ((misturado ^ (misturado >>> 14)) >>> 0) / 4294967296;
  };
}

export function criarDesenhoDoFundoEstrelado(volume: VolumeDaGalaxia): PecaDoDesenho {
  const sortear = criarSorteioComSemente(20261117);
  const posicoes = new Float32Array(QUANTIDADE_DE_ESTRELAS_DE_FUNDO * 3);
  const cores = new Float32Array(QUANTIDADE_DE_ESTRELAS_DE_FUNDO * 3);
  const raioMinimo = volume.raio * RAIO_MINIMO_DA_CASCA_EM_RAIOS_DA_GALAXIA;
  const raioMaximo = volume.raio * RAIO_MAXIMO_DA_CASCA_EM_RAIOS_DA_GALAXIA;

  for (let estrela = 0; estrela < QUANTIDADE_DE_ESTRELAS_DE_FUNDO; estrela++) {
    // Direção uniforme na esfera: z entre -1 e 1 e um ângulo qualquer.
    const z = sortear() * 2 - 1;
    const angulo = sortear() * Math.PI * 2;
    const raioNoPlano = Math.sqrt(1 - z * z);
    const distancia = raioMinimo + sortear() * (raioMaximo - raioMinimo);
    posicoes.set([raioNoPlano * Math.cos(angulo) * distancia, z * distancia, raioNoPlano * Math.sin(angulo) * distancia], estrela * 3);

    const corSorteada = new Color(CORES_DAS_ESTRELAS_DE_FUNDO[Math.floor(sortear() * CORES_DAS_ESTRELAS_DE_FUNDO.length)]);
    const brilhoSorteado = 0.35 + sortear() * 0.65;
    cores.set(corSorteada.multiplyScalar(brilhoSorteado).toArray(), estrela * 3);
  }

  const geometriaDoFundo = new BufferGeometry();
  geometriaDoFundo.setAttribute("position", new BufferAttribute(posicoes, 3));
  geometriaDoFundo.setAttribute("color", new BufferAttribute(cores, 3));
  const texturaDasEstrelas = criarTexturaDeBrilhoRadial("rgba(255, 255, 255, 1)", 0.25);
  const materialDoFundo = new PointsMaterial({
    map: texturaDasEstrelas,
    size: TAMANHO_DAS_ESTRELAS_DE_FUNDO_EM_PIXELS,
    sizeAttenuation: false,
    vertexColors: true,
    transparent: true,
    opacity: OPACIDADE_DAS_ESTRELAS_DE_FUNDO,
    depthWrite: false,
  });
  const estrelasDeFundo = new Points(geometriaDoFundo, materialDoFundo);
  estrelasDeFundo.frustumCulled = false;

  const texturasDasNebulosas = NEBULOSAS.map((nebulosa) => criarTexturaDeBrilhoRadial(nebulosa.cor, 0.01));
  const nebulosas = NEBULOSAS.map((nebulosa, posicao) => {
    const material = new SpriteMaterial({
      map: texturasDasNebulosas[posicao],
      transparent: true,
      opacity: nebulosa.opacidade,
      depthWrite: false,
      blending: AdditiveBlending,
    });
    const sprite = new Sprite(material);
    sprite.scale.setScalar(volume.raio * nebulosa.fracaoDoDiametro);
    // Dentro do disco: distância do centro até 80% do raio e pouca altura.
    const distanciaDoCentro = Math.sqrt(sortear()) * volume.raio * 0.8;
    const anguloNoDisco = sortear() * Math.PI * 2;
    sprite.position.set(Math.cos(anguloNoDisco) * distanciaDoCentro, (sortear() - 0.5) * volume.semiEspessura, Math.sin(anguloNoDisco) * distanciaDoCentro);
    return { sprite, material };
  });

  return {
    objetos: [estrelasDeFundo, ...nebulosas.map((nebulosa) => nebulosa.sprite)],
    animar(tempoEmSegundos) {
      estrelasDeFundo.rotation.y = tempoEmSegundos * VELOCIDADE_DE_ROTACAO_DO_FUNDO;
    },
    destruir() {
      geometriaDoFundo.dispose();
      materialDoFundo.dispose();
      texturaDasEstrelas.dispose();
      nebulosas.forEach((nebulosa) => nebulosa.material.dispose());
      texturasDasNebulosas.forEach((textura) => textura.dispose());
    },
  };
}
