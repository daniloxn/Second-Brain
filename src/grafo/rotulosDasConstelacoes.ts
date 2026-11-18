// Rótulos de texto flutuando no universo: o nome e a quantidade de cada constelação e o botão "Home"
// no buraco negro. São elementos HTML presos a pontos 3D (CSS2DObject), então o texto fica nítido
// e acompanha a câmera. Os textos vão por `textContent`, porque nome de pasta é digitado pelo usuário.

import { Group, type Scene } from "three";
import { CSS2DObject } from "three/examples/jsm/renderers/CSS2DRenderer.js";
import type { ConstelacaoDoGrafo } from "./constelacoesDoGrafo";

/** Quanto o rótulo de uma constelação fica acima do centro dela, para não cobrir as estrelas. */
const ALTURA_DO_ROTULO_ACIMA_DA_CONSTELACAO = 12;
/** Posição do botão "Home": logo abaixo do buraco negro. */
const POSICAO_DO_BOTAO_HOME: [number, number, number] = [0, -20, 0];

export interface RotulosDasConstelacoes {
  /** Troca os rótulos das constelações pelos da lista. */
  aplicar(constelacoes: ConstelacaoDoGrafo[]): void;
  destruir(): void;
}

/** Texto "1 nota" ou "26 notas". */
function textoDaQuantidade(quantidade: number): string {
  return quantidade === 1 ? "1 nota" : `${quantidade} notas`;
}

function criarElementoDoRotulo(constelacao: ConstelacaoDoGrafo): HTMLDivElement {
  const elemento = document.createElement("div");
  elemento.className = "grafo-rotulo";
  const nome = document.createElement("strong");
  nome.textContent = constelacao.rotulo;
  const quantidade = document.createElement("span");
  quantidade.textContent = textoDaQuantidade(constelacao.quantidade);
  elemento.append(nome, quantidade);
  return elemento;
}

export function criarRotulosDasConstelacoes(cena: Scene, aoClicarEmHome: () => void): RotulosDasConstelacoes {
  const grupoDosRotulos = new Group();
  cena.add(grupoDosRotulos);

  const botaoHome = document.createElement("button");
  botaoHome.type = "button";
  botaoHome.className = "grafo-home";
  botaoHome.textContent = "Home";
  botaoHome.setAttribute("aria-label", "Voltar para a vista inicial do universo");
  botaoHome.addEventListener("click", aoClicarEmHome);
  const objetoDoHome = new CSS2DObject(botaoHome);
  objetoDoHome.position.set(...POSICAO_DO_BOTAO_HOME);
  cena.add(objetoDoHome);

  /** Tira do grupo os rótulos antigos, inclusive o elemento HTML de cada um. */
  function removerRotulosAtuais() {
    for (const rotulo of [...grupoDosRotulos.children]) {
      grupoDosRotulos.remove(rotulo);
      (rotulo as CSS2DObject).element.remove();
    }
  }

  return {
    aplicar(constelacoes) {
      removerRotulosAtuais();
      for (const constelacao of constelacoes) {
        const rotulo = new CSS2DObject(criarElementoDoRotulo(constelacao));
        rotulo.position.set(constelacao.centro[0], constelacao.centro[1] + ALTURA_DO_ROTULO_ACIMA_DA_CONSTELACAO, constelacao.centro[2]);
        grupoDosRotulos.add(rotulo);
      }
    },
    destruir() {
      removerRotulosAtuais();
      cena.remove(grupoDosRotulos);
      botaoHome.removeEventListener("click", aoClicarEmHome);
      cena.remove(objetoDoHome);
      botaoHome.remove();
    },
  };
}
