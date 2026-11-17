//! Cópia de segurança de um texto que não pôde ser salvo porque a nota mudou no disco.
//! As cópias ficam em `_sistema/conflitos/`, com o nome da nota e a hora, para nada se perder.

use super::indice::nome_sem_extensao;
use crate::cofre::caminhos::{caminho_dentro_do_cofre, raiz_do_cofre_ativo, texto_para_frontend};
use std::fs;
use std::path::Path;

/// Grava `conteudo` em `_sistema/conflitos/` e devolve o caminho da cópia.
/// O nome da cópia usa a hora com milissegundos, então duas cópias seguidas não se sobrescrevem.
pub(crate) fn gravar_copia_do_conflito(app: &tauri::AppHandle, caminho_da_nota: &str, conteudo: &str) -> Result<String, String> {
    let nota = caminho_dentro_do_cofre(app, caminho_da_nota)?;
    let pasta_de_conflitos = raiz_do_cofre_ativo(app)?.join("_sistema").join("conflitos");
    fs::create_dir_all(&pasta_de_conflitos).map_err(|erro| erro.to_string())?;

    let caminho_da_copia = pasta_de_conflitos.join(nome_da_copia(&nota));
    fs::write(&caminho_da_copia, conteudo).map_err(|erro| erro.to_string())?;
    Ok(texto_para_frontend(&caminho_da_copia))
}

/// Nome da cópia: `<nome da nota>-<data e hora>.md`.
fn nome_da_copia(caminho_da_nota: &Path) -> String {
    let momento = chrono::Local::now().format("%Y%m%d-%H%M%S%3f");
    format!("{}-{momento}.md", nome_sem_extensao(caminho_da_nota))
}
