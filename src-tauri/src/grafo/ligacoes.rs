//! Regra das ligações do grafo: decide quais wikilinks viram uma linha entre duas notas.
//! Função pura: recebe as notas já lidas e não toca no disco.

use super::contrato::{LigacaoDoGrafo, TipoDoNo};
use std::collections::{HashMap, HashSet};

/// Uma nota já lida, com os alvos dos wikilinks que ela contém (minúsculos, sem alias nem pasta).
pub(crate) struct NotaLidaParaOGrafo {
    pub caminho: String,
    pub nome: String,
    pub pasta: String,
    pub tipo: TipoDoNo,
    pub alvos_dos_wikilinks: Vec<String>,
}

/// Liga cada nota às notas que ela cita por wikilink e conta as ligações de cada nota.
///
/// - Nome repetido em pastas diferentes: vale a primeira nota na ordem recebida, a mesma
///   regra do clique no wikilink.
/// - Link para nota que não existe, link para a própria nota e link repetido não geram ligação.
pub(crate) fn ligar_notas(notas: &[NotaLidaParaOGrafo]) -> (Vec<LigacaoDoGrafo>, Vec<usize>) {
    let mut posicao_da_nota_por_nome: HashMap<String, usize> = HashMap::new();
    for (posicao, nota) in notas.iter().enumerate() {
        posicao_da_nota_por_nome.entry(nota.nome.to_lowercase()).or_insert(posicao);
    }

    let mut ligacoes_ja_criadas: HashSet<(usize, usize)> = HashSet::new();
    let mut ligacoes = Vec::new();
    let mut conexoes_por_nota = vec![0usize; notas.len()];

    for (origem, nota) in notas.iter().enumerate() {
        for alvo in &nota.alvos_dos_wikilinks {
            let Some(&destino) = posicao_da_nota_por_nome.get(alvo) else {
                continue;
            };
            if destino == origem || !ligacoes_ja_criadas.insert((origem, destino)) {
                continue;
            }
            ligacoes.push(LigacaoDoGrafo { origem, destino });
            conexoes_por_nota[origem] += 1;
            conexoes_por_nota[destino] += 1;
        }
    }
    (ligacoes, conexoes_por_nota)
}

#[cfg(test)]
mod testes {
    use super::*;

    fn nota(nome: &str, alvos: &[&str]) -> NotaLidaParaOGrafo {
        NotaLidaParaOGrafo {
            caminho: format!("cofre/{nome}.md"),
            nome: nome.to_string(),
            pasta: String::new(),
            tipo: TipoDoNo::Nota,
            alvos_dos_wikilinks: alvos.iter().map(|alvo| alvo.to_string()).collect(),
        }
    }

    #[test]
    fn liga_sem_diferenciar_maiusculas_e_conta_as_duas_pontas() {
        let notas = [nota("Ideia", &["projeto"]), nota("Projeto", &[])];
        let (ligacoes, conexoes) = ligar_notas(&notas);
        assert_eq!(ligacoes, vec![LigacaoDoGrafo { origem: 0, destino: 1 }]);
        assert_eq!(conexoes, vec![1, 1]);
    }

    #[test]
    fn ignora_link_para_nota_inexistente_para_si_mesma_e_repetido() {
        let notas = [nota("a", &["a", "nao-existe", "b", "b"]), nota("b", &[])];
        let (ligacoes, conexoes) = ligar_notas(&notas);
        assert_eq!(ligacoes.len(), 1);
        assert_eq!(conexoes, vec![1, 1]);
    }

    #[test]
    fn nome_repetido_vale_a_primeira_nota() {
        let notas = [nota("x", &["y"]), nota("y", &[]), nota("y", &[])];
        let (ligacoes, _) = ligar_notas(&notas);
        assert_eq!(ligacoes, vec![LigacaoDoGrafo { origem: 0, destino: 1 }]);
    }

    #[test]
    fn links_nos_dois_sentidos_viram_duas_ligacoes() {
        let notas = [nota("a", &["b"]), nota("b", &["a"])];
        let (ligacoes, conexoes) = ligar_notas(&notas);
        assert_eq!(ligacoes.len(), 2);
        assert_eq!(conexoes, vec![2, 2]);
    }
}
