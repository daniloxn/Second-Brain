//! Lê e grava o envelope cifrado dentro de `_segredos/` no cofre ativo.
//! Nunca cria cópias em texto puro nem recebe caminhos do frontend.

use super::contrato::CofreDeSegredos;
use super::criptografia::{cifrar, decifrar};
use crate::cofre::caminhos::{caminho_dentro_do_cofre, raiz_do_cofre_ativo};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use zeroize::Zeroizing;

const NOME_DO_ARQUIVO: &str = "cofre.enc";
const TAMANHO_MAXIMO_DO_ARQUIVO: u64 = 64 * 1024 * 1024;

/// Caminho canônico do arquivo único do cofre de segredos.
fn caminho_do_cofre(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let pasta_dos_segredos = raiz_do_cofre_ativo(app)?.join("_segredos");
    fs::create_dir_all(&pasta_dos_segredos).map_err(|erro| erro.to_string())?;
    let caminho = pasta_dos_segredos.join(NOME_DO_ARQUIVO);
    caminho_dentro_do_cofre(app, &caminho.to_string_lossy())
}

/// Informa se o cofre de segredos já foi inicializado.
pub(crate) fn existe_cofre_de_segredos(app: &tauri::AppHandle) -> Result<bool, String> {
    Ok(caminho_do_cofre(app)?.is_file())
}

/// Cria um cofre cifrado vazio e recusa substituir um arquivo existente.
pub(crate) fn criar_cofre_de_segredos(app: &tauri::AppHandle, senha_mestra: &str) -> Result<(), String> {
    let caminho = caminho_do_cofre(app)?;
    if caminho.exists() {
        return Err("o cofre de segredos já foi criado".into());
    }
    gravar_cofre_cifrado(&caminho, &CofreDeSegredos { segredos: Vec::new() }, senha_mestra)
}

/// Abre e valida o conteúdo inteiro, mantendo o texto decifrado em memória protegida.
pub(crate) fn ler_cofre_de_segredos(app: &tauri::AppHandle, senha_mestra: &str) -> Result<CofreDeSegredos, String> {
    let caminho = caminho_do_cofre(app)?;
    let metadados = fs::metadata(&caminho).map_err(|erro| format!("cofre de segredos não encontrado: {erro}"))?;
    if metadados.len() > TAMANHO_MAXIMO_DO_ARQUIVO {
        return Err("arquivo do cofre de segredos excede o limite permitido".into());
    }
    let arquivo = fs::read(caminho).map_err(|erro| erro.to_string())?;
    let conteudo = decifrar(&arquivo, senha_mestra)?;
    serde_json::from_slice(&conteudo).map_err(|_| "conteúdo do cofre de segredos inválido".into())
}

/// Serializa e cifra antes de gravar qualquer conteúdo no disco.
pub(crate) fn salvar_cofre_de_segredos(
    app: &tauri::AppHandle,
    cofre: &CofreDeSegredos,
    senha_mestra: &str,
) -> Result<(), String> {
    gravar_cofre_cifrado(&caminho_do_cofre(app)?, cofre, senha_mestra)
}

/// Grava apenas bytes do envelope autenticado, nunca o JSON legível das credenciais.
fn gravar_cofre_cifrado(
    caminho: &PathBuf,
    cofre: &CofreDeSegredos,
    senha_mestra: &str,
) -> Result<(), String> {
    let conteudo = Zeroizing::new(serde_json::to_vec(cofre).map_err(|erro| erro.to_string())?);
    if conteudo.len() as u64 > TAMANHO_MAXIMO_DO_ARQUIVO {
        return Err("cofre de segredos excede o limite de 64 MB".into());
    }
    let arquivo = cifrar(&conteudo, senha_mestra)?;
    let mut identificador_temporario = [0; 8];
    getrandom::fill(&mut identificador_temporario).map_err(|erro| erro.to_string())?;
    let nome_temporario = identificador_temporario.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let arquivo_temporario = caminho.with_file_name(format!("cofre-{nome_temporario}.tmp"));
    let mut arquivo_aberto = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&arquivo_temporario)
        .map_err(|erro| erro.to_string())?;
    arquivo_aberto.write_all(&arquivo).map_err(|erro| erro.to_string())?;
    arquivo_aberto.sync_all().map_err(|erro| erro.to_string())?;
    drop(arquivo_aberto);
    if let Err(erro) = fs::rename(&arquivo_temporario, caminho) {
        let _ = fs::remove_file(arquivo_temporario);
        return Err(format!("não foi possível atualizar o cofre de segredos: {erro}"));
    }
    Ok(())
}
