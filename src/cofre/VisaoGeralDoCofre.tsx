// Visão geral da seção Arquivos: cartões das pastas de primeiro nível, criação de pastas
// dentro do cofre e a lista dos arquivos soltos na raiz.

import { Folder } from "lucide-react";
import type { NoArvore } from "./chamadasCofre";
import { FormularioNovaPastaNoCofre } from "./FormularioNovaPastaNoCofre";

interface Props {
  arvoreDoCofre: NoArvore;
  /** Cria uma pasta dentro do cofre. Lança erro com a mensagem a mostrar. */
  aoCriarPasta: (caminhoRelativo: string) => Promise<void>;
}

export function VisaoGeralDoCofre({ arvoreDoCofre, aoCriarPasta }: Props) {
  const pastasDaRaiz = arvoreDoCofre.filhos.filter((item) => item.pasta);
  const arquivosDaRaiz = arvoreDoCofre.filhos.filter((item) => !item.pasta);

  return (
    <div className="visao">
      <FormularioNovaPastaNoCofre aoCriarPasta={aoCriarPasta} />

      <h2 className="rotulo">Pastas</h2>
      <div className="grade-pastas">
        {pastasDaRaiz.map((pasta) => (
          <div key={pasta.caminho} className="cartao">
            <Folder size={18} aria-hidden="true" className="marca-suave" />
            <strong>{pasta.nome}</strong>
            <span className="suave">
              {pasta.filhos.length} {pasta.filhos.length === 1 ? "item" : "itens"}
            </span>
          </div>
        ))}
      </div>

      {arquivosDaRaiz.length > 0 && (
        <>
          <h2 className="rotulo">Na raiz</h2>
          <ul className="lista-simples">
            {arquivosDaRaiz.map((arquivo) => (
              <li key={arquivo.caminho}>{arquivo.nome}</li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
}
