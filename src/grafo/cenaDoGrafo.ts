// Cena 3D do universo: renderizador, câmera, órbita com o mouse, rótulos, passar o mouse e clicar nas estrelas.
// Não é React: o componente QuadroDoGrafo cria e destrói a cena. O que se desenha vem de desenhoDoGrafo.ts.

import { PerspectiveCamera, Raycaster, Scene, Vector2, Vector3, WebGLRenderer } from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { CSS2DRenderer } from "three/examples/jsm/renderers/CSS2DRenderer.js";
import { montarConstelacoes } from "./constelacoesDoGrafo";
import type { GrafoDoCofre } from "./contratoGrafo";
import { criarDesenhoDoGrafo } from "./desenhoDoGrafo";
import type { VisaoDoGrafo } from "./filtrosDoGrafo";
import { criarRotulosDasConstelacoes } from "./rotulosDasConstelacoes";

/** Quem usa a cena é avisado do que acontece com o mouse. */
export interface OuvintesDaCena {
  /** Estrela sob o mouse (posição em `grafo.nos`) ou null. `x` e `y` são pixels dentro da cena. */
  aoPassarSobreNo(posicaoDoNo: number | null, x: number, y: number): void;
  aoClicarNoNo(posicaoDoNo: number): void;
}

export interface CenaDoGrafo {
  aplicarVisao(visao: VisaoDoGrafo): void;
  aproximar(): void;
  afastar(): void;
  /** Volta a câmera para a vista inicial. */
  reiniciarVista(): void;
  /** Pausa ou retoma o movimento (rotação da câmera, disco, cometas, cintilação). Devolve se ficou pausado. */
  alternarPausa(): boolean;
  estaPausado(): boolean;
  /** Para o desenho, solta os eventos e libera a memória de vídeo. */
  destruir(): void;
}

/** A câmera começa de cima, na diagonal: dessa vista o disco da galáxia aparece como uma elipse. */
const DIRECAO_INICIAL_DA_CAMERA = new Vector3(0.12, 0.62, 0.78).normalize();
/** O disco inteiro, com folga, deve caber na largura da tela. */
const FOLGA_DO_ENQUADRAMENTO = 1.15;
const DISTANCIA_MINIMA_INICIAL_DA_CAMERA = 230;
const DISTANCIA_MINIMA_DA_CAMERA = 30;
const DISTANCIA_MAXIMA_DA_CAMERA = 700;
const VELOCIDADE_DA_ROTACAO_AUTOMATICA = 0.45;
/** Cada toque em + ou - muda a distância da câmera por este fator. */
const FATOR_DE_ZOOM_DOS_BOTOES = 0.78;
/** Quanto o raio do mouse "engorda" ao procurar estrelas, em unidades do grafo. */
const MARGEM_DE_ACERTO_DAS_ESTRELAS = 3.5;
/** Quanto o mouse pode andar entre apertar e soltar para ainda contar como clique (e não arrasto). */
const TOLERANCIA_DE_CLIQUE_EM_PIXELS = 5;

