// Estrelas do grafo: uma por nota, todas num único objeto de pontos com shader próprio.
// Cada estrela tem um miolo branco e um halo da cor da constelação, e cintila de leve.
// Esconder, apagar ou destacar uma estrela é só mudar o tamanho e o brilho dela.

import { AdditiveBlending, BufferAttribute, BufferGeometry, Points, ShaderMaterial } from "three";
import { corDoNo, criarPaletaDasPastas, tamanhoDaEstrela } from "./aparenciaDoGrafo";
import type { GrafoDoCofre } from "./contratoGrafo";
import { listarPastasDoGrafo, type VisaoDoGrafo } from "./filtrosDoGrafo";
import type { PecaDoDesenho } from "./pecaDoDesenho";

/** Quando a busca está ativa: quanto as estrelas que casam crescem e quanto brilham, e o quanto as outras se apagam. */
const CRESCIMENTO_DAS_ESTRELAS_ENCONTRADAS = 1.35;
const BRILHO_DAS_ESTRELAS_ENCONTRADAS = 1.5;
const ENCOLHIMENTO_DAS_ESTRELAS_APAGADAS = 0.6;
const BRILHO_DAS_ESTRELAS_APAGADAS = 0.15;

const SHADER_DE_VERTICE = /* glsl */ `
  attribute vec3 cor;
  attribute float tamanho;
  attribute float brilho;
  uniform float escalaDaTela;
  uniform float tempo;
  varying vec3 corDaEstrela;
  varying float brilhoDaEstrela;
  void main() {
    corDaEstrela = cor;
    brilhoDaEstrela = brilho;
    vec4 posicaoNaCamera = modelViewMatrix * vec4(position, 1.0);
    float cintilacao = 0.9 + 0.1 * sin(tempo * 2.0 + position.x * 0.37 + position.z * 0.91);
    gl_PointSize = tamanho * cintilacao * escalaDaTela / -posicaoNaCamera.z;
    gl_Position = projectionMatrix * posicaoNaCamera;
  }
`;

const SHADER_DE_FRAGMENTO = /* glsl */ `
  varying vec3 corDaEstrela;
  varying float brilhoDaEstrela;
  void main() {
    float distanciaDoCentro = length(gl_PointCoord - 0.5);
    if (distanciaDoCentro > 0.5) discard;
    float miolo = smoothstep(0.16, 0.0, distanciaDoCentro);
    float halo = pow(1.0 - distanciaDoCentro * 2.0, 2.5);
    vec3 luz = corDaEstrela * halo + vec3(1.0) * miolo;
    gl_FragColor = vec4(luz * brilhoDaEstrela, clamp(halo + miolo, 0.0, 1.0) * brilhoDaEstrela);
  }
`;

export interface DesenhoDasEstrelas extends PecaDoDesenho {
  /** Objeto de pontos; o `index` de um acerto do mouse é a posição em `grafo.nos`. */
  pontos: Points;
  aplicarVisao(visao: VisaoDoGrafo): void;
  /** Diz o tamanho da tela para o tamanho das estrelas acompanhar a perspectiva. */
  ajustarEscalaDaTela(alturaEmPixels: number, campoDeVisaoVerticalEmGraus: number): void;
}

export function criarDesenhoDasEstrelas(grafo: GrafoDoCofre): DesenhoDasEstrelas {
  const paleta = criarPaletaDasPastas(listarPastasDoGrafo(grafo));
  const posicoes = new Float32Array(grafo.nos.flatMap((no) => no.posicao));
  const cores = new Float32Array(grafo.nos.flatMap((no) => corDoNo(no, paleta).toArray()));
  const tamanhosBase = grafo.nos.map(tamanhoDaEstrela);
  const atributoDeTamanho = new BufferAttribute(new Float32Array(grafo.nos.length), 1);
  const atributoDeBrilho = new BufferAttribute(new Float32Array(grafo.nos.length), 1);

  const geometria = new BufferGeometry();
  geometria.setAttribute("position", new BufferAttribute(posicoes, 3));
  geometria.setAttribute("cor", new BufferAttribute(cores, 3));
  geometria.setAttribute("tamanho", atributoDeTamanho);
  geometria.setAttribute("brilho", atributoDeBrilho);

  const material = new ShaderMaterial({
    uniforms: { escalaDaTela: { value: 500 }, tempo: { value: 0 } },
    vertexShader: SHADER_DE_VERTICE,
    fragmentShader: SHADER_DE_FRAGMENTO,
    transparent: true,
    depthWrite: false,
    blending: AdditiveBlending,
  });
  const pontos = new Points(geometria, material);
  // O halo de uma estrela é maior que o ponto da posição dela. Sem isto, a cena cortaria estrelas
  // inteiras assim que o ponto central saísse da tela, mesmo com o halo ainda visível.
  pontos.frustumCulled = false;

  return {
    objetos: [pontos],
    pontos,
    aplicarVisao(visao) {
      const buscaAtiva = visao.nosEmDestaque !== null;
      tamanhosBase.forEach((tamanhoBase, posicao) => {
        const casaComABusca = !buscaAtiva || visao.nosEmDestaque?.[posicao] === true;
        let tamanho = tamanhoBase;
        if (buscaAtiva) tamanho *= casaComABusca ? CRESCIMENTO_DAS_ESTRELAS_ENCONTRADAS : ENCOLHIMENTO_DAS_ESTRELAS_APAGADAS;
        atributoDeTamanho.setX(posicao, visao.nosVisiveis[posicao] ? tamanho : 0);
        const brilho = casaComABusca ? (buscaAtiva ? BRILHO_DAS_ESTRELAS_ENCONTRADAS : 1) : BRILHO_DAS_ESTRELAS_APAGADAS;
        atributoDeBrilho.setX(posicao, brilho);
      });
      atributoDeTamanho.needsUpdate = true;
      atributoDeBrilho.needsUpdate = true;
    },
    ajustarEscalaDaTela(alturaEmPixels, campoDeVisaoVerticalEmGraus) {
      const tangenteDaMetadeDoCampo = Math.tan((campoDeVisaoVerticalEmGraus * Math.PI) / 360);
      material.uniforms.escalaDaTela.value = (alturaEmPixels * Math.min(window.devicePixelRatio, 2)) / (2 * tangenteDaMetadeDoCampo);
    },
    animar(tempoEmSegundos) {
      material.uniforms.tempo.value = tempoEmSegundos;
    },
    destruir() {
      geometria.dispose();
      material.dispose();
    },
  };
}
