// Conversão de wikilinks `[[...]]` em links Markdown internos. Função pura.

/** Esquema usado nos links internos. O render trata qualquer link com esse prefixo como wikilink. */
export const ESQUEMA_DO_WIKILINK = "wiki:";

/**
 * Troca cada `[[nota]]` e `[[nota|rótulo]]` por um link Markdown com o esquema `wiki:`.
 * O alvo vai codificado para aguentar espaços e acentos.
 */
export function converterWikilinksParaLinks(textoDaNota: string): string {
  return textoDaNota.replace(/\[\[([^\]\n]+)\]\]/g, (textoOriginal: string, conteudoDoLink: string) => {
    const [alvoBruto, rotuloOpcional] = conteudoDoLink.split("|");
    const alvoSemSecao = alvoBruto.split("#")[0].trim();
    // Link vazio como `[[]]` fica como está, sem virar um link quebrado.
    if (!alvoSemSecao) return textoOriginal;
    const rotuloVisivel = (rotuloOpcional ?? alvoSemSecao).trim();
    return `[${rotuloVisivel}](${ESQUEMA_DO_WIKILINK}${encodeURIComponent(alvoSemSecao)})`;
  });
}

/** Alvo original de um link `wiki:` a partir do href já convertido. */
export function alvoDoHrefWiki(hrefWiki: string): string {
  return decodeURIComponent(hrefWiki.slice(ESQUEMA_DO_WIKILINK.length));
}
