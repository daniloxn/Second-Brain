// Textos e regras de exibição do agente: rótulos das mudanças, resumo do pedido e de quando
// ele aconteceu. Funções puras, sem tela e sem chamadas ao backend.

import type { AquecimentoDoAgente, MudancaNoCofre, RegistroDoAgente, TipoDeMudanca } from "./contratoAgente";

const ROTULO_DO_TIPO_DE_MUDANCA: Record<TipoDeMudanca, string> = {
  criado: "Nota criada",
  alterado: "Nota alterada",
  apagado: "Nota apagada",
  pastaCriada: "Pasta criada",
  pastaApagada: "Pasta apagada",
};

/** Texto curto para o tipo de uma mudança, como "Nota criada". */
export function rotuloDoTipoDeMudanca(tipo: TipoDeMudanca): string {
  return ROTULO_DO_TIPO_DE_MUDANCA[tipo];
}

/** Resumo das mudanças de um pedido: "Nenhuma mudança no cofre" ou "2 mudanças no cofre". */
export function resumoDasMudancas(mudancas: MudancaNoCofre[]): string {
  if (mudancas.length === 0) return "Nenhuma mudança no cofre";
  return mudancas.length === 1 ? "1 mudança no cofre" : `${mudancas.length} mudanças no cofre`;
}

/** Um registro pode ser desfeito se ainda não foi desfeito e se mudou alguma coisa. */
export function podeDesfazerRegistro(registro: RegistroDoAgente): boolean {
  return !registro.desfeito && registro.mudancas.length > 0;
}

/** Data curta a partir de `AAAA-MM-DD HH:MM:SS`, no formato "17/11 10:20". Se não bater, devolve o texto original. */
export function formatarQuandoDoRegistro(quando: string): string {
  const casamento = /^(\d{4})-(\d{2})-(\d{2}) (\d{2}:\d{2})/.exec(quando);
  if (!casamento) return quando;
  const [, , mes, dia, hora] = casamento;
  return `${dia}/${mes} ${hora}`;
}

/** Linha de situação da tela: como está a confirmação de que o agente responde. */
export function textoDaSituacaoDoAquecimento(
  aquecimento: AquecimentoDoAgente | null,
  nomeDoProvedor: string,
): string {
  if (aquecimento === null || aquecimento.situacao === "aguardando" || aquecimento.situacao === "aquecendo") {
    return `Preparando ${nomeDoProvedor}…`;
  }
  if (aquecimento.situacao === "pronto") return `${nomeDoProvedor} está respondendo`;
  if (aquecimento.situacao === "naoInstalado") return aquecimento.mensagem ?? `${nomeDoProvedor} não instalado`;
  return `${nomeDoProvedor} não respondeu: ${aquecimento.mensagem ?? "motivo desconhecido"}`;
}

/** Texto para mostrar no lugar de um conteúdo vazio, como o antes de uma nota criada. */
export function textoOuTracoVazio(texto: string | null): string {
  return texto === null || texto === "" ? "(vazio)" : texto;
}
