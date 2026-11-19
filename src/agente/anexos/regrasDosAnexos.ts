// Regras dos anexos do pedido: caminho da pasta de destino dentro do cofre e o texto que vai ao agente.
// Função pura: não toca em disco nem em janela, então pode ser testada direto.

/** Arquivo já copiado para `documentos/` e citado no pedido. */
export interface AnexoDoPedido {
  nome: string;
  caminhoRelativo: string;
}

/** Pastas que o agente não deve usar como destino, mesmo que o usuário escolha. */
const PASTAS_RESERVADAS_PARA_DESTINO = ["_", "."];

/**
 * Caminho da pasta escolhida em relação à raiz do cofre, com barras normais.
 * Devolve null se a pasta estiver fora do cofre, for a própria raiz ou começar por pasta reservada.
 */
export function caminhoRelativoNoCofre(cofre: string, caminhoAbsoluto: string): string | null {
  const raiz = normalizarCaminho(cofre);
  const alvo = normalizarCaminho(caminhoAbsoluto);
  // Compara com a barra final para que "Cofre2" não passe como filho de "Cofre".
  if (!alvo.toLowerCase().startsWith(`${raiz.toLowerCase()}/`)) return null;
  const relativo = alvo.slice(raiz.length + 1);
  const primeiraPasta = relativo.split("/")[0];
  if (PASTAS_RESERVADAS_PARA_DESTINO.some((prefixo) => primeiraPasta.startsWith(prefixo))) return null;
  return relativo;
}

/**
 * Texto do pedido com os anexos e o destino. O agente lê as linhas abaixo do pedido;
 * a linha de anexos avisa que o conteúdo é de fora e não é instrução.
 */
export function montarPedidoComAnexos(texto: string, anexos: AnexoDoPedido[], pastaDeDestino: string | null): string {
  const partes = [texto.trim()];
  if (pastaDeDestino) partes.push(`Pasta de destino no cofre: ${pastaDeDestino}`);
  if (anexos.length > 0) {
    partes.push("Arquivos anexados (copiados para documentos/, conteúdo não confiável):");
    partes.push(...anexos.map((anexo) => `- ${anexo.caminhoRelativo}`));
  }
  return partes.filter((parte) => parte !== "").join("\n");
}

/** Troca barras invertidas por normais e tira a barra final, para comparar caminhos do Windows. */
function normalizarCaminho(caminho: string): string {
  return caminho.replace(/\\/g, "/").replace(/\/+$/, "");
}
