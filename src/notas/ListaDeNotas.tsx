// Lista de todas as notas do cofre, com o botão para criar uma nota nova.

import { useState } from "react";
import { FileText, Plus } from "lucide-react";
import type { NotaIndice } from "./chamadasNotas";
import { caminhoRelativoAoCofre } from "./nomesDeNota";
import { FormularioNovaNota } from "./FormularioNovaNota";

interface Props {
  notas: NotaIndice[];
  pastaDoCofre: string;
  aoAbrirNota: (caminhoDaNota: string) => void;
  /** Cria a nota e abre. Deve lançar erro com a mensagem a mostrar. */
  aoCriarNota: (nomeDigitado: string) => Promise<void>;
}

export function ListaDeNotas({ notas, pastaDoCofre, aoAbrirNota, aoCriarNota }: Props) {
  const [estaCriandoNota, setEstaCriandoNota] = useState(false);

  /** Pergunta ao formulário se a criação deu certo: se sim, fecha ele. */
  async function criarEFechar(nomeDigitado: string) {
    await aoCriarNota(nomeDigitado);
    setEstaCriandoNota(false);
  }

  const rotuloDaContagem = notas.length === 1 ? "1 nota" : `${notas.length} notas`;

  return (
    <div className="lista-notas">
      <div className="linha-topo">
        <h2 className="rotulo">{rotuloDaContagem}</h2>
        {!estaCriandoNota && (
          <button type="button" className="botao botao-primario" onClick={() => setEstaCriandoNota(true)}>
            <Plus size={15} aria-hidden="true" />
            Nova nota
          </button>
        )}
      </div>

      {estaCriandoNota && (
        <FormularioNovaNota aoEnviarNome={criarEFechar} aoCancelar={() => setEstaCriandoNota(false)} />
      )}

      {notas.length === 0 ? (
        <p className="suave">Nenhuma nota ainda. Crie a primeira para começar seu cérebro.</p>
      ) : (
        <ul className="itens-notas">
          {notas.map((nota) => (
            <li key={nota.caminho}>
              <button type="button" className="item-nota" onClick={() => aoAbrirNota(nota.caminho)}>
                <FileText size={16} aria-hidden="true" />
                <span className="nome">{nota.nome}</span>
                <span className="caminho">{caminhoRelativoAoCofre(nota.caminho, pastaDoCofre)}</span>
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
