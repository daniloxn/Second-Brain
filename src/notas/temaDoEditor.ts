// Aparência do editor de Markdown: cores do destaque de sintaxe e tamanhos do texto.
// Usa as variáveis de cor do tema, para que trocar a paleta não exija mexer aqui.

import { HighlightStyle } from "@codemirror/language";
import { EditorView } from "@codemirror/view";
import { tags } from "@lezer/highlight";

/** Cores de cada parte da sintaxe Markdown (títulos, negrito, links e código). */
export const destaqueDeSintaxeMarkdown = HighlightStyle.define([
  { tag: tags.heading, color: "var(--color-foreground)", fontWeight: "600" },
  { tag: tags.strong, fontWeight: "600" },
  { tag: tags.emphasis, fontStyle: "italic" },
  { tag: tags.link, color: "var(--color-primary)" },
  { tag: tags.url, color: "var(--color-muted-foreground)" },
  { tag: tags.monospace, color: "var(--color-accent)" },
  { tag: [tags.quote], color: "var(--color-muted-foreground)" },
  {
    tag: [tags.processingInstruction, tags.meta, tags.contentSeparator],
    color: "var(--color-muted-foreground)",
  },
]);

/** Layout do editor: fundo transparente, fonte do app, largura de leitura e sem números de linha. */
export const aparenciaDoEditor = EditorView.theme({
  "&": {
    height: "100%",
    color: "var(--color-foreground)",
    backgroundColor: "transparent",
  },
  "&.cm-focused": { outline: "none" },
  ".cm-scroller": {
    fontFamily: "var(--font-sans)",
    fontSize: "15px",
    lineHeight: "1.75",
    padding: "24px 32px 64px",
  },
  ".cm-content": {
    caretColor: "var(--color-primary)",
    maxWidth: "720px",
    margin: "0 auto",
  },
  ".cm-line": { padding: "0" },
  ".cm-activeLine": { backgroundColor: "transparent" },
  ".cm-selectionBackground, &.cm-focused .cm-selectionBackground, ::selection": {
    backgroundColor: "rgba(45, 212, 191, 0.22)",
  },
  ".cm-gutters": { display: "none" },
});
