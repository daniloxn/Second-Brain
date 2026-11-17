//! Frontmatter: o bloco `---` no topo de um arquivo Markdown, com linhas `chave: valor`.
//! Lê e grava esse bloco mantendo a ordem e o texto original de cada valor, para que
//! chaves que o app não conhece sobrevivam a uma gravação.
//! Não sabe o que cada chave significa: isso fica no módulo de domínio.

/// Texto de um arquivo dividido em pares do frontmatter e corpo.
#[derive(Debug, Clone, PartialEq)]
pub struct Frontmatter {
    /// Pares `(chave, valor)` na ordem em que aparecem. O valor fica como está no arquivo.
    pub pares: Vec<(String, String)>,
    /// Tudo que vem depois do bloco `---`, exatamente como estava.
    pub corpo: String,
}

/// Linha que abre e fecha o bloco de frontmatter.
const LINHA_DELIMITADORA: &str = "---";

/// Separa o frontmatter do corpo. Sem bloco `---` no início (ou sem o fechamento),
/// o texto inteiro vira corpo e não há pares.
pub fn separar(texto_do_arquivo: &str) -> Frontmatter {
    let Some(resto_apos_abertura) = texto_do_arquivo.strip_prefix("---\n") else {
        return sem_frontmatter(texto_do_arquivo);
    };

    let mut posicao_da_linha = 0;
    for linha in resto_apos_abertura.split_inclusive('\n') {
        let texto_da_linha = linha.trim_end_matches(['\r', '\n']);
        if texto_da_linha == LINHA_DELIMITADORA {
            let bloco = &resto_apos_abertura[..posicao_da_linha];
            let corpo = &resto_apos_abertura[posicao_da_linha + linha.len()..];
            return Frontmatter {
                pares: ler_pares(bloco),
                corpo: corpo.to_string(),
            };
        }
        posicao_da_linha += linha.len();
    }

    // Abriu o bloco mas não fechou: não é frontmatter, então é tudo corpo.
    sem_frontmatter(texto_do_arquivo)
}

/// Monta o texto final: bloco `---` com os pares, a linha de fechamento e o corpo.
pub fn montar(frontmatter: &Frontmatter) -> String {
    let mut texto = String::from("---\n");
    for (chave, valor) in &frontmatter.pares {
        texto.push_str(&format!("{chave}: {valor}\n"));
    }
    texto.push_str(LINHA_DELIMITADORA);
    texto.push('\n');
    texto.push_str(&frontmatter.corpo);
    texto
}

/// Envolve um texto em aspas duplas, escapando `\` e `"`, para que dois-pontos e
/// colchetes dentro de um título não quebrem a leitura.
pub fn citar(texto: &str) -> String {
    let escapado = texto.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escapado}\"")
}

/// Desfaz `citar`. Se o texto não estiver entre aspas, devolve como está.
pub fn descitar(bruto: &str) -> String {
    let limpo = bruto.trim();
    let Some(interno) = limpo
        .strip_prefix('"')
        .and_then(|resto| resto.strip_suffix('"'))
    else {
        return limpo.to_string();
    };

    let mut resultado = String::new();
    let mut caractere_escapado = false;
    for caractere in interno.chars() {
        if caractere_escapado {
            resultado.push(caractere);
            caractere_escapado = false;
        } else if caractere == '\\' {
            caractere_escapado = true;
        } else {
            resultado.push(caractere);
        }
    }
    resultado
}

fn sem_frontmatter(texto: &str) -> Frontmatter {
    Frontmatter {
        pares: Vec::new(),
        corpo: texto.to_string(),
    }
}

/// Lê as linhas do bloco. Linhas sem `:` ou sem chave são ignoradas.
fn ler_pares(bloco: &str) -> Vec<(String, String)> {
    bloco
        .lines()
        .filter_map(|linha| {
            let (chave, valor) = linha.split_once(':')?;
            let chave_limpa = chave.trim();
            if chave_limpa.is_empty() {
                return None;
            }
            Some((chave_limpa.to_string(), valor.trim().to_string()))
        })
        .collect()
}

#[cfg(test)]
mod testes {
    use super::{citar, descitar, montar, separar, Frontmatter};

    #[test]
    fn texto_sem_bloco_vira_corpo_inteiro() {
        let separado = separar("# Título\n\nTexto");
        assert!(separado.pares.is_empty());
        assert_eq!(separado.corpo, "# Título\n\nTexto");
    }

    #[test]
    fn separa_pares_e_corpo() {
        let separado = separar("---\nstatus: aberta\nprazo: 2026-11-18\n---\nCorpo da tarefa");
        assert_eq!(
            separado.pares,
            vec![
                ("status".to_string(), "aberta".to_string()),
                ("prazo".to_string(), "2026-11-18".to_string()),
            ]
        );
        assert_eq!(separado.corpo, "Corpo da tarefa");
    }

    #[test]
    fn bloco_sem_fechamento_nao_e_frontmatter() {
        let separado = separar("---\nstatus: aberta\nsem fim");
        assert!(separado.pares.is_empty());
    }

    #[test]
    fn ida_e_volta_preserva_corpo_exatamente() {
        let original = "---\ntitulo: \"A\"\n---\n\nLinha 1\nLinha 2\n";
        let separado = separar(original);
        assert_eq!(montar(&separado), original);
    }

    #[test]
    fn citar_e_descitar_sao_inversos() {
        let titulo = "Relatório: \"final\" \\ v2";
        assert_eq!(descitar(&citar(titulo)), titulo);
    }

    #[test]
    fn descitar_devolve_texto_sem_aspas_como_esta() {
        assert_eq!(descitar("  sem aspas  "), "sem aspas");
    }

    #[test]
    fn montar_escreve_pares_na_ordem() {
        let frontmatter = Frontmatter {
            pares: vec![("a".into(), "1".into()), ("b".into(), "2".into())],
            corpo: "x".into(),
        };
        assert_eq!(montar(&frontmatter), "---\na: 1\nb: 2\n---\nx");
    }
}
