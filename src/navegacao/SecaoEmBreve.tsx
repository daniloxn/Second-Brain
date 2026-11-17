// Tela de aviso para seções que ainda não foram construídas.
// Mostra a fase em que a seção chega, para não dar a impressão de que já funciona.

import type { SecaoDoMenu } from "./secoes";

interface Props {
  secao: SecaoDoMenu;
}

export function SecaoEmBreve({ secao }: Props) {
  const Icone = secao.icone;
  return (
    <div className="vazio">
      <Icone size={28} aria-hidden="true" className="marca-suave" />
      <h2 className="titulo-vazio">{secao.titulo}</h2>
      <p className="suave">{secao.descricaoDoAviso}</p>
      <span className="etiqueta">Disponível na {secao.faseDeEntrega}</span>
    </div>
  );
}
