//! Consultas de leitura sobre as notas do cofre, usadas pelo servidor MCP: buscar texto e ler uma nota.
//! Trabalham a partir da raiz do cofre, sem `AppHandle`, e não escrevem nada.
//! Ignoram as pastas reservadas (`_sistema/` com regras e logs, `_segredos/` com senhas).

use super::indice::{arquivo_e_grande_demais, coletar_caminhos_de_notas, e_arquivo_de_nota, nome_sem_extensao, PROFUNDIDADE_DE_BUSCA_DE_NOTAS};
use crate::cofre::caminhos::caminho_dentro_da_raiz;
use serde::Serialize;
use std::fs;
use std::path::Path;

/// Quantas notas a busca devolve no máximo. Para o texto inteiro, o agente usa `ler_nota`.
const LIMITE_DE_RESULTADOS_DA_BUSCA: usize = 20;

/// Tamanho máximo do trecho mostrado por nota, em caracteres.
const TAMANHO_DO_TRECHO: usize = 160;

/// Uma nota encontrada: nome, caminho relativo à raiz do cofre (com `/`) e a linha que casou.
/// `trecho` fica vazio quando a nota casou só pelo nome.
#[derive(Serialize)]
pub(crate) struct ResultadoDaBusca {
    pub nome: String,
    pub caminho: String,
    pub trecho: String,
}

/// Procura `termo` (sem diferenciar maiúsculas) no nome e no texto das notas de `raiz`.
/// Devolve no máximo `LIMITE_DE_RESULTADOS_DA_BUSCA` notas, em ordem de caminho.
pub(crate) fn buscar_notas_da_raiz(raiz: &Path, termo: &str) -> Result<Vec<ResultadoDaBusca>, String> {
    let termo_limpo = termo.trim();
    if termo_limpo.is_empty() {
        return Err("o termo da busca não pode ficar vazio".into());
    }
    let termo_minusculo = termo_limpo.to_lowercase();

    let mut caminhos = Vec::new();
    coletar_caminhos_de_notas(raiz, PROFUNDIDADE_DE_BUSCA_DE_NOTAS, &mut caminhos);
    caminhos.sort();

    let mut resultados = Vec::new();
    for caminho in caminhos {
        let Ok(caminho_relativo) = caminho.strip_prefix(raiz) else {
            continue;
        };
        if esta_em_pasta_reservada(caminho_relativo) || arquivo_e_grande_demais(&caminho) {
            continue;
        }
        let Ok(texto) = fs::read_to_string(&caminho) else {
            continue;
        };
        let nome = nome_sem_extensao(&caminho);
        let casa_pelo_nome = nome.to_lowercase().contains(&termo_minusculo);
        let trecho = trecho_que_casa(&texto, termo_limpo);
        if !casa_pelo_nome && trecho.is_none() {
            continue;
        }
        resultados.push(ResultadoDaBusca {
            nome,
            caminho: caminho_relativo.to_string_lossy().replace('\\', "/"),
            trecho: trecho.unwrap_or_default(),
        });
        if resultados.len() == LIMITE_DE_RESULTADOS_DA_BUSCA {
            break;
        }
    }
    Ok(resultados)
}

/// Texto de uma nota, pelo caminho relativo à raiz (como o devolvido pela busca).
/// Recusa caminhos que saem do cofre e arquivos que não são `.md`.
pub(crate) fn ler_nota_da_raiz(raiz: &Path, caminho_relativo: &str) -> Result<String, String> {
    let caminho_da_nota = caminho_dentro_da_raiz(raiz, Path::new(caminho_relativo))?;
    if !e_arquivo_de_nota(&caminho_da_nota) {
        return Err("não é uma nota .md".into());
    }
    fs::read_to_string(&caminho_da_nota).map_err(|erro| format!("não foi possível ler a nota: {erro}"))
}

