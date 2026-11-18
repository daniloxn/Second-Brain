// Chamadas ao backend da funcionalidade agente. Componentes passam por aqui, não usam invoke direto.

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { AquecimentoDoAgente, EstadoDoProvedor, FalaParaOAgente, RegistroDoAgente } from "./contratoAgente";

/** Nome do evento que o backend emite a cada mudança do aquecimento. Mantém igual ao Rust. */
const NOME_DO_EVENTO_DE_AQUECIMENTO = "agente-aquecimento";

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

/** Envia um pedido em linguagem natural com a conversa anterior. Pode levar minutos. */
export const enviarPedidoAoAgente = (pedido: string, conversaAnterior: FalaParaOAgente[]) =>
  invoke<RegistroDoAgente>("enviar_pedido_ao_agente", { pedido, conversaAnterior });

/** Histórico de pedidos, do mais recente para o mais antigo. */
export const listarHistoricoDoAgente = () => invoke<RegistroDoAgente[]>("listar_historico_do_agente");

/** Desfaz um pedido pelo id. Devolve o registro marcado como desfeito. */
export const desfazerAcaoDoAgente = (id: string) =>
  invoke<RegistroDoAgente>("desfazer_acao_do_agente", { id });
