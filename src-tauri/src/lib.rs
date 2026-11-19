//! Ponto de entrada do app. Este arquivo só declara os módulos, registra os comandos
//! e inicia os serviços de fundo. A lógica de cada funcionalidade fica nos próprios módulos.

use tauri::Manager;

mod agente;
mod atalhos;
mod cofre;
mod configuracao;
mod documentos;
mod grafo;
mod ilha;
pub mod mcp;
mod notas;
mod projetos;
mod resumo;
mod segredos;
mod tarefas;

/// Inicia a janela do app, registra os comandos e o agendador de avisos.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Atalhos globais salvos no config.json, para abrir a ilha e a janela de qualquer programa.
            atalhos::registro::registrar_atalhos_salvos(app.handle());
            // A ilha é a entrada habitual; sem cofre, mostre também a configuração inicial.
            if cofre::comandos::cofre_atual(app.handle().clone()).ok().flatten().is_none() {
                if let Some(janela) = app.get_webview_window("main") { janela.show()?; }
            }
            app.manage(agente::aquecimento::EstadoDoAquecimento::default());
            app.manage(segredos::comandos::EstadoDoCofreDeSegredos::default());
            // Confere os prazos em segundo plano e avisa pelo sistema.
            tarefas::avisos::iniciar_avisos(app.handle().clone());
            // Pede uma saudação ao agente em segundo plano, para a tela já abrir com a resposta.
            agente::aquecimento::iniciar_aquecimento(app.handle());
            Ok(())
        })
        .on_window_event(|janela, evento| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = evento {
                api.prevent_close();
                if janela.label() == "main" {
                    let _ = janela.hide();
                } else {
                    let _ = ilha::dimensionar_ilha(janela.app_handle().clone(), false);
                    let _ = tauri::Emitter::emit(janela, "ilha-recolher", ());
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            ilha::dimensionar_ilha,
            ilha::abrir_chat_na_ilha,
            ilha::abrir_janela_principal,
            ilha::encerrar_aplicativo,
            cofre::comandos::cofre_atual,
            cofre::comandos::abrir_cofre,
            cofre::comandos::criar_cofre,
            cofre::comandos::criar_pasta_no_cofre,
            cofre::comandos::listar_arvore,
            documentos::comandos::importar_documentos,
            projetos::comandos::listar_projetos,
            projetos::comandos::criar_projeto,
            notas::comandos::listar_notas,
            notas::comandos::ler_nota,
            notas::comandos::salvar_nota,
            notas::comandos::criar_nota,
            notas::comandos::backlinks,
            notas::comandos::guardar_copia_do_conflito,
            grafo::comandos::grafo_do_cofre,
            tarefas::comandos::listar_tarefas,
            tarefas::comandos::criar_tarefa,
            tarefas::comandos::atualizar_tarefa,
            tarefas::comandos::apagar_tarefa,
            segredos::comandos::estado_do_cofre_de_segredos,
            segredos::comandos::criar_cofre_de_segredos,
            segredos::comandos::listar_segredos,
            segredos::comandos::criar_segredo,
            segredos::comandos::copiar_senha_do_segredo,
            segredos::comandos::apagar_segredo,
            agente::comandos::estado_do_agente,
            agente::comandos::definir_provedor_do_agente,
            agente::comandos::enviar_pedido_ao_agente,
            agente::comandos::listar_historico_do_agente,
            agente::comandos::desfazer_acao_do_agente,
            agente::comandos::estado_do_aquecimento,
            agente::comandos::aquecer_agente,
            atalhos::comandos::atalhos_do_app,
            atalhos::comandos::definir_atalho,
            resumo::comandos::resumo_da_semana,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
