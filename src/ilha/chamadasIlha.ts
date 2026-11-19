import { invoke, isTauri } from "@tauri-apps/api/core";

export const dimensionarIlha = (expandida: boolean) => isTauri()
  ? invoke<void>("dimensionar_ilha", { expandida }) : Promise.resolve();
export const abrirChatNaIlha = (pedido: string | null = null) =>
  invoke<void>("abrir_chat_na_ilha", { pedido });
export const abrirJanelaPrincipal = () => invoke<void>("abrir_janela_principal");
export const encerrarAplicativo = () => invoke<void>("encerrar_aplicativo");
