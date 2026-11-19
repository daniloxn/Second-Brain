// Linha de anexos do pedido: anexar arquivos, escolher a pasta de destino e ver o que já foi escolhido.
// A cópia e as janelas ficam em `chamadasAnexos.ts`; aqui só está a tela.

import { useState } from "react";
import { FolderOpen, Paperclip, X } from "lucide-react";
import { caminhoRelativoNoCofre, type AnexoDoPedido } from "./regrasDosAnexos";
import { escolherEImportarAnexos, escolherPastaDeDestino } from "./chamadasAnexos";

interface Props {
  cofre: string;
  anexos: AnexoDoPedido[];
  pastaDeDestino: string | null;
  desativado: boolean;
  aoAnexar: (novos: AnexoDoPedido[]) => void;
  aoRemoverAnexo: (caminhoRelativo: string) => void;
  aoDefinirPastaDeDestino: (pasta: string | null) => void;
}

export function AnexosDoPedido({ cofre, anexos, pastaDeDestino, desativado, aoAnexar, aoRemoverAnexo, aoDefinirPastaDeDestino }: Props) {
  const [erro, setErro] = useState<string | null>(null);

  /** Copia os arquivos escolhidos para `documentos/` e os junta aos anexos do pedido. */
  async function anexarArquivos() {
    setErro(null);
    try {
      aoAnexar(await escolherEImportarAnexos());
    } catch (falha) {
      setErro(String(falha));
    }
  }

  /** Guarda a pasta de destino como caminho relativo ao cofre. Pasta fora do cofre é recusada. */
  async function escolherDestino() {
    setErro(null);
    try {
      const absoluta = await escolherPastaDeDestino(cofre);
      if (!absoluta) return;
      const relativa = caminhoRelativoNoCofre(cofre, absoluta);
      if (!relativa) {
        setErro("Escolha uma pasta dentro do cofre, fora de _sistema e _segredos.");
        return;
      }
      aoDefinirPastaDeDestino(relativa);
    } catch (falha) {
      setErro(String(falha));
    }
  }

  return (
    <div className="anexos-do-pedido">
      <div className="anexos-do-pedido-botoes">
        <button type="button" className="anexo-botao" onClick={() => void anexarArquivos()} disabled={desativado}>
          <Paperclip size={13} aria-hidden="true" /> Anexar
        </button>
        <button type="button" className="anexo-botao" onClick={() => void escolherDestino()} disabled={desativado}>
          <FolderOpen size={13} aria-hidden="true" /> Pasta de destino
        </button>
      </div>
      {(anexos.length > 0 || pastaDeDestino) && (
        <ul className="anexos-do-pedido-lista" aria-label="Anexos e destino do pedido">
          {pastaDeDestino && (
            <li className="anexo-chip">
              Destino: {pastaDeDestino}
              <button type="button" aria-label="Remover pasta de destino" onClick={() => aoDefinirPastaDeDestino(null)}><X size={11} /></button>
            </li>
          )}
          {anexos.map((anexo) => (
            <li key={anexo.caminhoRelativo} className="anexo-chip">
              {anexo.nome}
              <button type="button" aria-label={`Remover ${anexo.nome}`} onClick={() => aoRemoverAnexo(anexo.caminhoRelativo)}><X size={11} /></button>
            </li>
          ))}
        </ul>
      )}
      {erro && <p role="alert" className="ilha-erro">{erro}</p>}
    </div>
  );
}
