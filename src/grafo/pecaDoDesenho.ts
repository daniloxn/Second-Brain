// Contrato de cada peça do desenho do universo (estrelas, ligações, buraco negro, órbitas, fundo).
// A cena junta as peças sem saber o que cada uma desenha.

import type { Camera, Object3D } from "three";

export interface PecaDoDesenho {
  /** Objetos para pôr na cena. */
  objetos: Object3D[];
  /** Chamada a cada quadro. `tempoEmSegundos` fica parado quando o usuário prefere menos movimento. */
  animar?(tempoEmSegundos: number, camera: Camera): void;
  /** Libera a memória de vídeo. */
  destruir(): void;
}
