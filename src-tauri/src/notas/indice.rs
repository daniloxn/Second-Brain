//! Índice das notas do cofre: encontra todos os arquivos .md, sem seguir atalhos
//! de pasta (links simbólicos) e sem entrar em pastas ocultas.

use crate::cofre::caminhos::{raiz_do_cofre_ativo, texto_para_frontend};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Profundidade máxima de pastas ao procurar notas.
pub(crate) const PROFUNDIDADE_DE_BUSCA_DE_NOTAS: u8 = 12;

/// Uma nota do cofre: nome sem extensão e caminho completo.
#[derive(Serialize)]
pub struct NotaIndice {
    nome: String,
    caminho: String,
}

/// Verdadeiro se o caminho termina em `.md`, sem diferenciar maiúsculas.
pub(crate) fn e_arquivo_de_nota(caminho: &Path) -> bool {
    caminho
        .extension()
        .map(|extensao| extensao.eq_ignore_ascii_case("md"))
        .unwrap_or(false)
}

/// Nome do arquivo sem a extensão. Ex.: `ideia.md` vira `ideia`.
pub(crate) fn nome_sem_extensao(caminho: &Path) -> String {
    caminho
        .file_stem()
        .map(|nome| nome.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Junta em `saida` os caminhos de todas as notas dentro de `pasta`, descendo até
/// `profundidade` níveis. Ignora pastas ocultas e não segue links simbólicos.
pub(crate) fn coletar_caminhos_de_notas(pasta: &Path, profundidade: u8, saida: &mut Vec<PathBuf>) {
    if profundidade == 0 {
        return;
    }
    let Ok(entradas) = fs::read_dir(pasta) else {
        return;
    };
    for entrada in entradas.flatten() {
        if entrada.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        // `file_type` não segue links simbólicos, então um link nunca entra na busca.
        let Ok(tipo_da_entrada) = entrada.file_type() else {
            continue;
        };
        let caminho_da_entrada = entrada.path();
        if tipo_da_entrada.is_dir() {
            coletar_caminhos_de_notas(&caminho_da_entrada, profundidade - 1, saida);
        } else if tipo_da_entrada.is_file() && e_arquivo_de_nota(&caminho_da_entrada) {
            saida.push(caminho_da_entrada);
        }
    }
}

/// Todas as notas do cofre ativo, em ordem alfabética sem diferenciar maiúsculas.
pub(crate) fn listar_notas_do_cofre(app: &tauri::AppHandle) -> Result<Vec<NotaIndice>, String> {
    let raiz_do_cofre = raiz_do_cofre_ativo(app)?;
    let mut caminhos_encontrados = Vec::new();
    coletar_caminhos_de_notas(&raiz_do_cofre, PROFUNDIDADE_DE_BUSCA_DE_NOTAS, &mut caminhos_encontrados);

    let mut notas: Vec<NotaIndice> = caminhos_encontrados
        .into_iter()
        .map(|caminho| NotaIndice {
            nome: nome_sem_extensao(&caminho),
            caminho: texto_para_frontend(&caminho),
        })
        .collect();
    notas.sort_by_key(|nota| nota.nome.to_lowercase());
    Ok(notas)
}
