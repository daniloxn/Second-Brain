//! Tipos internos do cofre criptografado e resumos que a tela pode exibir.
//! Senhas e anotações nunca fazem parte da resposta de listagem.

use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Versão da estrutura que existe somente dentro do texto criptografado.
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub(super) struct CofreDeSegredos {
    pub segredos: Vec<SegredoArmazenado>,
}

/// Credencial completa, mantida em memória só enquanto um comando executa.
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub(super) struct SegredoArmazenado {
    pub id: String,
    pub titulo: String,
    pub usuario: String,
    pub senha: String,
    pub anotacoes: String,
}

/// Campos que podem ser exibidos depois que a pessoa desbloqueia o cofre.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResumoDeSegredo {
    pub id: String,
    pub titulo: String,
    pub usuario: String,
    pub anotacoes: String,
}

/// Dados necessários para salvar uma credencial nova.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovoSegredo {
    pub titulo: String,
    #[serde(default)]
    pub usuario: String,
    pub senha: String,
    #[serde(default)]
    pub anotacoes: String,
}

impl From<&SegredoArmazenado> for ResumoDeSegredo {
    /// Descarta a senha antes da resposta sair do backend.
    fn from(segredo: &SegredoArmazenado) -> Self {
        Self {
            id: segredo.id.clone(),
            titulo: segredo.titulo.clone(),
            usuario: segredo.usuario.clone(),
            anotacoes: segredo.anotacoes.clone(),
        }
    }
}
