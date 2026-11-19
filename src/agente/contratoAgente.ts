// Tipos do agente, espelhados de src-tauri/src/agente/contrato.rs. Ao mudar um campo lá,
// mude aqui na mesma alteração.

/** Tipo de mudança encontrada no cofre depois de um pedido. */
export type TipoDeMudanca = "criado" | "alterado" | "apagado" | "pastaCriada" | "pastaApagada";

/** Uma mudança em nota ou pasta. `antes` e `depois` são o texto da nota (null quando não existia). */
export interface MudancaNoCofre {
  caminho: string;
  tipo: TipoDeMudanca;
  antes: string | null;
  depois: string | null;
}

/** Como a rodada roda. `executar` pode mudar o cofre; `planejar` só lê e devolve um plano. Espelha `ModoDeExecucao` no Rust. */
export type ModoDeExecucao = "executar" | "planejar" | "consultar";

/** Um pedido feito ao agente, com a resposta e o que mudou. É uma linha do histórico. */
export interface RegistroDoAgente {
  id: string;
  /** Data e hora locais, no formato `AAAA-MM-DD HH:MM:SS`. */
  quando: string;
  pedido: string;
  provedor: string;
  resposta: string;
  mudancas: MudancaNoCofre[];
  desfeito: boolean;
  /** Registros antigos não têm o campo; contam como `executar`. */
  modo?: ModoDeExecucao;
  /** Tempo da rodada em milissegundos. Zero ou ausente quando o registro é anterior à medição. */
  duracaoMs?: number;
}

/** Fala da conversa enviada junto do pedido, para o agente entender "isso" e "sim". Espelha `FalaDaConversa` no Rust. */
export interface FalaParaOAgente {
  papel: "voce" | "agente";
  texto: string;
}

/** Provedor de agente e se o programa dele está instalado. */
export interface EstadoDoProvedor {
  id: string;
  nome: string;
  comando: string;
  instalado: boolean;
}

/** Etapa do aquecimento: a saudação feita ao abrir o app. */
export type SituacaoDoAquecimento = "aguardando" | "aquecendo" | "pronto" | "falhou" | "naoInstalado";

/** Situação do aquecimento. `mensagem` é a resposta do agente quando está pronto, ou o motivo da falha. */
export interface AquecimentoDoAgente {
  situacao: SituacaoDoAquecimento;
  mensagem: string | null;
}
