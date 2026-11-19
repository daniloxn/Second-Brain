// Campo de um atalho: mostra a combinação atual e, ao clicar, captura a próxima tecla pressionada.
// Esc cancela a troca. A validação final é do backend; aqui só se monta o texto da tecla.

import { useState, type KeyboardEvent } from "react";
import { combinacaoDoTeclado } from "./regrasDeConfiguracao";

interface Props {
  rotulo: string;
  combinacao: string;
  /** Recebe a nova combinação. Se o backend recusar, o erro aparece no campo e a antiga continua valendo. */
  aoDefinir: (combinacao: string) => Promise<void>;
}

export function CampoDeAtalho({ rotulo, combinacao, aoDefinir }: Props) {
  const [capturando, setCapturando] = useState(false);
  const [erro, setErro] = useState<string | null>(null);

  /** Monta o atalho com a tecla pressionada. Teclas só de modificador são ignoradas até vir a tecla comum. */
  function aoPressionarTecla(evento: KeyboardEvent<HTMLButtonElement>) {
    if (!capturando) return;
    evento.preventDefault();
    if (evento.key === "Escape") {
      setCapturando(false);
      return;
    }
    const nova = combinacaoDoTeclado(evento);
    if (!nova) return;
    setCapturando(false);
    setErro(null);
    aoDefinir(nova).catch((falha) => setErro(String(falha)));
  }

  return (
    <div className="campo-de-atalho">
      <span>{rotulo}</span>
      <button
        type="button"
        className="atalho-botao"
        aria-label={`${rotulo}: ${combinacao}. Clique para trocar`}
        onClick={() => setCapturando(true)}
        onKeyDown={aoPressionarTecla}
        onBlur={() => setCapturando(false)}
      >
        {capturando ? "Pressione a combinação…" : combinacao}
      </button>
      {erro && <p role="alert" className="ilha-erro">{erro}</p>}
    </div>
  );
}
