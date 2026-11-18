//! Leitura das notas do cofre para o grafo: acha os arquivos .md, descarta os de sistema
//! e extrai os wikilinks de cada um. Não monta ligações nem posições.

use super::contrato::TipoDoNo;
use super::ligacoes::NotaLidaParaOGrafo;
use crate::cofre::caminhos::texto_para_frontend;
use crate::notas::indice::{
    arquivo_e_grande_demais, coletar_caminhos_de_notas, nome_sem_extensao, PROFUNDIDADE_DE_BUSCA_DE_NOTAS,
};
use crate::notas::wikilinks::extrair_alvos_dos_wikilinks;
use std::fs;
use std::path::Path;

/// Pastas do app que guardam configuração e segredos. Nunca aparecem no grafo.
const PASTAS_DE_SISTEMA_FORA_DO_GRAFO: [&str; 2] = ["_sistema", "_segredos"];

/// Pasta cujos arquivos são tarefas.
const PASTA_DAS_TAREFAS: &str = "tarefas";

/// Regras dos agentes que cofres antigos guardam na raiz. É arquivo de sistema, não nota.
const NOME_DO_ARQUIVO_DE_REGRAS_NA_RAIZ: &str = "AGENTS";

/// Todas as notas e tarefas do cofre, já lidas, em ordem alfabética do nome (e do caminho, no empate).
pub(crate) fn ler_notas_do_cofre(raiz_do_cofre: &Path) -> Vec<NotaLidaParaOGrafo> {
    let mut caminhos_das_notas = Vec::new();
    coletar_caminhos_de_notas(raiz_do_cofre, PROFUNDIDADE_DE_BUSCA_DE_NOTAS, &mut caminhos_das_notas);

    let mut notas: Vec<NotaLidaParaOGrafo> = caminhos_das_notas
        .iter()
        .filter_map(|caminho| {
            let pasta = pasta_relativa_da_nota(raiz_do_cofre, caminho)?;
            let nome = nome_sem_extensao(caminho);
            if e_arquivo_de_sistema(&pasta, &nome) {
                return None;
            }
            Some(NotaLidaParaOGrafo {
                caminho: texto_para_frontend(caminho),
                tipo: tipo_da_nota(&pasta),
                alvos_dos_wikilinks: ler_alvos_dos_wikilinks(caminho),
                nome,
                pasta,
            })
        })
        .collect();
    notas.sort_by(|primeira, segunda| {
        (primeira.nome.to_lowercase(), &primeira.caminho).cmp(&(segunda.nome.to_lowercase(), &segunda.caminho))
    });
    notas
}

/// Pasta da nota relativa ao cofre, com `/` entre os níveis. Vazio para notas na raiz.
fn pasta_relativa_da_nota(raiz_do_cofre: &Path, caminho_da_nota: &Path) -> Option<String> {
    let pasta_da_nota = caminho_da_nota.strip_prefix(raiz_do_cofre).ok()?.parent()?;
    let niveis: Vec<String> = pasta_da_nota
        .components()
        .map(|nivel| nivel.as_os_str().to_string_lossy().into_owned())
        .collect();
    Some(niveis.join("/"))
}

/// Primeiro nível da pasta relativa. Vazio para notas na raiz.
fn primeiro_nivel_da_pasta(pasta_relativa: &str) -> &str {
    pasta_relativa.split('/').next().unwrap_or("")
}

/// Verdadeiro para arquivos que o app usa por dentro e que não são notas do usuário.
fn e_arquivo_de_sistema(pasta_relativa: &str, nome_da_nota: &str) -> bool {
    let primeiro_nivel = primeiro_nivel_da_pasta(pasta_relativa);
    PASTAS_DE_SISTEMA_FORA_DO_GRAFO.contains(&primeiro_nivel)
        || (pasta_relativa.is_empty() && nome_da_nota == NOME_DO_ARQUIVO_DE_REGRAS_NA_RAIZ)
}

/// Tarefa se está dentro de `tarefas/`; nota em qualquer outro lugar.
fn tipo_da_nota(pasta_relativa: &str) -> TipoDoNo {
    if primeiro_nivel_da_pasta(pasta_relativa) == PASTA_DAS_TAREFAS {
        TipoDoNo::Tarefa
    } else {
        TipoDoNo::Nota
    }
}

/// Alvos de todos os wikilinks do arquivo. Arquivo grande demais ou ilegível entra no grafo sem links.
fn ler_alvos_dos_wikilinks(caminho_da_nota: &Path) -> Vec<String> {
    if arquivo_e_grande_demais(caminho_da_nota) {
        return Vec::new();
    }
    let Ok(texto_da_nota) = fs::read_to_string(caminho_da_nota) else {
        return Vec::new();
    };
    texto_da_nota.lines().flat_map(extrair_alvos_dos_wikilinks).collect()
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn pasta_relativa_usa_barra_e_fica_vazia_na_raiz() {
        let raiz = Path::new("cofre");
        assert_eq!(pasta_relativa_da_nota(raiz, &raiz.join("notas").join("ideias").join("a.md")).unwrap(), "notas/ideias");
        assert_eq!(pasta_relativa_da_nota(raiz, &raiz.join("a.md")).unwrap(), "");
        assert!(pasta_relativa_da_nota(raiz, Path::new("outro/a.md")).is_none());
    }

    #[test]
    fn arquivos_de_sistema_sao_reconhecidos() {
        assert!(e_arquivo_de_sistema("_sistema", "AGENTS"));
        assert!(e_arquivo_de_sistema("_sistema/conflitos", "copia"));
        assert!(e_arquivo_de_sistema("_segredos", "senhas"));
        assert!(e_arquivo_de_sistema("", "AGENTS"));
        assert!(!e_arquivo_de_sistema("notas", "AGENTS"));
        assert!(!e_arquivo_de_sistema("", "ideia"));
    }

    #[test]
    fn so_o_que_esta_em_tarefas_e_tarefa() {
        assert_eq!(tipo_da_nota("tarefas"), TipoDoNo::Tarefa);
        assert_eq!(tipo_da_nota("tarefas/arquivadas"), TipoDoNo::Tarefa);
        assert_eq!(tipo_da_nota("notas/tarefas"), TipoDoNo::Nota);
        assert_eq!(tipo_da_nota(""), TipoDoNo::Nota);
    }
}
