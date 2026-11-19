//! Rótulos dos passos do agente. Cada CLI publica o nome da ferramenta que usou; este módulo
//! só traduz esse nome para o texto que a tela mostra. Não guarda nada.

/// Rótulo em português para uma ferramenta. Os nomes chegam em formatos diferentes
/// (OpenCode usa minúsculas, Claude Code usa maiúsculas), por isso a comparação ignora caixa.
/// Ferramenta desconhecida vira "Usou <nome>", para a tela não esconder um passo.
pub fn rotulo_do_passo(nome_da_ferramenta: &str) -> String {
    match nome_da_ferramenta.to_ascii_lowercase().as_str() {
        "read" => "Leu um arquivo".into(),
        "glob" | "list" => "Listou arquivos".into(),
        "grep" => "Buscou no cofre".into(),
        "edit" | "write" | "multiedit" | "patch" => "Alterou um arquivo".into(),
        _ => format!("Usou {nome_da_ferramenta}"),
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn nomes_dos_dois_clis_viram_o_mesmo_rotulo() {
        assert_eq!(rotulo_do_passo("read"), rotulo_do_passo("Read"));
        assert_eq!(rotulo_do_passo("edit"), "Alterou um arquivo");
    }

    #[test]
    fn ferramenta_desconhecida_aparece_pelo_proprio_nome() {
        assert_eq!(rotulo_do_passo("WebFetch"), "Usou WebFetch");
    }
}
