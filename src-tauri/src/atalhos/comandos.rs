//! Comandos Tauri dos atalhos: ler as combinações e trocar uma delas. Cada troca é validada,
//! registrada no sistema e só depois gravada no config.json.

use super::contrato::{validar_combinacao, AcaoDeAtalho};
use super::registro::trocar_atalho;
use crate::configuracao::{gravar_configuracao, ler_configuracao, AtalhosDoApp};

/// Combinações atuais de cada ação, para a tela de configurações.
#[tauri::command]
pub fn atalhos_do_app(app: tauri::AppHandle) -> Result<AtalhosDoApp, String> {
    Ok(ler_configuracao(&app)?.atalhos)
}

/// Troca a combinação de uma ação. Recusa combinação inválida ou já usada por outra ação.
#[tauri::command]
pub fn definir_atalho(app: tauri::AppHandle, acao: AcaoDeAtalho, combinacao: String) -> Result<AtalhosDoApp, String> {
    let nova = validar_combinacao(&combinacao)?;
    let mut configuracao = ler_configuracao(&app)?;
    if configuracao.atalhos.outra_acao_usa(acao, &nova) {
        return Err("essa combinação já é usada por outra ação".into());
    }
    let atual = configuracao.atalhos.da_acao(acao).to_string();
    trocar_atalho(&app, acao, &atual, &nova)?;
    configuracao.atalhos.definir(acao, nova);
    gravar_configuracao(&app, &configuracao)?;
    Ok(configuracao.atalhos)
}
