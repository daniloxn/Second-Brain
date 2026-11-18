// O buraco negro "Home" no centro da galáxia: esfera escura, disco de acreção que gira,
// anel de luz que sempre olha para a câmera e um halo dourado ao redor. É só desenho;
// o espaço vazio do núcleo vem do backend (`raioDoNucleo`).

import {
  AdditiveBlending,
  DoubleSide,
  Mesh,
  MeshBasicMaterial,
  RingGeometry,
  ShaderMaterial,
  Sprite,
  SpriteMaterial,
  SphereGeometry,
  type Camera,
} from "three";
import type { VolumeDaGalaxia } from "./contratoGrafo";
import type { PecaDoDesenho } from "./pecaDoDesenho";
import { criarTexturaDeBrilhoRadial } from "./texturaDeBrilhoRadial";

/** Tamanhos como fração do raio do núcleo. Tudo cabe dentro do vazio onde nenhuma estrela entra. */
const FRACAO_DO_RAIO_DA_ESFERA_ESCURA = 0.34;
const FRACAO_DO_RAIO_INTERNO_DO_DISCO = 0.42;
const FRACAO_DO_RAIO_EXTERNO_DO_DISCO = 1.15;
const FRACAO_DO_RAIO_DO_ANEL_DE_LUZ = 0.4;
const FRACAO_DA_LARGURA_DO_ANEL_DE_LUZ = 0.07;
const DIAMETRO_DO_HALO_EM_RAIOS_DO_NUCLEO = 7;

/** Inclinação do disco em relação ao plano da galáxia, para ele não ficar chapado. */
const INCLINACAO_DO_DISCO_EM_RADIANOS: [number, number] = [0.16, 0.1];
const OPACIDADE_DO_HALO = 0.5;
/** Voltas por segundo do padrão de luz no disco, em radianos. */
const VELOCIDADE_DO_DISCO = 0.8;

const SHADER_DE_VERTICE_PLANO = /* glsl */ `
  varying vec2 posicaoNoPlano;
  void main() {
    posicaoNoPlano = position.xy;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
  }
`;

/** Disco: claro e dourado por dentro, laranja e transparente por fora, com faixas que giram e um lado mais brilhante. */
const SHADER_DE_FRAGMENTO_DO_DISCO = /* glsl */ `
  uniform float tempo;
  uniform float raioInterno;
  uniform float raioExterno;
  varying vec2 posicaoNoPlano;
  void main() {
    float raio = length(posicaoNoPlano);
    float posicaoNaFaixa = clamp((raio - raioInterno) / (raioExterno - raioInterno), 0.0, 1.0);
    float angulo = atan(posicaoNoPlano.y, posicaoNoPlano.x);
    float faixas = 0.75 + 0.25 * sin(angulo * 4.0 - tempo * 0.8 + raio * 0.35);
    float ladoMaisBrilhante = 0.55 + 0.45 * cos(angulo - 0.6);
    float intensidade = pow(1.0 - posicaoNaFaixa, 1.6) * faixas * ladoMaisBrilhante;
    vec3 cor = mix(vec3(1.0, 0.93, 0.75), vec3(1.0, 0.55, 0.18), posicaoNaFaixa);
    gl_FragColor = vec4(cor * intensidade * 1.6, intensidade);
  }
`;

/** Anel fino com borda suave, na cor do ouro. */
const SHADER_DE_FRAGMENTO_DO_ANEL = /* glsl */ `
  uniform float raioDoCentroDoAnel;
  uniform float metadeDaLarguraDoAnel;
  varying vec2 posicaoNoPlano;
  void main() {
    float proximidade = 1.0 - abs(length(posicaoNoPlano) - raioDoCentroDoAnel) / metadeDaLarguraDoAnel;
    float intensidade = pow(clamp(proximidade, 0.0, 1.0), 2.0);
    gl_FragColor = vec4(vec3(1.0, 0.88, 0.6) * intensidade * 1.4, intensidade);
  }
`;

