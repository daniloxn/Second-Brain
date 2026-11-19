// Regras da tela de configurações: a busca nas opções e a tradução de uma tecla pressionada
// no texto de atalho. Funções puras, sem componente nem chamada ao backend.

/** Campos de uma tecla pressionada que importam para montar o atalho. */
export interface TeclaPressionada {
  key: string;
  ctrlKey: boolean;
  altKey: boolean;
  shiftKey: boolean;
  metaKey: boolean;
}

/** Teclas que só mudam modificador: sozinhas não formam atalho. */
const MODIFICADORES_SOZINHOS = ["Control", "Alt", "Shift", "Meta", "OS"];

/** Nomes que o plugin de atalhos usa para teclas que não são letras nem números. */
const NOMES_DAS_TECLAS_ESPECIAIS: Record<string, string> = {
  " ": "Space",
  Enter: "Enter",
  Tab: "Tab",
  Backspace: "Backspace",
  Delete: "Delete",
  Home: "Home",
  End: "End",
  PageUp: "PageUp",
  PageDown: "PageDown",
  ArrowUp: "Up",
  ArrowDown: "Down",
  ArrowLeft: "Left",
  ArrowRight: "Right",
};

/**
 * Texto do atalho para a tecla pressionada (ex.: `Ctrl+Alt+M`). Devolve null quando a tecla
 * é só um modificador ou quando não há modificador, porque sem ele a tecla seria capturada em qualquer programa.
 */
export function combinacaoDoTeclado(tecla: TeclaPressionada): string | null {
  if (MODIFICADORES_SOZINHOS.includes(tecla.key)) return null;
  const modificadores: string[] = [];
  if (tecla.ctrlKey) modificadores.push("Ctrl");
  if (tecla.altKey) modificadores.push("Alt");
  if (tecla.shiftKey) modificadores.push("Shift");
  if (tecla.metaKey) modificadores.push("Super");
  if (modificadores.length === 0) return null;
  const nomeDaTecla = NOMES_DAS_TECLAS_ESPECIAIS[tecla.key] ?? (tecla.key.length === 1 ? tecla.key.toUpperCase() : tecla.key);
  return [...modificadores, nomeDaTecla].join("+");
}

/** Texto sem acento e em minúsculas, para a busca não depender de maiúsculas ou acentos. */
function normalizarParaBusca(texto: string): string {
  return texto.normalize("NFD").replace(/[̀-ͯ]/g, "").toLowerCase();
}

/**
 * Diz se a opção combina com a busca: cada palavra digitada precisa aparecer no título
 * ou nas palavras-chave da opção. Busca vazia mostra tudo.
 */
export function opcaoCombinaComABusca(textoDaOpcao: string, termoDigitado: string): boolean {
  const palavrasDaBusca = normalizarParaBusca(termoDigitado).split(/\s+/).filter((palavra) => palavra !== "");
  const textoNormalizado = normalizarParaBusca(textoDaOpcao);
  return palavrasDaBusca.every((palavra) => textoNormalizado.includes(palavra));
}
