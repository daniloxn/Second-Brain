//! Comando Tauri do grafo. Pega a pasta do cofre ativo e entrega o grafo calculado.

use super::contrato::GrafoDoCofre;
use super::montagem::montar_grafo_do_cofre;
use crate::cofre::caminhos::raiz_do_cofre_ativo;

/// Grafo de notas e tarefas do cofre ativo, com ligações e posições 3D.
/// Roda em outra thread: ler o cofre e calcular o layout pode levar um tempo, e um comando
/// comum travaria a janela.
#[tauri::command]
pub async fn grafo_do_cofre(app: tauri::AppHandle) -> Result<GrafoDoCofre, String> {
    let raiz_do_cofre = raiz_do_cofre_ativo(&app)?;
    tauri::async_runtime::spawn_blocking(move || montar_grafo_do_cofre(&raiz_do_cofre))
        .await
        .map_err(|erro| erro.to_string())
}
