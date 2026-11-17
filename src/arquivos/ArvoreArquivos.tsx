// Árvore recursiva de pastas e arquivos do explorador. Pastas do primeiro nível
// começam abertas; notas (.md) são clicáveis e abrem na seção Notas.

import { useState } from "react";
import { ChevronDown, ChevronRight, File, FileText, Folder, FolderOpen } from "lucide-react";
import type { NoArvore } from "../cofre/chamadasCofre";

interface Props {
  no: NoArvore;
  /** Profundidade deste item na árvore. Zero é o primeiro nível, que começa aberto. */
  nivel?: number;
  /** Caminho da nota aberta agora, para destacar o item na árvore. */
  caminhoDaNotaAberta?: string | null;
  aoAbrirNota?: (caminhoDaNota: string) => void;
}

/** Verdadeiro se o nome do arquivo termina em `.md`, sem diferenciar maiúsculas. */
function ehArquivoMarkdown(nomeDoArquivo: string): boolean {
  return /\.md$/i.test(nomeDoArquivo);
}

export function ArvoreArquivos({ no, nivel = 0, caminhoDaNotaAberta = null, aoAbrirNota }: Props) {
  const [estaAberta, setEstaAberta] = useState(nivel === 0);
  const recuoEsquerdo = { paddingLeft: 8 + nivel * 14 };

  if (!no.pasta) {
    if (ehArquivoMarkdown(no.nome) && aoAbrirNota) {
      const estaSelecionada = caminhoDaNotaAberta === no.caminho;
      return (
        <button
          type="button"
          className="item nota-item"
          style={recuoEsquerdo}
          title={no.nome}
          aria-current={estaSelecionada ? "true" : undefined}
          onClick={() => aoAbrirNota(no.caminho)}
        >
          <FileText size={14} aria-hidden="true" />
          <span className="nome">{no.nome.replace(/\.md$/i, "")}</span>
        </button>
      );
    }
    // Arquivo que ainda não tem tela própria: aparece, mas não é clicável.
    return (
      <div className="item arquivo" style={recuoEsquerdo} title={no.nome}>
        <File size={14} aria-hidden="true" />
        <span className="nome">{no.nome}</span>
      </div>
    );
  }

  return (
    <div>
      <button
        type="button"
        className="item pasta-titulo"
        style={recuoEsquerdo}
        aria-expanded={estaAberta}
        onClick={() => setEstaAberta(!estaAberta)}
      >
        {estaAberta ? <ChevronDown size={14} aria-hidden="true" /> : <ChevronRight size={14} aria-hidden="true" />}
        {estaAberta ? <FolderOpen size={15} aria-hidden="true" /> : <Folder size={15} aria-hidden="true" />}
        <span className="nome">{no.nome}</span>
      </button>
      {estaAberta &&
        no.filhos.map((filho) => (
          <ArvoreArquivos
            key={filho.caminho}
            no={filho}
            nivel={nivel + 1}
            caminhoDaNotaAberta={caminhoDaNotaAberta}
            aoAbrirNota={aoAbrirNota}
          />
        ))}
    </div>
  );
}
