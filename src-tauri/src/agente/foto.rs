//! Foto do cofre: o texto de cada nota `.md` e a lista de pastas em um momento.
//! Comparando a foto de antes e de depois de um pedido, descobrimos o que o agente mudou,
//! mesmo quando ele escreveu direto no disco.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

/// Notas maiores que isso não entram na foto. Arquivo grande costuma ser anexo, não texto.
const TAMANHO_MAXIMO_DA_NOTA_EM_BYTES: u64 = 2 * 1024 * 1024;

/// Até que profundidade de pastas a foto desce.
const PROFUNDIDADE_MAXIMA_DA_FOTO: u8 = 12;

/// Estado do cofre em um momento. Caminhos são relativos à raiz, com `/` como separador.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FotoDoCofre {
    /// Caminho de cada nota e o texto dela.
    pub notas: BTreeMap<String, String>,
    /// Caminho de cada pasta.
    pub pastas: BTreeSet<String>,
}

/// Tira a foto do cofre inteiro. Itens ocultos (nome com ponto) ficam de fora,
/// assim como links simbólicos, para não sair da pasta do cofre.
pub fn tirar_foto(raiz_do_cofre: &Path) -> Result<FotoDoCofre, String> {
    let mut foto = FotoDoCofre::default();
    percorrer_pasta(raiz_do_cofre, raiz_do_cofre, 0, &mut foto)?;
    Ok(foto)
}

/// Desce pelas pastas e preenche a foto. Uma subpasta que não pode ser lida é pulada.
fn percorrer_pasta(raiz: &Path, pasta_atual: &Path, profundidade: u8, foto: &mut FotoDoCofre) -> Result<(), String> {
    if profundidade > PROFUNDIDADE_MAXIMA_DA_FOTO {
        return Ok(());
    }
    let entradas = fs::read_dir(pasta_atual).map_err(|erro| erro.to_string())?;
    for entrada in entradas.flatten() {
        if entrada.file_name().to_string_lossy().starts_with('.') {
            continue;
        }
        let Ok(tipo) = entrada.file_type() else {
            continue;
        };
        let caminho = entrada.path();
        let caminho_relativo = caminho_relativo_com_barras(raiz, &caminho);
        if tipo.is_dir() {
            foto.pastas.insert(caminho_relativo);
            // Subpasta ilegível é pulada; o resto da foto continua.
            let _ = percorrer_pasta(raiz, &caminho, profundidade + 1, foto);
        } else if tipo.is_file() && e_nota_de_texto(&caminho) {
            if let Some(texto) = ler_nota_pequena(&caminho) {
                foto.notas.insert(caminho_relativo, texto);
            }
        }
    }
    Ok(())
}

/// Só arquivos `.md` entram na foto. Os outros tipos não são comparados.
fn e_nota_de_texto(caminho: &Path) -> bool {
    caminho.extension().is_some_and(|extensao| extensao.eq_ignore_ascii_case("md"))
}

/// Texto da nota, ou None se ela for grande demais ou não estiver em UTF-8.
fn ler_nota_pequena(caminho: &Path) -> Option<String> {
    let tamanho = fs::metadata(caminho).ok()?.len();
    if tamanho > TAMANHO_MAXIMO_DA_NOTA_EM_BYTES {
        return None;
    }
    fs::read_to_string(caminho).ok()
}

/// Caminho relativo à raiz, com `/` entre as partes, para comparar fotos de qualquer sistema.
fn caminho_relativo_com_barras(raiz: &Path, caminho: &Path) -> String {
    caminho
        .strip_prefix(raiz)
        .map(|relativo| {
            relativo
                .components()
                .map(|parte| parte.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<String>>()
                .join("/")
        })
        .unwrap_or_default()
}