export function criarCenaDoGrafo(recipiente: HTMLElement, grafo: GrafoDoCofre, ouvintes: OuvintesDaCena): CenaDoGrafo {
  const renderizador = new WebGLRenderer({ antialias: true, alpha: true });
  renderizador.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  const tela = renderizador.domElement;
  recipiente.appendChild(tela);
  const renderizadorDeRotulos = new CSS2DRenderer();
  renderizadorDeRotulos.domElement.className = "grafo-rotulos";
  recipiente.appendChild(renderizadorDeRotulos.domElement);

  const cena = new Scene();
  const camera = new PerspectiveCamera(50, 1, 1, 6000);
  const desenho = criarDesenhoDoGrafo(grafo);
  cena.add(...desenho.objetos);

  /** Ajusta o tamanho do desenho, dos rótulos e da câmera ao tamanho do recipiente. */
  function ajustarAoTamanhoDoRecipiente() {
    const largura = Math.max(recipiente.clientWidth, 1);
    const altura = Math.max(recipiente.clientHeight, 1);
    renderizador.setSize(largura, altura);
    renderizadorDeRotulos.setSize(largura, altura);
    camera.aspect = largura / altura;
    camera.updateProjectionMatrix();
    desenho.ajustarEscalaDaTela(altura, camera.fov);
  }
  const observadorDeTamanho = new ResizeObserver(ajustarAoTamanhoDoRecipiente);
  observadorDeTamanho.observe(recipiente);
  ajustarAoTamanhoDoRecipiente();

  // Distância inicial: longe o bastante para o disco caber na largura. Em áreas estreitas o campo de
  // visão horizontal é menor, então a câmera recua mais.
  const tangenteDaMetadeDoCampoVertical = Math.tan((camera.fov * Math.PI) / 360);
  const distanciaParaCaberALargura = (grafo.volume.raio * FOLGA_DO_ENQUADRAMENTO) / (tangenteDaMetadeDoCampoVertical * camera.aspect);
  const posicaoInicialDaCamera = DIRECAO_INICIAL_DA_CAMERA.clone().multiplyScalar(
    Math.max(DISTANCIA_MINIMA_INICIAL_DA_CAMERA, distanciaParaCaberALargura),
  );
  camera.position.copy(posicaoInicialDaCamera);

  const controles = new OrbitControls(camera, tela);
  controles.enableDamping = true;
  controles.minDistance = DISTANCIA_MINIMA_DA_CAMERA;
  controles.maxDistance = DISTANCIA_MAXIMA_DA_CAMERA;
  controles.autoRotateSpeed = VELOCIDADE_DA_ROTACAO_AUTOMATICA;
  // Quem prefere menos movimento começa pausado. Ao pegar a cena com o mouse, a rotação automática para.
  const prefereMenosMovimento = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  let pausado = prefereMenosMovimento;
  controles.autoRotate = !pausado;
  controles.addEventListener("start", () => {
    controles.autoRotate = false;
  });

  /** Aproxima ou afasta a câmera do centro, respeitando as distâncias mínima e máxima. */
  function mudarDistanciaDaCamera(fator: number) {
    const deslocamento = camera.position.clone().sub(controles.target);
    const novaDistancia = Math.min(Math.max(deslocamento.length() * fator, DISTANCIA_MINIMA_DA_CAMERA), DISTANCIA_MAXIMA_DA_CAMERA);
    camera.position.copy(controles.target).add(deslocamento.setLength(novaDistancia));
  }

  function reiniciarVista() {
    controles.target.set(0, 0, 0);
    camera.position.copy(posicaoInicialDaCamera);
    controles.update();
  }

  const rotulos = criarRotulosDasConstelacoes(cena, reiniciarVista);

  let visaoAtual: VisaoDoGrafo | null = null;
  let tempoDaAnimacao = 0;
  let instanteDoQuadroAnterior = performance.now();
  let idDoQuadro = 0;
  function desenharQuadro() {
    idDoQuadro = requestAnimationFrame(desenharQuadro);
    const agora = performance.now();
    if (!pausado) tempoDaAnimacao += (agora - instanteDoQuadroAnterior) / 1000;
    instanteDoQuadroAnterior = agora;
    controles.update();
    desenho.animar(tempoDaAnimacao, camera);
    renderizador.render(cena, camera);
    renderizadorDeRotulos.render(cena, camera);
  }
  desenharQuadro();

  const raio = new Raycaster();
  raio.params.Points.threshold = MARGEM_DE_ACERTO_DAS_ESTRELAS;
  const pontoNaTela = new Vector2();

  /** Posição da estrela visível mais perto da câmera sob o mouse, ou null. */
  function noSobOMouse(evento: PointerEvent): number | null {
    const retangulo = tela.getBoundingClientRect();
    pontoNaTela.set(
      ((evento.clientX - retangulo.left) / retangulo.width) * 2 - 1,
      -((evento.clientY - retangulo.top) / retangulo.height) * 2 + 1,
    );
    raio.setFromCamera(pontoNaTela, camera);
    const acertos = raio.intersectObject(desenho.pontosDasEstrelas);
    // Estrelas escondidas têm tamanho 0, mas ainda existem para o raio: só vale a visível.
    const acerto = acertos.find((candidato) => candidato.index !== undefined && visaoAtual?.nosVisiveis[candidato.index]);
    return acerto?.index ?? null;
  }

  function aoMoverOMouse(evento: PointerEvent) {
    const posicaoDoNo = noSobOMouse(evento);
    tela.style.cursor = posicaoDoNo === null ? "grab" : "pointer";
    const retangulo = tela.getBoundingClientRect();
    ouvintes.aoPassarSobreNo(posicaoDoNo, evento.clientX - retangulo.left, evento.clientY - retangulo.top);
  }
  function aoSairDoMouse() {
    ouvintes.aoPassarSobreNo(null, 0, 0);
  }

  let ondeApertou: { x: number; y: number } | null = null;
  function aoApertarOMouse(evento: PointerEvent) {
    ondeApertou = { x: evento.clientX, y: evento.clientY };
  }
  function aoSoltarOMouse(evento: PointerEvent) {
    if (!ondeApertou) return;
    const distanciaPercorrida = Math.hypot(evento.clientX - ondeApertou.x, evento.clientY - ondeApertou.y);
    ondeApertou = null;
    if (distanciaPercorrida > TOLERANCIA_DE_CLIQUE_EM_PIXELS) return;
    const posicaoDoNo = noSobOMouse(evento);
    if (posicaoDoNo !== null) ouvintes.aoClicarNoNo(posicaoDoNo);
  }

  tela.addEventListener("pointermove", aoMoverOMouse);
  tela.addEventListener("pointerleave", aoSairDoMouse);
  tela.addEventListener("pointerdown", aoApertarOMouse);
  tela.addEventListener("pointerup", aoSoltarOMouse);

  return {
    aplicarVisao(visao) {
      visaoAtual = visao;
      desenho.aplicarVisao(visao);
      rotulos.aplicar(montarConstelacoes(grafo, visao));
    },
    aproximar: () => mudarDistanciaDaCamera(FATOR_DE_ZOOM_DOS_BOTOES),
    afastar: () => mudarDistanciaDaCamera(1 / FATOR_DE_ZOOM_DOS_BOTOES),
    reiniciarVista,
    alternarPausa() {
      pausado = !pausado;
      controles.autoRotate = !pausado;
      return pausado;
    },
    estaPausado: () => pausado,
    destruir() {
      cancelAnimationFrame(idDoQuadro);
      observadorDeTamanho.disconnect();
      tela.removeEventListener("pointermove", aoMoverOMouse);
      tela.removeEventListener("pointerleave", aoSairDoMouse);
      tela.removeEventListener("pointerdown", aoApertarOMouse);
      tela.removeEventListener("pointerup", aoSoltarOMouse);
      controles.dispose();
      rotulos.destruir();
      desenho.destruir();
      renderizador.dispose();
      tela.remove();
      renderizadorDeRotulos.domElement.remove();
    },
  };
}
