//! Compara a foto do cofre de antes com a de depois e lista as mudanças.
//! Função pura: não lê nem grava arquivos, então é testada só com fotos montadas na memória.

use super::contrato::{MudancaNoCofre, TipoDeMudanca};
use super::foto::FotoDoCofre;

/// Lista o que mudou entre as duas fotos, em ordem de caminho.
pub fn comparar_fotos(antes: &FotoDoCofre, depois: &FotoDoCofre) -> Vec<MudancaNoCofre> {
    let mut mudancas = Vec::new();

    for (caminho, texto_de_antes) in &antes.notas {
        match depois.notas.get(caminho) {
            None => mudancas.push(mudanca_de_nota(caminho, TipoDeMudanca::Apagado, Some(texto_de_antes), None)),
            Some(texto_de_depois) if texto_de_depois != texto_de_antes => mudancas.push(mudanca_de_nota(
                caminho,
                TipoDeMudanca::Alterado,
                Some(texto_de_antes),
                Some(texto_de_depois),
            )),
            Some(_) => {}
        }
    }

    for (caminho, texto_de_depois) in &depois.notas {
        if !antes.notas.contains_key(caminho) {
            mudancas.push(mudanca_de_nota(caminho, TipoDeMudanca::Criado, None, Some(texto_de_depois)));
        }
    }

    for pasta in antes.pastas.difference(&depois.pastas) {
        mudancas.push(mudanca_de_pasta(pasta, TipoDeMudanca::PastaApagada));
    }
    for pasta in depois.pastas.difference(&antes.pastas) {
        mudancas.push(mudanca_de_pasta(pasta, TipoDeMudanca::PastaCriada));
    }

    mudancas.sort_by(|primeira, segunda| primeira.caminho.cmp(&segunda.caminho));
    mudancas
}

/// Mudança de uma nota, com o texto de antes e de depois (None quando não existia).
fn mudanca_de_nota(caminho: &str, tipo: TipoDeMudanca, antes: Option<&String>, depois: Option<&String>) -> MudancaNoCofre {
    MudancaNoCofre {
        caminho: caminho.to_string(),
        tipo,
        antes: antes.cloned(),
        depois: depois.cloned(),
    }
}

/// Mudança de pasta. Pasta não tem texto, então `antes` e `depois` ficam vazios.
fn mudanca_de_pasta(caminho: &str, tipo: TipoDeMudanca) -> MudancaNoCofre {
    MudancaNoCofre {
        caminho: caminho.to_string(),
        tipo,
        antes: None,
        depois: None,
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn foto_com_nota(caminho: &str, texto: &str) -> FotoDoCofre {
        let mut foto = FotoDoCofre::default();
        foto.notas.insert(caminho.to_string(), texto.to_string());
        foto
    }

    #[test]
    fn nota_nova_aparece_como_criada() {
        let mudancas = comparar_fotos(&FotoDoCofre::default(), &foto_com_nota("tarefas/a.md", "oi"));
        assert_eq!(mudancas.len(), 1);
        assert_eq!(mudancas[0].tipo, TipoDeMudanca::Criado);
        assert_eq!(mudancas[0].antes, None);
        assert_eq!(mudancas[0].depois.as_deref(), Some("oi"));
    }

    #[test]
    fn texto_trocado_guarda_antes_e_depois() {
        let mudancas = comparar_fotos(&foto_com_nota("n.md", "antes"), &foto_com_nota("n.md", "depois"));
        assert_eq!(mudancas[0].tipo, TipoDeMudanca::Alterado);
        assert_eq!(mudancas[0].antes.as_deref(), Some("antes"));
        assert_eq!(mudancas[0].depois.as_deref(), Some("depois"));
    }

    #[test]
    fn nota_igual_nao_aparece() {
        assert!(comparar_fotos(&foto_com_nota("n.md", "x"), &foto_com_nota("n.md", "x")).is_empty());
    }

    #[test]
    fn nota_removida_e_pasta_criada_aparecem() {
        let antes = foto_com_nota("n.md", "x");
        let mut depois = FotoDoCofre::default();
        depois.pastas.insert("projetos".to_string());
        let mudancas = comparar_fotos(&antes, &depois);
        let tipos: Vec<TipoDeMudanca> = mudancas.iter().map(|mudanca| mudanca.tipo).collect();
        assert_eq!(tipos, vec![TipoDeMudanca::Apagado, TipoDeMudanca::PastaCriada]);
    }
}
