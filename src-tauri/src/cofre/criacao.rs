//! Criação de cofres e de pastas dentro deles. É o único lugar que cria pastas do cofre,
//! seja a pedido do usuário ou da IA. Nunca apaga nem altera o que já existe.

use super::caminhos::{raiz_do_cofre_ativo, texto_para_frontend};
use super::estrutura::criar_estrutura_padrao_do_cofre;
use std::fs;
use std::path::{Path, PathBuf};

/// Caracteres que o Windows não aceita em nomes de pasta.
const CARACTERES_PROIBIDOS_NO_NOME: &str = "<>:\"/\\|?*";

/// Confere o nome de uma pasta: não pode ficar vazio, começar com ponto ou ter caracteres proibidos.
pub(crate) fn validar_nome_de_pasta(nome_digitado: &str) -> Result<String, String> {
    let nome_limpo = nome_digitado.trim();
    if nome_limpo.is_empty() {
        return Err("o nome da pasta não pode ficar vazio".into());
    }
    if nome_limpo.starts_with('.') {
        return Err("o nome da pasta não pode começar com ponto".into());
    }
    let tem_caractere_proibido = nome_limpo
        .chars()
        .any(|caractere| CARACTERES_PROIBIDOS_NO_NOME.contains(caractere) || caractere.is_control());
    if tem_caractere_proibido {
        return Err(format!("o nome \"{nome_limpo}\" tem caracteres que não são permitidos"));
    }
    Ok(nome_limpo.to_string())
}

/// Separa um caminho relativo ao cofre em nomes de pasta. Ex.: `projetos/app` vira `["projetos", "app"]`.
/// Cada parte é validada, então `..` e caminhos absolutos são recusados.
pub(crate) fn partes_do_caminho_relativo(caminho_digitado: &str) -> Result<Vec<String>, String> {
    let caminho_normalizado = caminho_digitado.trim().replace('\\', "/");
    let partes: Vec<&str> = caminho_normalizado
        .split('/')
        .filter(|parte| !parte.trim().is_empty())
        .collect();
    if partes.is_empty() {
        return Err("o caminho da pasta não pode ficar vazio".into());
    }
    partes.into_iter().map(validar_nome_de_pasta).collect()
}

/// Cria um cofre novo: uma pasta `nome` dentro de `pasta_pai`, com a estrutura padrão.
/// Recusa se a pasta já existir, para nunca misturar o cofre com dados que já estão lá.
pub(crate) fn criar_cofre_novo(pasta_pai: &Path, nome_digitado: &str) -> Result<PathBuf, String> {
    let nome_do_cofre = validar_nome_de_pasta(nome_digitado)?;
    if !pasta_pai.is_dir() {
        return Err("a pasta escolhida para o cofre não existe".into());
    }
    let pasta_do_cofre = pasta_pai.join(&nome_do_cofre);
    if pasta_do_cofre.exists() {
        return Err(format!(
            "já existe uma pasta chamada \"{nome_do_cofre}\" nesse local. Escolha outro nome."
        ));
    }
    criar_estrutura_padrao_do_cofre(&pasta_do_cofre)?;
    Ok(pasta_do_cofre)
}

/// Cria uma pasta dentro do cofre ativo. Aceita vários níveis (ex.: `projetos/app`).
/// Recusa se a pasta já existir. É a mesma permissão que a IA vai usar para criar pastas.
pub(crate) fn criar_pasta_no_cofre_ativo(app: &tauri::AppHandle, caminho_relativo: &str) -> Result<String, String> {
    let partes = partes_do_caminho_relativo(caminho_relativo)?;
    let raiz_do_cofre = raiz_do_cofre_ativo(app)?;
    let pasta_nova = partes
        .iter()
        .fold(raiz_do_cofre, |caminho_acumulado, parte| caminho_acumulado.join(parte));
    if pasta_nova.exists() {
        return Err("essa pasta já existe no cofre".into());
    }
    fs::create_dir_all(&pasta_nova).map_err(|erro| erro.to_string())?;
    Ok(texto_para_frontend(&pasta_nova))
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::cofre::estrutura::e_cofre_do_app;

    /// Pasta temporária única para cada teste, dentro da pasta temporária do sistema.
    fn pasta_temporaria_de_teste(nome_do_teste: &str) -> PathBuf {
        let instante = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duracao| duracao.as_nanos())
            .unwrap_or(0);
        let pasta = std::env::temp_dir().join(format!("segundo-cerebro-teste-{nome_do_teste}-{instante}"));
        fs::create_dir_all(&pasta).unwrap();
        pasta
    }

    #[test]
    fn cria_cofre_novo_com_estrutura_padrao() {
        let pasta_pai = pasta_temporaria_de_teste("cofre-novo");
        let cofre = criar_cofre_novo(&pasta_pai, "Segundo Cérebro").unwrap();

        assert!(e_cofre_do_app(&cofre));
        assert!(cofre.join("tarefas").is_dir());
        assert!(cofre.join("notas/anotacoes").is_dir());
        assert!(cofre.join("_sistema").join("AGENTS.md").is_file());
        fs::remove_dir_all(&pasta_pai).unwrap();
    }

    #[test]
    fn nao_cria_cofre_em_pasta_que_ja_existe() {
        let pasta_pai = pasta_temporaria_de_teste("ja-existe");
        fs::create_dir_all(pasta_pai.join("Obsidian")).unwrap();
        fs::write(pasta_pai.join("Obsidian").join("nota.md"), "dado do usuário").unwrap();

        assert!(criar_cofre_novo(&pasta_pai, "Obsidian").is_err());
        // O arquivo que já existia continua intacto.
        assert_eq!(fs::read_to_string(pasta_pai.join("Obsidian").join("nota.md")).unwrap(), "dado do usuário");
        fs::remove_dir_all(&pasta_pai).unwrap();
    }

    #[test]
    fn caminho_relativo_aceita_niveis_e_recusa_saida_do_cofre() {
        assert_eq!(partes_do_caminho_relativo("projetos/app").unwrap(), vec!["projetos", "app"]);
        assert_eq!(partes_do_caminho_relativo("projetos\\app").unwrap(), vec!["projetos", "app"]);
        assert!(partes_do_caminho_relativo("../fora").is_err());
        assert!(partes_do_caminho_relativo("C:/Windows").is_err());
        assert!(partes_do_caminho_relativo("   ").is_err());
    }

    #[test]
    fn nome_de_pasta_recusa_caracteres_proibidos_e_ponto_no_inicio() {
        assert!(validar_nome_de_pasta("Relatório?").is_err());
        assert!(validar_nome_de_pasta(".oculta").is_err());
        assert_eq!(validar_nome_de_pasta("  Projetos  ").unwrap(), "Projetos");
    }
}
