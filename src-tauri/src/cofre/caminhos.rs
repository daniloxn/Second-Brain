//! Caminhos do cofre: descobre a pasta raiz ativa e garante que nenhum caminho
//! recebido do frontend escapa dela.

use crate::configuracao::ler_configuracao;
use std::path::{Path, PathBuf};

/// Pasta raiz do cofre ativo, já canonizada (caminho absoluto e sem atalhos).
/// Falha se nenhum cofre foi escolhido ainda.
pub(crate) fn raiz_do_cofre_ativo(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let configuracao = ler_configuracao(app)?;
    let caminho_salvo = configuracao
        .cofre
        .ok_or_else(|| "nenhum cofre aberto".to_string())?;
    PathBuf::from(caminho_salvo)
        .canonicalize()
        .map_err(|e| e.to_string())
}

/// Converte um caminho recebido do frontend em caminho canônico e confere
/// que ele está dentro do cofre ativo. Recusa qualquer outro lugar do disco.
///
/// Para arquivos que ainda não existem (ex.: nota nova), canoniza a pasta pai
/// e junta o nome do arquivo, porque `canonicalize` exige que o caminho exista.
pub(crate) fn caminho_dentro_do_cofre(app: &tauri::AppHandle, caminho_recebido: &str) -> Result<PathBuf, String> {
    let raiz_do_cofre = raiz_do_cofre_ativo(app)?;
    caminho_dentro_da_raiz(&raiz_do_cofre, Path::new(caminho_recebido))
}

/// Mesma checagem de `caminho_dentro_do_cofre`, para quem já tem a raiz em mãos e não tem
/// `AppHandle` (o servidor MCP). `raiz_do_cofre` precisa estar canonizada. Um caminho relativo
/// é juntado à raiz; um caminho absoluto de fora do cofre é recusado pela mesma checagem.
pub(crate) fn caminho_dentro_da_raiz(raiz_do_cofre: &Path, caminho_alvo: &Path) -> Result<PathBuf, String> {
    let caminho_alvo = raiz_do_cofre.join(caminho_alvo);

    let caminho_canonico = if caminho_alvo.exists() {
        caminho_alvo.canonicalize().map_err(|e| e.to_string())?
    } else {
        let pasta_pai = caminho_alvo.parent().ok_or("caminho inválido")?;
        let nome_do_arquivo = caminho_alvo.file_name().ok_or("caminho inválido")?;
        pasta_pai
            .canonicalize()
            .map_err(|e| e.to_string())?
            .join(nome_do_arquivo)
    };

    if !caminho_canonico.starts_with(&raiz_do_cofre) {
        return Err("caminho fora do cofre".into());
    }
    Ok(caminho_canonico)
}

/// Texto de um caminho para enviar ao frontend. Remove o prefixo `\\?\` que o
/// Windows põe em caminhos canonizados, para que o frontend compare caminhos iguais.
pub(crate) fn texto_para_frontend(caminho: &Path) -> String {
    let texto = caminho.to_string_lossy().into_owned();
    texto
        .strip_prefix(r"\\?\")
        .map(str::to_string)
        .unwrap_or(texto)
}
