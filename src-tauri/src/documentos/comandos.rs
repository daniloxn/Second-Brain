//! Comandos Tauri da importação de documentos.
//! A tela envia só os caminhos de arquivos que a pessoa soltou no app.

use super::contrato::DocumentoImportado;
use super::importacao::copiar_arquivos_para_documentos;

/// Copia arquivos soltos no app para a pasta `documentos/` do cofre ativo.
#[tauri::command]
pub fn importar_documentos(
    app: tauri::AppHandle,
    caminhos_dos_arquivos: Vec<String>,
) -> Result<Vec<DocumentoImportado>, String> {
    copiar_arquivos_para_documentos(&app, &caminhos_dos_arquivos)
}
