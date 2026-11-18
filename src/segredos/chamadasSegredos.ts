// Chamadas Tauri do cofre de segredos. Este arquivo é a única camada frontend que usa invoke.

import { invoke } from "@tauri-apps/api/core";
import type { NovoSegredo, ResumoDeSegredo } from "./contratoSegredos";

/** Verifica se existe um arquivo criptografado de segredos no cofre ativo. */
export const existeCofreDeSegredos = () => invoke<boolean>("estado_do_cofre_de_segredos");

/** Cria o arquivo cifrado inicial a partir da senha mestra escolhida. */
export const criarCofreDeSegredos = (senhaMestra: string) =>
  invoke<void>("criar_cofre_de_segredos", { senhaMestra });

/** Desbloqueia o cofre e recebe somente os campos que podem aparecer na lista. */
export const listarSegredos = (senhaMestra: string) =>
  invoke<ResumoDeSegredo[]>("listar_segredos", { senhaMestra });

/** Grava uma credencial cifrando de novo todo o arquivo. */
export const criarSegredo = (senhaMestra: string, entrada: NovoSegredo) =>
  invoke<ResumoDeSegredo[]>("criar_segredo", { senhaMestra, entrada });

/** Desbloqueia apenas a senha pedida para a ação de copiar. */
export const copiarSenhaDoSegredo = (id: string, senhaMestra: string) =>
  invoke<string>("copiar_senha_do_segredo", { id, senhaMestra });

/** Remove uma credencial após a confirmação da pessoa na tela. */
export const apagarSegredo = (id: string, senhaMestra: string) =>
  invoke<ResumoDeSegredo[]>("apagar_segredo", { id, senhaMestra });
