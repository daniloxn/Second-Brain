//! Árvore de arquivos e pastas do cofre, no formato que o explorador do frontend espera.
//! Pastas ocultas (nome começando com ponto, como .git) ficam de fora.

use super::caminhos::texto_para_frontend;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Profundidade máxima da árvore enviada ao frontend, para não varrer o disco inteiro.
const PROFUNDIDADE_MAXIMA_DA_ARVORE: u8 = 6;

/// Um item da árvore: arquivo ou pasta, com os filhos já ordenados.
#[derive(Serialize)]
pub struct NoArvore {
    /// Nome do arquivo ou pasta, sem o caminho.
    nome: String,
    /// Caminho completo, no formato que o frontend usa.
    caminho: String,
    /// Verdadeiro para pasta, falso para arquivo.
    pasta: bool,
    /// Itens dentro desta pasta. Vazio para arquivos.
    filhos: Vec<NoArvore>,
}

/// Monta a árvore a partir de `caminho`, descendo no máximo `profundidade` níveis.
/// Pastas vêm antes dos arquivos, e cada grupo em ordem alfabética.
pub(crate) fn montar_arvore(caminho: &Path, profundidade: u8) -> Result<NoArvore, String> {
    let nome = caminho
        .file_name()
        .map(|nome| nome.to_string_lossy().into_owned())
        .unwrap_or_else(|| caminho.display().to_string());
    let e_pasta = caminho.is_dir();
    let mut filhos = Vec::new();

    if e_pasta && profundidade > 0 {
        let entradas = fs::read_dir(caminho).map_err(|e| e.to_string())?;
        for entrada in entradas.flatten() {
            let nome_da_entrada = entrada.file_name().to_string_lossy().into_owned();
            if nome_da_entrada.starts_with('.') {
                continue;
            }
            filhos.push(montar_arvore(&entrada.path(), profundidade - 1)?);
        }
        filhos.sort_by(|a, b| b.pasta.cmp(&a.pasta).then_with(|| a.nome.cmp(&b.nome)));
    }

    Ok(NoArvore {
        nome,
        caminho: texto_para_frontend(caminho),
        pasta: e_pasta,
        filhos,
    })
}

/// Árvore completa do cofre, usando a profundidade padrão do explorador.
pub(crate) fn montar_arvore_padrao(caminho: &Path) -> Result<NoArvore, String> {
    montar_arvore(caminho, PROFUNDIDADE_MAXIMA_DA_ARVORE)
}
