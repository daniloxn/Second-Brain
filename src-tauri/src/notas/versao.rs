//! Versão de uma nota no disco: a data de modificação do arquivo, em milissegundos.
//! Quando a versão lida na tela não bate com a do disco, o arquivo mudou por fora
//! (ex.: um agente editou a nota), e o salvamento não pode sobrescrever.

use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

/// Data de modificação do arquivo em milissegundos desde 1970.
pub(crate) fn versao_do_arquivo(caminho_do_arquivo: &Path) -> Result<u64, String> {
    let data_de_modificacao = fs::metadata(caminho_do_arquivo)
        .and_then(|metadados| metadados.modified())
        .map_err(|erro| erro.to_string())?;
    let desde_1970 = data_de_modificacao
        .duration_since(UNIX_EPOCH)
        .map_err(|erro| erro.to_string())?;
    // Em u64 cabem milhões de anos em milissegundos; o limite é só para não estourar.
    Ok(u64::try_from(desde_1970.as_millis()).unwrap_or(u64::MAX))
}
