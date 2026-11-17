//! Histórico do agente: `_sistema/historico-agente.log`, com um registro JSON por linha.
//! Só este módulo lê e grava o log. Linhas que não lêem são preservadas ao regravar.

use super::contrato::{MudancaNoCofre, RegistroDoAgente};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Caminho do log dentro do cofre.
pub fn caminho_do_historico(raiz_do_cofre: &Path) -> PathBuf {
    raiz_do_cofre.join("_sistema").join("historico-agente.log")
}

/// Monta o registro de um pedido. `resultado` é a resposta do agente ou o erro que ele deu.
/// Mesmo com erro o registro é montado, porque o agente pode ter mudado arquivos antes de falhar.
pub fn montar_registro(
    id: String,
    quando: String,
    pedido: &str,
    nome_do_provedor: &str,
    resultado: &Result<String, String>,
    mudancas: Vec<MudancaNoCofre>,
) -> RegistroDoAgente {
    let resposta = match resultado {
        Ok(texto) => texto.clone(),
        Err(erro) => format!("Falhou: {erro}"),
    };
    RegistroDoAgente {
        id,
        quando,
        pedido: pedido.to_string(),
        provedor: nome_do_provedor.to_string(),
        resposta,
        mudancas,
        desfeito: false,
    }
}

/// Acrescenta um registro ao fim do log. Cria o log se ele ainda não existir.
pub fn anexar_registro(raiz_do_cofre: &Path, registro: &RegistroDoAgente) -> Result<(), String> {
    let caminho = caminho_do_historico(raiz_do_cofre);
    let linha = serde_json::to_string(registro).map_err(|erro| erro.to_string())?;
    let mut arquivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&caminho)
        .map_err(|erro| erro.to_string())?;
    writeln!(arquivo, "{linha}").map_err(|erro| erro.to_string())
}

/// Todos os registros, do mais recente para o mais antigo. Linhas inválidas são ignoradas.
pub fn ler_registros(raiz_do_cofre: &Path) -> Result<Vec<RegistroDoAgente>, String> {
    let linhas = ler_linhas_do_log(raiz_do_cofre)?;
    let mut registros: Vec<RegistroDoAgente> = linhas
        .iter()
        .filter_map(|linha| serde_json::from_str(linha).ok())
        .collect();
    registros.reverse();
    Ok(registros)
}

/// Troca no log o registro com o mesmo id pelo `registro_atualizado`. As outras linhas ficam como estão.
pub fn regravar_registro(raiz_do_cofre: &Path, registro_atualizado: &RegistroDoAgente) -> Result<(), String> {
    let linhas = ler_linhas_do_log(raiz_do_cofre)?;
    let novas_linhas: Vec<String> = linhas
        .into_iter()
        .map(|linha| match serde_json::from_str::<RegistroDoAgente>(&linha) {
            Ok(registro) if registro.id == registro_atualizado.id => {
                serde_json::to_string(registro_atualizado).unwrap_or(linha)
            }
            _ => linha,
        })
        .collect();

    let mut texto = novas_linhas.join("\n");
    texto.push('\n');
    fs::write(caminho_do_historico(raiz_do_cofre), texto).map_err(|erro| erro.to_string())
}

/// Linhas do log na ordem em que foram gravadas. Log que não existe é uma lista vazia.
fn ler_linhas_do_log(raiz_do_cofre: &Path) -> Result<Vec<String>, String> {
    let caminho = caminho_do_historico(raiz_do_cofre);
    if !caminho.exists() {
        return Ok(Vec::new());
    }
    let texto = fs::read_to_string(&caminho).map_err(|erro| erro.to_string())?;
    Ok(texto.lines().filter(|linha| !linha.trim().is_empty()).map(str::to_string).collect())
}

#[cfg(test)]
mod testes {
    use super::*;

    fn registro_de_teste(id: &str, resultado: &Result<String, String>) -> RegistroDoAgente {
        montar_registro(
            id.to_string(),
            "2026-11-17 10:00:00".to_string(),
            "me lembre amanhã",
            "Antigravity",
            resultado,
            Vec::new(),
        )
    }

    #[test]
    fn falha_do_agente_vira_resposta_com_prefixo() {
        let registro = registro_de_teste("ag-1", &Err("sem capacidade".to_string()));
        assert_eq!(registro.resposta, "Falhou: sem capacidade");
        assert!(!registro.desfeito);
    }

    #[test]
    fn registros_voltam_do_mais_recente_e_regravar_troca_so_o_escolhido() {
        let raiz = std::env::temp_dir().join(format!("segundo-cerebro-historico-{}", std::process::id()));
        fs::create_dir_all(raiz.join("_sistema")).unwrap();

        let primeiro = registro_de_teste("ag-1", &Ok("feito".to_string()));
        anexar_registro(&raiz, &primeiro).unwrap();
        anexar_registro(&raiz, &registro_de_teste("ag-2", &Ok("feito".to_string()))).unwrap();
        assert_eq!(ler_registros(&raiz).unwrap()[0].id, "ag-2");

        let mut marcado = primeiro.clone();
        marcado.desfeito = true;
        regravar_registro(&raiz, &marcado).unwrap();
        let depois = ler_registros(&raiz).unwrap();
        assert_eq!(depois.len(), 2);
        assert!(depois.iter().any(|registro| registro.id == "ag-1" && registro.desfeito));
        assert!(depois.iter().any(|registro| registro.id == "ag-2" && !registro.desfeito));

        fs::remove_dir_all(&raiz).ok();
    }
}
