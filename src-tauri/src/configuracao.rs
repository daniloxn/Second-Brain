//! Configuração do app: guarda qual pasta é o cofre ativo, em config.json.
//! Não sabe nada sobre notas nem sobre a estrutura interna do cofre.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

/// Conteúdo do config.json guardado na pasta de configuração do app.
#[derive(Serialize, Deserialize, Default)]
pub struct Configuracao {
    /// Caminho canônico do cofre ativo, sem o prefixo `\\?\` do Windows.
    pub cofre: Option<String>,
    /// Combinações dos atalhos globais. Config antigo sem o campo recebe os padrões.
    #[serde(default)]
    pub atalhos: AtalhosDoApp,
}

/// Combinação de cada atalho global, no formato do plugin (ex.: `Ctrl+Alt+Space`).
/// Só guarda o texto; quem valida e registra os atalhos é o módulo `atalhos`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AtalhosDoApp {
    pub abrir_ilha: String,
    pub abrir_janela_principal: String,
}

impl Default for AtalhosDoApp {
    /// Combinações iniciais, usadas quando o config.json ainda não tem atalhos.
    fn default() -> Self {
        Self {
            abrir_ilha: "Ctrl+Alt+Space".into(),
            abrir_janela_principal: "Ctrl+Alt+M".into(),
        }
    }
}

/// Caminho completo do config.json. Cria a pasta de configuração se ela não existir.
fn caminho_do_arquivo_de_configuracao(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let pasta_de_configuracao = app.path().app_config_dir().map_err(|e| e.to_string())?;
    fs::create_dir_all(&pasta_de_configuracao).map_err(|e| e.to_string())?;
    Ok(pasta_de_configuracao.join("config.json"))
}

/// Lê a configuração salva. Se o arquivo não existir ou estiver corrompido,
/// devolve a configuração vazia em vez de travar o app.
pub fn ler_configuracao(app: &tauri::AppHandle) -> Result<Configuracao, String> {
    let caminho_do_arquivo = caminho_do_arquivo_de_configuracao(app)?;
    if !caminho_do_arquivo.exists() {
        return Ok(Configuracao::default());
    }
    let texto = fs::read_to_string(&caminho_do_arquivo).map_err(|e| e.to_string())?;
    Ok(serde_json::from_str(&texto).unwrap_or_default())
}

/// Grava a configuração inteira no config.json, substituindo a anterior.
pub fn gravar_configuracao(app: &tauri::AppHandle, configuracao: &Configuracao) -> Result<(), String> {
    let texto = serde_json::to_string_pretty(configuracao).map_err(|e| e.to_string())?;
    fs::write(caminho_do_arquivo_de_configuracao(app)?, texto).map_err(|e| e.to_string())
}
