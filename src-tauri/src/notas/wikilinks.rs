//! Leitura de wikilinks `[[...]]` dentro de uma linha de texto.
//! Função pura: não toca no disco.

/// Lista os alvos de todos os `[[...]]` de uma linha, já normalizados:
/// minúsculos, sem `|alias`, sem `#seção`, sem pasta e sem `.md`.
///
/// Exemplos: `[[Projeto X]]` vira `projeto x`; `[[pasta/nota.md#topo|apelido]]` vira `nota`.
pub(crate) fn extrair_alvos_dos_wikilinks(linha: &str) -> Vec<String> {
    let mut alvos_encontrados = Vec::new();
    let mut restante_da_linha = linha;

    while let Some(posicao_da_abertura) = restante_da_linha.find("[[") {
        let depois_da_abertura = &restante_da_linha[posicao_da_abertura + 2..];
        // Sem `]]` de fechamento, o colchete não é um wikilink. Para de procurar.
        let Some(posicao_do_fechamento) = depois_da_abertura.find("]]") else {
            break;
        };
        let conteudo_do_link = &depois_da_abertura[..posicao_do_fechamento];
        let alvo = normalizar_alvo(conteudo_do_link);
        if !alvo.is_empty() {
            alvos_encontrados.push(alvo);
        }
        restante_da_linha = &depois_da_abertura[posicao_do_fechamento + 2..];
    }
    alvos_encontrados
}

/// Reduz o conteúdo de um wikilink ao nome da nota que ele aponta.
fn normalizar_alvo(conteudo_do_link: &str) -> String {
    let antes_do_alias = conteudo_do_link.split('|').next().unwrap_or("");
    let antes_da_secao = antes_do_alias.split('#').next().unwrap_or("").trim();
    let ultimo_trecho_do_caminho = antes_da_secao.rsplit('/').next().unwrap_or(antes_da_secao).trim();
    let minusculo = ultimo_trecho_do_caminho.to_lowercase();
    minusculo.strip_suffix(".md").unwrap_or(&minusculo).to_string()
}

#[cfg(test)]
mod testes {
    use super::extrair_alvos_dos_wikilinks;

    #[test]
    fn extrai_nome_em_minusculas() {
        assert_eq!(extrair_alvos_dos_wikilinks("veja [[Projeto X]] agora"), vec!["projeto x"]);
    }

    #[test]
    fn ignora_alias_secao_e_pasta() {
        assert_eq!(extrair_alvos_dos_wikilinks("[[nota|apelido]]"), vec!["nota"]);
        assert_eq!(extrair_alvos_dos_wikilinks("[[nota#secao]]"), vec!["nota"]);
        assert_eq!(extrair_alvos_dos_wikilinks("[[pasta/nota.md]]"), vec!["nota"]);
    }

    #[test]
    fn ignora_colchete_sem_fechamento_e_link_vazio() {
        assert!(extrair_alvos_dos_wikilinks("[[sem fim").is_empty());
        assert!(extrair_alvos_dos_wikilinks("[[]]").is_empty());
    }

    #[test]
    fn encontra_varios_links_na_mesma_linha() {
        assert_eq!(extrair_alvos_dos_wikilinks("[[a]] e [[b]]"), vec!["a", "b"]);
    }
}
