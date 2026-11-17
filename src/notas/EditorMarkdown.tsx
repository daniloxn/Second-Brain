// Editor de Markdown (CodeMirror). Recebe o texto inicial uma vez: quem troca de nota
// remonta o componente com `key`, então não é preciso sincronizar o texto depois.

import { useEffect, useRef } from "react";
import { EditorState } from "@codemirror/state";
import { EditorView, keymap } from "@codemirror/view";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import { syntaxHighlighting } from "@codemirror/language";
import { markdown } from "@codemirror/lang-markdown";
import { aparenciaDoEditor, destaqueDeSintaxeMarkdown } from "./temaDoEditor";

interface Props {
  textoInicial: string;
  /** Chamado a cada tecla que altera o texto, com o texto completo. */
  aoMudarTexto: (textoAtualizado: string) => void;
  /** Chamado com Ctrl+S, para salvar na hora. */
  aoSalvarAgora: () => void;
}

export function EditorMarkdown({ textoInicial, aoMudarTexto, aoSalvarAgora }: Props) {
  const elementoDoEditor = useRef<HTMLDivElement>(null);
  // Valor fixo do texto na montagem. Mudanças depois vêm do próprio editor.
  const textoNaMontagem = useRef(textoInicial).current;

  // Refs para que os callbacks mais recentes sejam usados sem recriar o editor.
  const aoMudarTextoRef = useRef(aoMudarTexto);
  const aoSalvarAgoraRef = useRef(aoSalvarAgora);
  useEffect(() => {
    aoMudarTextoRef.current = aoMudarTexto;
    aoSalvarAgoraRef.current = aoSalvarAgora;
  });

  useEffect(() => {
    const editor = new EditorView({
      parent: elementoDoEditor.current as HTMLDivElement,
      state: EditorState.create({
        doc: textoNaMontagem,
        extensions: [
          history(),
          keymap.of([
            {
              key: "Mod-s",
              preventDefault: true,
              run: () => {
                aoSalvarAgoraRef.current();
                return true;
              },
            },
            ...defaultKeymap,
            ...historyKeymap,
          ]),
          markdown(),
          syntaxHighlighting(destaqueDeSintaxeMarkdown),
          EditorView.lineWrapping,
          EditorView.updateListener.of((atualizacao) => {
            if (atualizacao.docChanged) {
              aoMudarTextoRef.current(atualizacao.state.doc.toString());
            }
          }),
          aparenciaDoEditor,
        ],
      }),
    });
    editor.focus();
    return () => editor.destroy();
  }, [textoNaMontagem]);

  return <div className="editor" ref={elementoDoEditor} />;
}
