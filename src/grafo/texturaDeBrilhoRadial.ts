// Textura de brilho: uma mancha redonda que vai da cor ao transparente. Serve de halo do buraco
// negro, de nebulosa e de ponto redondo nas estrelas de fundo.

import { CanvasTexture } from "three";

/** Lado da imagem. Pequena basta: o brilho é suave e a GPU amplia. */
const LADO_DA_TEXTURA_EM_PIXELS = 128;

/** Mancha com `cor` (texto CSS) no centro e transparente na borda. `dureza` de 0 a 1: quanto maior, mais o miolo fica forte. */
export function criarTexturaDeBrilhoRadial(cor: string, dureza: number): CanvasTexture {
  const imagem = document.createElement("canvas");
  imagem.width = imagem.height = LADO_DA_TEXTURA_EM_PIXELS;
  const contexto = imagem.getContext("2d");
  if (contexto) {
    const meio = LADO_DA_TEXTURA_EM_PIXELS / 2;
    const gradiente = contexto.createRadialGradient(meio, meio, 0, meio, meio, meio);
    gradiente.addColorStop(0, cor);
    gradiente.addColorStop(Math.min(Math.max(dureza, 0.01), 0.99), cor);
    gradiente.addColorStop(1, "rgba(0, 0, 0, 0)");
    contexto.fillStyle = gradiente;
    contexto.fillRect(0, 0, LADO_DA_TEXTURA_EM_PIXELS, LADO_DA_TEXTURA_EM_PIXELS);
  }
  return new CanvasTexture(imagem);
}
