//! Monta o grafo do cofre: lê as notas, liga por wikilink, posiciona na galáxia e
//! devolve o contrato. Cada passo tem seu módulo; aqui só se encadeia.

use super::contrato::{GrafoDoCofre, NoDoGrafo};
use super::layout_galaxia::{posicionar_na_galaxia, EntradaDeLayout};
use super::leitura::ler_notas_do_cofre;
use super::ligacoes::{ligar_notas, NotaLidaParaOGrafo};
use super::volume_da_galaxia::volume_da_galaxia;
use std::collections::BTreeMap;
use std::path::Path;

/// Máximo de pontos no grafo. Acima disso o desenho fica pesado e ilegível; o excesso é cortado.
const LIMITE_DE_NOS_NO_GRAFO: usize = 1500;

/// Grafo completo do cofre em `raiz_do_cofre`.
pub(crate) fn montar_grafo_do_cofre(raiz_do_cofre: &Path) -> GrafoDoCofre {
    let mut notas = ler_notas_do_cofre(raiz_do_cofre);
    let truncado = notas.len() > LIMITE_DE_NOS_NO_GRAFO;
    notas.truncate(LIMITE_DE_NOS_NO_GRAFO);

    let (ligacoes, conexoes_por_nota) = ligar_notas(&notas);
    let pares_ligados: Vec<(usize, usize)> = ligacoes.iter().map(|ligacao| (ligacao.origem, ligacao.destino)).collect();
    let posicoes = posicionar_na_galaxia(&entradas_de_layout(&notas), &pares_ligados);

    let nos = notas
        .into_iter()
        .zip(conexoes_por_nota)
        .zip(posicoes)
        .map(|((nota, conexoes), posicao)| NoDoGrafo {
            caminho: nota.caminho,
            nome: nota.nome,
            pasta: nota.pasta,
            tipo: nota.tipo,
            conexoes,
            posicao,
        })
        .collect();
    GrafoDoCofre { nos, ligacoes, volume: volume_da_galaxia(), truncado }
}

/// Um grupo por pasta, numerados pela ordem alfabética das pastas. A semente vem da pasta e do nome,
/// não do caminho completo, então a posição não depende de onde o cofre está no disco.
fn entradas_de_layout(notas: &[NotaLidaParaOGrafo]) -> Vec<EntradaDeLayout> {
    let grupo_por_pasta: BTreeMap<&str, usize> = notas
        .iter()
        .map(|nota| nota.pasta.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .map(|(grupo, pasta)| (pasta, grupo))
        .collect();

    notas
        .iter()
        .map(|nota| EntradaDeLayout {
            semente: semente_da_nota(&nota.pasta, &nota.nome),
            grupo: grupo_por_pasta[nota.pasta.as_str()],
        })
        .collect()
}

/// Número estável a partir da pasta e do nome (FNV-1a de 64 bits).
fn semente_da_nota(pasta: &str, nome: &str) -> u64 {
    let mut semente: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in pasta.bytes().chain(std::iter::once(b'/')).chain(nome.bytes()) {
        semente ^= u64::from(byte);
        semente = semente.wrapping_mul(0x0000_0100_0000_01b3);
    }
    semente
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::grafo::contrato::TipoDoNo;
    use crate::grafo::volume_da_galaxia::esta_dentro_da_galaxia;
    use std::fs;
    use std::path::PathBuf;

    /// Cofre de teste numa pasta temporária própria deste teste. Quem chama apaga no fim.
    fn criar_cofre_de_teste(nome_do_teste: &str, arquivos: &[(&str, &str)]) -> PathBuf {
        let raiz = std::env::temp_dir().join(format!("segundo-cerebro-grafo-{nome_do_teste}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&raiz);
        for (caminho_relativo, conteudo) in arquivos {
            let caminho = raiz.join(caminho_relativo);
            fs::create_dir_all(caminho.parent().unwrap()).unwrap();
            fs::write(caminho, conteudo).unwrap();
        }
        raiz
    }

    #[test]
    fn grafo_do_cofre_liga_notas_separa_tarefas_e_esconde_arquivos_de_sistema() {
        let raiz = criar_cofre_de_teste(
            "completo",
            &[
                ("notas/a.md", "veja [[B]] e [[inexistente]]"),
                ("notas/b.md", "volta para [[a]]"),
                ("notas/solta.md", "sem links"),
                ("tarefas/t.md", "---\ntitulo: x\n---\nver [[a]]"),
                ("_sistema/regras.md", "[[a]]"),
                ("AGENTS.md", "[[a]]"),
            ],
        );
        let grafo = montar_grafo_do_cofre(&raiz);
        fs::remove_dir_all(&raiz).unwrap();

        let nomes: Vec<&str> = grafo.nos.iter().map(|no| no.nome.as_str()).collect();
        assert_eq!(nomes, vec!["a", "b", "solta", "t"]);
        assert_eq!(grafo.ligacoes.len(), 3, "a→b, b→a e t→a");
        let conexoes: Vec<usize> = grafo.nos.iter().map(|no| no.conexoes).collect();
        assert_eq!(conexoes, vec![3, 2, 0, 1]);
        assert_eq!(grafo.nos[3].tipo, TipoDoNo::Tarefa);
        assert_eq!(grafo.nos[0].pasta, "notas");
        assert!(!grafo.truncado);
        assert!(grafo.nos.iter().all(|no| esta_dentro_da_galaxia(no.posicao)));
    }

    #[test]
    fn cofre_sem_notas_devolve_grafo_vazio() {
        let raiz = criar_cofre_de_teste("vazio", &[("_sistema/regras.md", "x")]);
        let grafo = montar_grafo_do_cofre(&raiz);
        fs::remove_dir_all(&raiz).unwrap();
        assert!(grafo.nos.is_empty() && grafo.ligacoes.is_empty());
    }

    #[test]
    fn a_semente_depende_so_da_pasta_e_do_nome() {
        assert_eq!(semente_da_nota("notas", "a"), semente_da_nota("notas", "a"));
        assert_ne!(semente_da_nota("notas", "a"), semente_da_nota("notas", "b"));
        assert_ne!(semente_da_nota("notas", "a"), semente_da_nota("outra", "a"));
    }
}
