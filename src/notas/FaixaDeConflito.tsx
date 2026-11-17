// Faixa mostrada quando a nota mudou no disco enquanto ela estava aberta (ex.: o agente editou).
// Oferece as duas escolhas. A decisão em si fica no hook; aqui só há os botões.

interface Props {
  /** Descarta o texto digitado e carrega a versão do disco. */
  aoUsarDoDisco: () => void;
  /** Mantém o texto digitado e sobrescreve a versão do disco. */
  aoManterMinha: () => void;
}

/** Aviso de conflito com as duas opções. Nada é apagado sem cópia guardada antes. */
export function FaixaDeConflito({ aoUsarDoDisco, aoManterMinha }: Props) {
  return (
    <div role="alert" className="faixa-conflito">
      <p>
        Esta nota foi alterada por fora (por exemplo, pelo agente) enquanto você editava.
        Uma cópia da versão que não ficar fica guardada em <code>_sistema/conflitos</code>.
      </p>
      <div className="faixa-conflito-acoes">
        <button type="button" className="botao" onClick={aoUsarDoDisco}>
          Usar a versão do disco
        </button>
        <button type="button" className="botao botao-primario" onClick={aoManterMinha}>
          Manter a minha
        </button>
      </div>
    </div>
  );
}
