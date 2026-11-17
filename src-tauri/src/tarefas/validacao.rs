//! Regras de entrada para título e tags, e a geração do nome de arquivo a partir do título.
//! Função pura: não toca no disco nem no backend.

/// Tamanho máximo do título, para caber nas notificações e no painel.
const TAMANHO_MAXIMO_DO_TITULO: usize = 200;

/// Confere o título e devolve a versão limpa. Não pode ficar vazio nem ter quebra de linha.
pub fn validar_titulo(titulo_digitado: &str) -> Result<String, String> {
    let limpo = titulo_digitado.trim();
    if limpo.is_empty() {
        return Err("o título não pode ficar vazio".into());
    }
    if limpo.contains('\n') || limpo.contains('\r') {
        return Err("o título não pode ter quebra de linha".into());
    }
    if limpo.chars().count() > TAMANHO_MAXIMO_DO_TITULO {
        return Err(format!("o título passou de {TAMANHO_MAXIMO_DO_TITULO} caracteres"));
    }
    Ok(limpo.to_string())
}

/// Confere as tags: sem vírgula, colchetes ou aspas, porque elas separam o formato da lista.
/// Tags vazias são descartadas.
pub fn validar_tags(tags_digitadas: &[String]) -> Result<Vec<String>, String> {
    let mut tags_limpas = Vec::new();
    for tag_digitada in tags_digitadas {
        let tag_limpa = tag_digitada.trim();
        if tag_limpa.is_empty() {
            continue;
        }
        if tag_limpa.chars().any(|caractere| ",[]\"".contains(caractere)) {
            return Err(format!("a tag \"{tag_limpa}\" tem caracteres não permitidos (, [ ] \")"));
        }
        tags_limpas.push(tag_limpa.to_string());
    }
    Ok(tags_limpas)
}

/// Transforma um título em nome de arquivo: minúsculas, letras e números, hífen no lugar do resto.
/// Ex.: `Entregar Relatório!` vira `entregar-relatório`.
pub fn slug_do_titulo(titulo: &str) -> String {
    let mut slug = String::new();
    for caractere in titulo.to_lowercase().chars() {
        if caractere.is_alphanumeric() {
            slug.push(caractere);
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug_sem_bordas = slug.trim_matches('-');
    if slug_sem_bordas.is_empty() {
        "tarefa".to_string()
    } else {
        slug_sem_bordas.to_string()
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn slug_mantem_letras_acentuadas_e_limpa_pontuacao() {
        assert_eq!(slug_do_titulo("Entregar Relatório!"), "entregar-relatório");
        assert_eq!(slug_do_titulo("   "), "tarefa");
    }

    #[test]
    fn validar_titulo_recusa_vazio_e_quebra_de_linha() {
        assert!(validar_titulo("  ").is_err());
        assert!(validar_titulo("linha\numa").is_err());
        assert_eq!(validar_titulo("  ok  ").unwrap(), "ok");
    }

    #[test]
    fn validar_tags_recusa_separadores() {
        assert!(validar_tags(&["a,b".to_string()]).is_err());
        assert_eq!(validar_tags(&[" x ".to_string(), "".to_string()]).unwrap(), vec!["x"]);
    }
}
