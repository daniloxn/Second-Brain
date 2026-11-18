//! Ponto de entrada do app. Este arquivo só declara os módulos, registra os comandos
//! e inicia os serviços de fundo. A lógica de cada funcionalidade fica nos próprios módulos.

use tauri::Manager;

mod agente;
mod cofre;
mod configuracao;
mod documentos;
mod grafo;
mod notas;
mod projetos;
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
        .setup(|app| {
            app.manage(agente::aquecimento::EstadoDoAquecimento::default());
            app.manage(segredos::comandos::EstadoDoCofreDeSegredos::default());
            // Confere os prazos em segundo plano e avisa pelo sistema.
            tarefas::avisos::iniciar_avisos(app.handle().clone());
            // Pede uma saudação ao agente em segundo plano, para a tela já abrir com a resposta.
            agente::aquecimento::iniciar_aquecimento(app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