export function criarDesenhoDoBuracoNegro(volume: VolumeDaGalaxia): PecaDoDesenho {
  const raioDoNucleo = volume.raioDoNucleo;

  const geometriaDaEsfera = new SphereGeometry(raioDoNucleo * FRACAO_DO_RAIO_DA_ESFERA_ESCURA, 32, 24);
  const materialDaEsfera = new MeshBasicMaterial({ color: 0x000000 });
  const esferaEscura = new Mesh(geometriaDaEsfera, materialDaEsfera);

  const raioInterno = raioDoNucleo * FRACAO_DO_RAIO_INTERNO_DO_DISCO;
  const raioExterno = raioDoNucleo * FRACAO_DO_RAIO_EXTERNO_DO_DISCO;
  const geometriaDoDisco = new RingGeometry(raioInterno, raioExterno, 128, 1);
  const materialDoDisco = new ShaderMaterial({
    uniforms: { tempo: { value: 0 }, raioInterno: { value: raioInterno }, raioExterno: { value: raioExterno } },
    vertexShader: SHADER_DE_VERTICE_PLANO,
    fragmentShader: SHADER_DE_FRAGMENTO_DO_DISCO,
    transparent: true,
    depthWrite: false,
    side: DoubleSide,
    blending: AdditiveBlending,
  });
  const discoDeAcrecao = new Mesh(geometriaDoDisco, materialDoDisco);
  discoDeAcrecao.rotation.set(-Math.PI / 2 + INCLINACAO_DO_DISCO_EM_RADIANOS[0], 0, INCLINACAO_DO_DISCO_EM_RADIANOS[1]);

  const raioDoAnel = raioDoNucleo * FRACAO_DO_RAIO_DO_ANEL_DE_LUZ;
  const metadeDaLarguraDoAnel = raioDoNucleo * FRACAO_DA_LARGURA_DO_ANEL_DE_LUZ;
  const geometriaDoAnel = new RingGeometry(raioDoAnel - metadeDaLarguraDoAnel * 1.5, raioDoAnel + metadeDaLarguraDoAnel * 1.5, 96, 1);
  const materialDoAnel = new ShaderMaterial({
    uniforms: { raioDoCentroDoAnel: { value: raioDoAnel }, metadeDaLarguraDoAnel: { value: metadeDaLarguraDoAnel * 1.5 } },
    vertexShader: SHADER_DE_VERTICE_PLANO,
    fragmentShader: SHADER_DE_FRAGMENTO_DO_ANEL,
    transparent: true,
    depthWrite: false,
    side: DoubleSide,
    blending: AdditiveBlending,
  });
  const anelDeLuz = new Mesh(geometriaDoAnel, materialDoAnel);

  const texturaDoHalo = criarTexturaDeBrilhoRadial("rgba(255, 190, 110, 1)", 0.02);
  const materialDoHalo = new SpriteMaterial({
    map: texturaDoHalo,
    transparent: true,
    opacity: OPACIDADE_DO_HALO,
    depthWrite: false,
    blending: AdditiveBlending,
  });
  const halo = new Sprite(materialDoHalo);
  halo.scale.setScalar(raioDoNucleo * DIAMETRO_DO_HALO_EM_RAIOS_DO_NUCLEO);

  return {
    objetos: [halo, discoDeAcrecao, esferaEscura, anelDeLuz],
    animar(tempoEmSegundos: number, camera: Camera) {
      materialDoDisco.uniforms.tempo.value = tempoEmSegundos * VELOCIDADE_DO_DISCO;
      // O anel é plano: copiar a orientação da câmera faz ele sempre aparecer de frente, como um contorno de luz.
      anelDeLuz.quaternion.copy(camera.quaternion);
    },
    destruir() {
      geometriaDaEsfera.dispose();
      materialDaEsfera.dispose();
      geometriaDoDisco.dispose();
      materialDoDisco.dispose();
      geometriaDoAnel.dispose();
      materialDoAnel.dispose();
      texturaDoHalo.dispose();
      materialDoHalo.dispose();
    },
  };
}
