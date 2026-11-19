// Chamadas ao backend da funcionalidade agente. Componentes passam por aqui, não usam invoke direto.

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AquecimentoDoAgente, EstadoDoProvedor, FalaParaOAgente, ModoDeExecucao, RegistroDoAgente } from "./contratoAgente";

/** Nome do evento que o backend emite a cada mudança do aquecimento. Mantém igual ao Rust. */
const NOME_DO_EVENTO_DE_AQUECIMENTO = "agente-aquecimento";

/** Nome do evento com cada passo do agente (ferramenta usada). Mantém igual ao Rust. */
const NOME_DO_EVENTO_DE_PASSO = "agente-passo";

/** Provedor de agente e se o programa dele está instalado. */
export const estadoDoAgente = () => invoke<EstadoDoProvedor>("estado_do_agente");

/** Seleciona um provedor para os próximos pedidos. */
export const definirProvedorDoAgente = (id: string) =>
  invoke<EstadoDoProvedor>("definir_provedor_do_agente", { id });

/** Situação atual do aquecimento, para a tela ler ao abrir. */
export const estadoDoAquecimento = () => invoke<AquecimentoDoAgente>("estado_do_aquecimento");

/** Refaz o aquecimento, por exemplo depois de uma falha. */
export const aquecerAgente = () => invoke<void>("aquecer_agente");

/** Avisa `aoReceber` a cada mudança do aquecimento. Devolve a função que cancela a escuta. */
export const ouvirAquecimentoDoAgente = (aoReceber: (aquecimento: AquecimentoDoAgente) => void) =>
  listen<AquecimentoDoAgente>(NOME_DO_EVENTO_DE_AQUECIMENTO, (evento) => aoReceber(evento.payload));

/** Escuta partes novas de uma resposta OpenCode enquanto o pedido ainda está sendo processado. */
export const ouvirRespostaParcialDoAgente = (aoReceber: (trecho: { idDaRodada: string; texto: string }) => void) =>
  listen<{ idDaRodada: string; texto: string }>("agente-resposta-parcial", (evento) => aoReceber(evento.payload));

/** Escuta cada passo do agente (ferramenta usada). Devolve a função que cancela a escuta. */
export const ouvirPassoDoAgente = (aoReceber: (passo: { idDaRodada: string; texto: string }) => void) =>
  listen<{ idDaRodada: string; texto: string }>(NOME_DO_EVENTO_DE_PASSO, (evento) => aoReceber(evento.payload));

/** Envia o pedido junto ao identificador usado para filtrar eventos. `planoAprovado` vai só na rodada de execução do plano. */
export const enviarPedidoAoAgente = (
  pedido: string,
  conversaAnterior: FalaParaOAgente[],
  idDaRodada: string,
  modo: ModoDeExecucao,
  planoAprovado: string | null,
) => invoke<RegistroDoAgente>("enviar_pedido_ao_agente", { pedido, conversaAnterior, idDaRodada, modo, planoAprovado });

/** Histórico de pedidos, do mais recente para o mais antigo. */
export const listarHistoricoDoAgente = () => invoke<RegistroDoAgente[]>("listar_historico_do_agente");

/** Desfaz um pedido pelo id. Devolve o registro marcado como desfeito. */
export const desfazerAcaoDoAgente = (id: string) =>
  invoke<RegistroDoAgente>("desfazer_acao_do_agente", { id });
