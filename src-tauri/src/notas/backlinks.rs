//! Backlinks: procura, em todas as outras notas do cofre, as linhas que
//! apontam por wikilink para a nota escolhida.

use super::indice::{
    arquivo_e_grande_demais, coletar_caminhos_de_notas, e_arquivo_de_nota, nome_sem_extensao,
    PROFUNDIDADE_DE_BUSCA_DE_NOTAS,
};
use super::wikilinks::extrair_alvos_dos_wikilinks;
use crate::cofre::caminhos::{caminho_dentro_do_cofre, raiz_do_cofre_ativo, texto_para_frontend};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

/// Quantidade máxima de backlinks devolvidos de uma vez.
const LIMITE_DE_BACKLINKS: usize = 300;

/// Uma linha de outra nota que aponta para a nota consultada.
#[derive(Serialize)]
pub struct Backlink {
    /// Nome da nota que contém o link.
    nome: String,
    /// Caminho completo dessa nota.
    caminho: String,
    /// Número da linha (a partir de 1) onde está o link.
    linha: usize,
    /// Texto da linha, cortado para caber no painel.
    trecho: String,
}

/// Backlinks da nota em `caminho_da_nota`. Ignora a própria nota.
pub(crate) fn procurar_backlinks(app: &tauri::AppHandle, caminho_da_nota: &str) -> Result<Vec<Backlink>, String> {
    let nota_consultada = caminho_dentro_do_cofre(app, caminho_da_nota)?;
    let nome_procurado = nome_sem_extensao(&nota_consultada).to_lowercase();
    let raiz_do_cofre = raiz_do_cofre_ativo(app)?;

    let mut caminhos_das_notas: Vec<PathBuf> = Vec::new();
    coletar_caminhos_de_notas(&raiz_do_cofre, PROFUNDIDADE_DE_BUSCA_DE_NOTAS, &mut caminhos_das_notas);

    let mut backlinks_encontrados = Vec::new();
    for caminho_de_outra_nota in caminhos_das_notas {
        if caminho_de_outra_nota == nota_consultada || !e_arquivo_de_nota(&caminho_de_outra_nota) {
            continue;
        }
        if arquivo_e_grande_demais(&caminho_de_outra_nota) {
            continue;
        }
        let Ok(texto_da_nota) = fs::read_to_string(&caminho_de_outra_nota) else {
            continue;
        };
        for (indice_da_linha, linha_do_texto) in texto_da_nota.lines().enumerate() {
            let linha_aponta_para_a_nota = extrair_alvos_dos_wikilinks(linha_do_texto)
                .iter()
                .any(|alvo| alvo == &nome_procurado);
            if !linha_aponta_para_a_nota {
                continue;
            }
            backlinks_encontrados.push(Backlink {
                nome: nome_sem_extensao(&caminho_de_outra_nota),
                caminho: texto_para_frontend(&caminho_de_outra_nota),
                linha: indice_da_linha + 1,
                trecho: linha_do_texto.trim().chars().take(200).collect(),
            });
            if backlinks_encontrados.len() >= LIMITE_DE_BACKLINKS {
                return Ok(backlinks_encontrados);
            }
        }
    }
    Ok(backlinks_encontrados)
}
