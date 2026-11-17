// Leitura da nota: renderiza o Markdown com tabelas e listas. Não aceita HTML bruto,
// e os wikilinks viram botões que abrem a nota correspondente.

import { useMemo } from "react";
import ReactMarkdown, { defaultUrlTransform } from "react-markdown";
import remarkGfm from "remark-gfm";
import { alvoDoHrefWiki, converterWikilinksParaLinks, ESQUEMA_DO_WIKILINK } from "./wikilinks";

interface Props {
  textoDaNota: string;
  /** Chamado com o alvo do wikilink clicado (ex.: `Projeto X` ou `pasta/nota`). */
  aoClicarNoWikilink: (alvoDoLink: string) => void;
}

export function LeituraMarkdown({ textoDaNota, aoClicarNoWikilink }: Props) {
  const textoComLinksInternos = useMemo(() => converterWikilinksParaLinks(textoDaNota), [textoDaNota]);

  return (
    <div className="prosa">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        // Mantém os links `wiki:`, que o padrão do react-markdown removeria por não serem http.
        urlTransform={(endereco) =>
          endereco.startsWith(ESQUEMA_DO_WIKILINK) ? endereco : defaultUrlTransform(endereco)
        }
        components={{
          a({ href, children }) {
            if (href?.startsWith(ESQUEMA_DO_WIKILINK)) {
              return (
                <button
                  type="button"
                  className="wiki"
                  onClick={() => aoClicarNoWikilink(alvoDoHrefWiki(href))}
                >
                  {children}
                </button>
              );
            }
            return (
              <a href={href} target="_blank" rel="noreferrer">
                {children}
              </a>
            );
          },
        }}
      >
        {textoComLinksInternos}
      </ReactMarkdown>
    </div>
  );
}
