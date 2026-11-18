//! Resposta das importações: nome original para exibição e caminho relativo seguro.

use serde::Serialize;

/// Um arquivo copiado para a pasta `documentos/` do cofre.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentoImportado {
    pub nome: String,
    pub caminho_relativo: String,
}
