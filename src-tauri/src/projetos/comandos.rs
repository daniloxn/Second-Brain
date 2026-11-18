//! Comandos Tauri do painel de projetos.
//! A criação usa modelos Markdown, e a listagem devolve somente esses documentos.

use super::repositorio::{criar_projeto_no_cofre, listar_projetos_do_cofre, ProjetoDoCofre};

/// Lista os projetos da pasta `projetos/` do cofre ativo.
#[tauri::command]
pub fn listar_projetos(app: tauri::AppHandle) -> Result<Vec<ProjetoDoCofre>, String> {
    listar_projetos_do_cofre(&app)
}

/// Cria uma pasta de projeto com `PROJETO.md`, `MAPA.md` e `LOG.md` iniciais.
#[tauri::command]
pub fn criar_projeto(app: tauri::AppHandle, nome: String) -> Result<ProjetoDoCofre, String> {
    criar_projeto_no_cofre(&app, &nome)
}
