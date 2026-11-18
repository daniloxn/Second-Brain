//! Formatos que o agente troca com a tela e grava no histórico. Tudo em camelCase,
//! como nas outras funcionalidades. Mudar um campo aqui exige mudar `src/agente/contratoAgente.ts`.

use serde::{Deserialize, Serialize};

/// Tipo de mudança encontrada no cofre depois de um pedido.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TipoDeMudanca {
    Criado,
    Alterado,
    Apagado,
    PastaCriada,
    PastaApagada,
}

/// Uma mudança em uma nota ou pasta do cofre. `antes` e `depois` guardam o texto da nota
/// (None quando ela não existia naquele momento). Pastas não têm texto.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MudancaNoCofre {
    pub caminho: String,
    pub tipo: TipoDeMudanca,
    pub antes: Option<String>,
    pub depois: Option<String>,
}

/// Um pedido feito ao agente, com a resposta e as mudanças. É uma linha do histórico.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegistroDoAgente {
    pub id: String,
    /// Data e hora local no formato `AAAA-MM-DD HH:MM:SS`.
    pub quando: String,
    pub pedido: String,
    pub provedor: String,
    pub resposta: String,
    pub mudancas: Vec<MudancaNoCofre>,
    pub desfeito: bool,
}

/// Uma fala da conversa da tela, enviada junto do pedido para o agente entender "isso", "aquilo" e "sim".
/// `papel` é "voce" ou "agente". Espelha `FalaParaOAgente` em `src/agente/contratoAgente.ts`.
#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FalaDaConversa {
    pub papel: String,
    pub texto: String,
}

/// Situação do provedor de agente na máquina.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct EstadoDoProvedor {
    pub id: String,
    pub nome: String,
    pub comando: String,
    pub instalado: bool,
}