/// Primeira linha do texto que contém o termo, sem diferenciar maiúsculas, cortada no tamanho do trecho.
/// Função pura: não toca no disco.
pub(crate) fn trecho_que_casa(texto: &str, termo: &str) -> Option<String> {
    let termo_minusculo = termo.to_lowercase();
    texto
        .lines()
        .find(|linha| linha.to_lowercase().contains(&termo_minusculo))
        .map(|linha| linha.trim().chars().take(TAMANHO_DO_TRECHO).collect())
}

/// Verdadeiro para arquivos dentro de uma pasta que começa com `_` (reservada do sistema).
/// Um arquivo solto na raiz do cofre não conta como pasta reservada.
fn esta_em_pasta_reservada(caminho_relativo: &Path) -> bool {
    let componentes: Vec<_> = caminho_relativo.components().collect();
    componentes.len() > 1
        && componentes[0].as_os_str().to_string_lossy().starts_with('_')
}

#[cfg(test)]
mod testes {
    use super::*;
    use std::path::PathBuf;

    fn cofre_de_teste(nome: &str) -> PathBuf {
        let raiz = std::env::temp_dir().join(format!("segundo-cerebro-consulta-{nome}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&raiz);
        fs::create_dir_all(raiz.join("_sistema")).unwrap();
        fs::create_dir_all(raiz.join("_segredos")).unwrap();
        fs::create_dir_all(raiz.join("notas").join("ideias")).unwrap();
        raiz.canonicalize().unwrap()
    }

    #[test]
    fn trecho_ignora_maiusculas_e_devolve_a_linha() {
        let texto = "Título\nA Horta cresce no inverno\nfim";
        assert_eq!(trecho_que_casa(texto, "horta").as_deref(), Some("A Horta cresce no inverno"));
        assert!(trecho_que_casa(texto, "praia").is_none());
    }

    #[test]
    fn trecho_e_cortado_no_tamanho_maximo() {
        let linha = "x".repeat(500);
        let trecho = trecho_que_casa(&linha, "x").unwrap();
        assert_eq!(trecho.chars().count(), TAMANHO_DO_TRECHO);
    }

    #[test]
    fn busca_acha_por_texto_e_por_nome_e_pula_pastas_reservadas() {
        let raiz = cofre_de_teste("busca");
        fs::write(raiz.join("notas").join("ideias").join("horta.md"), "Plantar tomate").unwrap();
        fs::write(raiz.join("notas").join("receita.md"), "Bolo de horta").unwrap();
        fs::write(raiz.join("_sistema").join("AGENTS.md"), "horta nas regras").unwrap();
        fs::write(raiz.join("_segredos").join("horta.md"), "senha da horta").unwrap();

        let resultados = buscar_notas_da_raiz(&raiz, "horta").unwrap();
        let caminhos: Vec<&str> = resultados.iter().map(|resultado| resultado.caminho.as_str()).collect();

        assert_eq!(caminhos, vec!["notas/ideias/horta.md", "notas/receita.md"]);
        // Casou só pelo nome: sem trecho. Casou pelo texto: mostra a linha.
        assert_eq!(resultados[0].trecho, "");
        assert_eq!(resultados[1].trecho, "Bolo de horta");
    }

    #[test]
    fn busca_vazia_e_recusada() {
        let raiz = cofre_de_teste("vazia");
        assert!(buscar_notas_da_raiz(&raiz, "   ").is_err());
    }

    #[test]
    fn ler_nota_recusa_caminho_que_sai_do_cofre() {
        let raiz = cofre_de_teste("escapa");
        let erro = ler_nota_da_raiz(&raiz, "../fora.md").unwrap_err();
        assert_eq!(erro, "caminho fora do cofre");
    }

    #[test]
    fn ler_nota_devolve_o_texto_de_uma_nota() {
        let raiz = cofre_de_teste("ler");
        fs::write(raiz.join("notas").join("receita.md"), "# Receita\n\nFarinha").unwrap();
        let texto = ler_nota_da_raiz(&raiz, "notas/receita.md").unwrap();
        assert!(texto.contains("Farinha"));
    }
}
