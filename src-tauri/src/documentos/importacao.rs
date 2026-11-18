//! Valida nomes e tamanhos antes de copiar os arquivos para o cofre.
//! Arquivos de origem ficam intactos; colisões recebem um sufixo numérico.

use super::contrato::DocumentoImportado;
use crate::cofre::caminhos::{caminho_dentro_do_cofre, raiz_do_cofre_ativo};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

const LIMITE_DE_ARQUIVOS: usize = 20;
const LIMITE_POR_ARQUIVO: u64 = 50 * 1024 * 1024;
const LIMITE_DA_IMPORTACAO: u64 = 100 * 1024 * 1024;

struct ArquivoProntoParaCopiar {
    origem: PathBuf,
    destino: PathBuf,
    nome: String,
}

/// Valida todos os arquivos, escolhe destinos únicos e só então começa a copiá-los.
pub(super) fn copiar_arquivos_para_documentos(
    app: &tauri::AppHandle,
    caminhos_recebidos: &[String],
) -> Result<Vec<DocumentoImportado>, String> {
    if caminhos_recebidos.is_empty() || caminhos_recebidos.len() > LIMITE_DE_ARQUIVOS {
        return Err(format!("solte entre 1 e {LIMITE_DE_ARQUIVOS} arquivos por importação"));
    }

    let raiz = raiz_do_cofre_ativo(app)?;
    let pasta_documentos = raiz.join("documentos");
    fs::create_dir_all(&pasta_documentos).map_err(|erro| erro.to_string())?;
    caminho_dentro_do_cofre(app, &pasta_documentos.to_string_lossy())?;

    let mut total_em_bytes = 0_u64;
    let mut nomes_reservados = HashSet::new();
    let mut arquivos = Vec::with_capacity(caminhos_recebidos.len());
    for caminho_recebido in caminhos_recebidos {
        let origem = PathBuf::from(caminho_recebido)
            .canonicalize()
            .map_err(|erro| format!("não foi possível acessar um arquivo solto: {erro}"))?;
        let metadados = fs::metadata(&origem).map_err(|erro| erro.to_string())?;
        if !metadados.is_file() {
            return Err(format!("{} não é um arquivo", origem.display()));
        }
        if metadados.len() > LIMITE_POR_ARQUIVO {
            return Err(format!("{} excede o limite de 50 MB", origem.display()));
        }
        total_em_bytes = total_em_bytes.saturating_add(metadados.len());
        if total_em_bytes > LIMITE_DA_IMPORTACAO {
            return Err("os arquivos excedem o limite total de 100 MB".into());
        }

        let destino = caminho_com_nome_livre(&pasta_documentos, &origem, &mut nomes_reservados);
        caminho_dentro_do_cofre(app, &destino.to_string_lossy())?;
        let nome = destino.file_name().unwrap_or_default().to_string_lossy().into_owned();
        arquivos.push(ArquivoProntoParaCopiar { origem, destino, nome });
    }

    copiar_todos_ou_reverter(&arquivos)?;
    Ok(arquivos
        .into_iter()
        .map(|arquivo| DocumentoImportado {
            caminho_relativo: format!("documentos/{}", arquivo.nome),
            nome: arquivo.nome,
        })
        .collect())
}

/// Copia cada arquivo; em erro, remove os destinos criados nesta mesma tentativa.
fn copiar_todos_ou_reverter(arquivos: &[ArquivoProntoParaCopiar]) -> Result<(), String> {
    let mut destinos_criados = Vec::new();
    for arquivo in arquivos {
        if let Err(erro) = fs::copy(&arquivo.origem, &arquivo.destino) {
            for destino in destinos_criados {
                let _ = fs::remove_file(destino);
            }
            return Err(format!("não foi possível importar {}: {erro}", arquivo.nome));
        }
        destinos_criados.push(arquivo.destino.clone());
    }
    Ok(())
}

/// Escolhe um nome seguro e livre, preservando a extensão quando ela é simples.
fn caminho_com_nome_livre(pasta: &Path, origem: &Path, nomes_reservados: &mut HashSet<String>) -> PathBuf {
    let nome_base = origem.file_stem().unwrap_or_default().to_string_lossy();
    let nome_base = limpar_nome(&nome_base);
    let extensao = origem
        .extension()
        .map(|extensao| extensao.to_string_lossy().chars().filter(char::is_ascii_alphanumeric).take(16).collect::<String>())
        .filter(|extensao| !extensao.is_empty());

    for numero in 1..=u32::MAX {
        let base_com_sufixo = if numero == 1 { nome_base.clone() } else { format!("{nome_base} ({numero})") };
        let nome = match &extensao {
            Some(extensao) => format!("{base_com_sufixo}.{extensao}"),
            None => base_com_sufixo,
        };
        let chave_do_nome = nome.to_lowercase();
        let destino = pasta.join(&nome);
        if !destino.exists() && nomes_reservados.insert(chave_do_nome) {
            return destino;
        }
    }
    pasta.join(format!("documento-{}.bin", std::process::id()))
}

/// Troca caracteres inadequados no nome do arquivo e impede nomes vazios.
fn limpar_nome(nome: &str) -> String {
    let nome_limpo: String = nome
        .chars()
        .map(|caractere| {
            if caractere.is_alphanumeric() || matches!(caractere, '-' | '_' | ' ' | '(' | ')' | '.') {
                caractere
            } else {
                '_'
            }
        })
        .collect();
    let nome_limpo = nome_limpo.trim().trim_matches('.');
    if nome_limpo.is_empty() { "documento".into() } else { nome_limpo.chars().take(160).collect() }
}
