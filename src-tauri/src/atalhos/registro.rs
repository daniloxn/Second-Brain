//! Registro dos atalhos no sistema pelo plugin de atalhos globais. Só este arquivo fala com o plugin.
//! Quando a combinação já é usada por outro programa, o plugin recusa e o erro vai para a tela.

use super::contrato::AcaoDeAtalho;
use crate::configuracao::ler_configuracao;
use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};

/// Registra as combinações salvas no config.json. Chamado ao abrir o app.
/// Um atalho que falha não impede os outros: o erro vai para o terminal e o app segue.
pub fn registrar_atalhos_salvos(app: &AppHandle) {
    let Ok(configuracao) = ler_configuracao(app) else {
        return;
    };
    for acao in AcaoDeAtalho::TODAS {
        if let Err(erro) = registrar_atalho(app, acao, configuracao.atalhos.da_acao(acao)) {
            eprintln!("{erro}");
        }
    }
}

/// Liga uma combinação a uma ação. O disparo só conta na tecla pressionada, não na soltura.
pub fn registrar_atalho(app: &AppHandle, acao: AcaoDeAtalho, combinacao: &str) -> Result<(), String> {
    app.global_shortcut()
        .on_shortcut(combinacao, move |app_do_evento, _atalho, evento| {
            if evento.state() == ShortcutState::Pressed {
                executar_acao(app_do_evento, acao);
            }
        })
        .map_err(|erro| format!("o atalho {combinacao} não pôde ser usado; talvez outro programa já o tenha: {erro}"))
}

/// Troca a combinação de uma ação. Se a nova não puder ser registrada, a antiga volta a valer.
pub fn trocar_atalho(app: &AppHandle, acao: AcaoDeAtalho, atual: &str, nova: &str) -> Result<(), String> {
    let _ = app.global_shortcut().unregister(atual);
    if let Err(erro) = registrar_atalho(app, acao, nova) {
        let _ = registrar_atalho(app, acao, atual);
        return Err(erro);
    }
    Ok(())
}

/// Faz a ação do atalho. Erro ao abrir a janela não tem para onde ir, então é ignorado.
fn executar_acao(app: &AppHandle, acao: AcaoDeAtalho) {
    match acao {
        AcaoDeAtalho::AbrirIlha => {
            let _ = crate::ilha::abrir_chat_na_ilha(app.clone(), None);
        }
        AcaoDeAtalho::AbrirJanelaPrincipal => {
            let _ = crate::ilha::abrir_janela_principal(app.clone());
        }
    }
}
