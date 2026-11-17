//! Regras para o nome de uma nota nova. Função pura: não toca no disco,
//! então pode ser testada sozinha.

/// Caracteres que o Windows não aceita em nomes de arquivo.
const CARACTERES_PROIBIDOS_NO_NOME: &str = "<>:\"/\\|?*";

/// Confere e limpa o nome de uma nota nova. Devolve o nome sem espaços nas pontas.
/// Recusa nome vazio, nome começando com ponto e caracteres proibidos no Windows.
pub(crate) fn validar_nome_da_nota(nome_digitado: &str) -> Result<String, String> {
    let nome_limpo = nome_digitado.trim();
    if nome_limpo.is_empty() {
        return Err("o nome não pode ficar vazio".into());
    }
    if nome_limpo.starts_with('.') {
        return Err("o nome não pode começar com ponto".into());
    }
    let tem_caractere_proibido = nome_limpo
        .chars()
        .any(|caractere| CARACTERES_PROIBIDOS_NO_NOME.contains(caractere) || caractere.is_control());
    if tem_caractere_proibido {
        return Err("o nome tem caracteres que não são permitidos em arquivos".into());
    }
    Ok(nome_limpo.to_string())
}

#[cfg(test)]
mod testes {
    use super::validar_nome_da_nota;

    #[test]
    fn aceita_nome_comum_e_remove_espacos_das_pontas() {
        assert_eq!(validar_nome_da_nota("  Ideias 2 ").unwrap(), "Ideias 2");
    }

    #[test]
    fn recusa_nome_vazio_ou_so_com_espacos() {
        assert!(validar_nome_da_nota("   ").is_err());
    }

    #[test]
    fn recusa_nome_com_ponto_no_inicio() {
        assert!(validar_nome_da_nota(".oculta").is_err());
    }

    #[test]
    fn recusa_caracteres_proibidos_no_windows() {
        assert!(validar_nome_da_nota("Teste?").is_err());
        assert!(validar_nome_da_nota("pasta/nota").is_err());
    }
}
