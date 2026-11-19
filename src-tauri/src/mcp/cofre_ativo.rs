//! Descobre o cofre que o servidor MCP deve usar. Com `--cofre <pasta>`, usa essa pasta.
//! Sem a opção, lê o mesmo config.json que o app grava, então segue o cofre escolhido na tela.
//! Nos dois casos, a pasta precisa ser um cofre do app (ter `_sistema/`), como manda o PLANO.md.

use crate::cofre::estrutura::e_cofre_do_app;
use crate::configuracao::Configuracao;
use std::fs;
use std::path::{Path, PathBuf};

/// Identificador do app, que dá o nome da pasta de configuração em %APPDATA%.
/// Precisa ser igual ao `identifier` do tauri.conf.json; o teste `identificador_bate_com_o_tauri_conf` confere.
const IDENTIFICADOR_DO_APP: &str = "com.exemplo.tauribasico";

/// Raiz canonizada do cofre para o servidor usar. Falha com mensagem para o usuário se não houver cofre.
pub(crate) fn raiz_do_cofre_para_o_mcp(argumentos: &[String]) -> Result<PathBuf, String> {
    if let Some(pasta_informada) = valor_da_opcao(argumentos, "--cofre") {
        return confirmar_cofre(Path::new(pasta_informada));
    }
    let arquivo_de_configuracao = caminho_do_config_do_app()?;
    let texto = fs::read_to_string(&arquivo_de_configuracao).map_err(|_| {
        format!(
            "nenhum cofre aberto no app ({}). Abra um cofre na tela ou informe --cofre <pasta>",
            arquivo_de_configuracao.display()
        )
    })?;
    // Configuração corrompida vira "sem cofre", o mesmo critério do app.
    let configuracao: Configuracao = serde_json::from_str(&texto).unwrap_or_default();
    let cofre_salvo = configuracao.cofre.ok_or("nenhum cofre aberto no app")?;
    confirmar_cofre(Path::new(&cofre_salvo))
}

/// Valor que segue a opção, em `--opcao valor`. Devolve None se a opção não aparece ou não tem valor.
fn valor_da_opcao<'a>(argumentos: &'a [String], opcao: &str) -> Option<&'a str> {
    argumentos
        .windows(2)
        .find(|par| par[0] == opcao)
        .map(|par| par[1].as_str())
}

/// Caminho do config.json do app: `%APPDATA%\<identificador>\config.json` no Windows.
fn caminho_do_config_do_app() -> Result<PathBuf, String> {
    let appdata = std::env::var_os("APPDATA").ok_or("variável APPDATA não encontrada")?;
    Ok(PathBuf::from(appdata).join(IDENTIFICADOR_DO_APP).join("config.json"))
}

/// Canoniza a pasta e confere que ela é um cofre do app. Recusa pasta comum, como um cofre de Obsidian.
fn confirmar_cofre(pasta: &Path) -> Result<PathBuf, String> {
    let raiz = pasta
        .canonicalize()
        .map_err(|erro| format!("pasta do cofre não encontrada: {} ({erro})", pasta.display()))?;
    if !e_cofre_do_app(&raiz) {
        return Err(format!("{} não é um cofre do Segundo Cérebro (falta a pasta _sistema)", raiz.display()));
    }
    Ok(raiz)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn valor_da_opcao_le_o_argumento_seguinte() {
        let argumentos: Vec<String> = ["--cofre", "C:/cofre"].iter().map(|texto| texto.to_string()).collect();
        assert_eq!(valor_da_opcao(&argumentos, "--cofre"), Some("C:/cofre"));
        assert_eq!(valor_da_opcao(&argumentos, "--outra"), None);
    }

    #[test]
    fn valor_da_opcao_sem_valor_devolve_none() {
        let argumentos: Vec<String> = vec!["--cofre".to_string()];
        assert_eq!(valor_da_opcao(&argumentos, "--cofre"), None);
    }

    #[test]
    fn pasta_sem_sistema_nao_e_cofre() {
        let pasta = std::env::temp_dir().join(format!("segundo-cerebro-sem-sistema-{}", std::process::id()));
        fs::create_dir_all(&pasta).unwrap();
        let erro = confirmar_cofre(&pasta).unwrap_err();
        assert!(erro.contains("não é um cofre do Segundo Cérebro"));
    }

    #[test]
    fn identificador_bate_com_o_tauri_conf() {
        let texto = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/tauri.conf.json")).unwrap();
        let configuracao_do_tauri: serde_json::Value = serde_json::from_str(&texto).unwrap();
        assert_eq!(configuracao_do_tauri["identifier"], IDENTIFICADOR_DO_APP);
    }
}
