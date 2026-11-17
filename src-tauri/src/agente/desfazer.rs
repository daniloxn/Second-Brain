//! Desfaz um pedido do agente: devolve cada nota e pasta ao estado de antes do pedido.
//! Antes de mexer em qualquer coisa, confere que o cofre ainda está como o agente deixou.
//! Se o usuário mexeu depois, o desfazer para e não altera nada.

use super::contrato::{MudancaNoCofre, RegistroDoAgente, TipoDeMudanca};
use crate::cofre::criacao::partes_do_caminho_relativo;
use std::fs;
use std::path::{Path, PathBuf};

/// Estado atual de um item no disco, lido antes de decidir se o desfazer pode seguir.
#[derive(Debug, PartialEq, Eq)]
pub struct EstadoNoDisco {
    /// Texto da nota, ou None se o arquivo não existe.
    pub texto_da_nota: Option<String>,
    pub pasta_existe: bool,
}

/// Desfaz todas as mudanças do registro, da última para a primeira.
pub fn desfazer_registro(raiz_do_cofre: &Path, registro: &RegistroDoAgente) -> Result<(), String> {
    // Primeiro confere tudo; só depois mexe. Assim um conflito não deixa o cofre pela metade.
    for mudanca in &registro.mudancas {
        let caminho = caminho_no_cofre(raiz_do_cofre, &mudanca.caminho)?;
        let estado = ler_estado_no_disco(&caminho);
        if !bate_com_o_registro(mudanca, &estado) {
            return Err(format!(
                "\"{}\" foi mudado depois da ação do agente. Desfazer cancelado, nada foi alterado.",
                mudanca.caminho
            ));
        }
    }

    for mudanca in registro.mudancas.iter().rev() {
        let caminho = caminho_no_cofre(raiz_do_cofre, &mudanca.caminho)?;
        restaurar(&caminho, mudanca)?;
    }
    Ok(())
}

/// Confere se o disco está como o agente deixou esta mudança.
/// Criado e alterado: o texto atual é o `depois`. Apagado: o arquivo não existe.
/// Pasta criada: ela existe. Pasta apagada: ela não existe.
pub fn bate_com_o_registro(mudanca: &MudancaNoCofre, estado: &EstadoNoDisco) -> bool {
    match mudanca.tipo {
        TipoDeMudanca::Criado | TipoDeMudanca::Alterado => estado.texto_da_nota == mudanca.depois,
        TipoDeMudanca::Apagado => estado.texto_da_nota.is_none(),
        TipoDeMudanca::PastaCriada => estado.pasta_existe,
        TipoDeMudanca::PastaApagada => !estado.pasta_existe,
    }
}

/// Coloca o item de volta ao estado de antes. Só chamado depois de `bate_com_o_registro`.
fn restaurar(caminho: &Path, mudanca: &MudancaNoCofre) -> Result<(), String> {
    match mudanca.tipo {
        TipoDeMudanca::Criado => fs::remove_file(caminho).map_err(|erro| erro.to_string()),
        TipoDeMudanca::Alterado | TipoDeMudanca::Apagado => {
            let texto_de_antes = mudanca.antes.as_deref().unwrap_or_default();
            fs::write(caminho, texto_de_antes).map_err(|erro| erro.to_string())
        }
        // Só remove pasta vazia: se sobrou algo dentro, o sistema recusa e nada se perde.
        TipoDeMudanca::PastaCriada => fs::remove_dir(caminho).map_err(|erro| erro.to_string()),
        TipoDeMudanca::PastaApagada => fs::create_dir_all(caminho).map_err(|erro| erro.to_string()),
    }
}

/// Lê o estado atual de uma nota e de uma pasta no mesmo caminho.
fn ler_estado_no_disco(caminho: &Path) -> EstadoNoDisco {
    EstadoNoDisco {
        texto_da_nota: fs::read_to_string(caminho).ok(),
        pasta_existe: caminho.is_dir(),
    }
}

/// Caminho absoluto de um caminho relativo do registro. Recusa `..` e caminho absoluto.
fn caminho_no_cofre(raiz_do_cofre: &Path, caminho_relativo: &str) -> Result<PathBuf, String> {
    let partes = partes_do_caminho_relativo(caminho_relativo)?;
    Ok(partes.iter().fold(raiz_do_cofre.to_path_buf(), |caminho, parte| caminho.join(parte)))
}

#[cfg(test)]
mod testes {
    use super::*;

    fn mudanca(tipo: TipoDeMudanca, antes: Option<&str>, depois: Option<&str>) -> MudancaNoCofre {
        MudancaNoCofre {
            caminho: "tarefas/a.md".to_string(),
            tipo,
            antes: antes.map(str::to_string),
            depois: depois.map(str::to_string),
        }
    }

    fn estado(texto: Option<&str>, pasta_existe: bool) -> EstadoNoDisco {
        EstadoNoDisco {
            texto_da_nota: texto.map(str::to_string),
            pasta_existe,
        }
    }

    #[test]
    fn nota_criada_so_bate_se_o_texto_continua_igual() {
        let criada = mudanca(TipoDeMudanca::Criado, None, Some("oi"));
        assert!(bate_com_o_registro(&criada, &estado(Some("oi"), false)));
        assert!(!bate_com_o_registro(&criada, &estado(Some("outro"), false)));
        assert!(!bate_com_o_registro(&criada, &estado(None, false)));
    }

    #[test]
    fn nota_apagada_so_bate_se_continua_sem_arquivo() {
        let apagada = mudanca(TipoDeMudanca::Apagado, Some("oi"), None);
        assert!(bate_com_o_registro(&apagada, &estado(None, false)));
        assert!(!bate_com_o_registro(&apagada, &estado(Some("oi"), false)));
    }

    #[test]
    fn pasta_criada_precisa_continuar_existindo() {
        let pasta = mudanca(TipoDeMudanca::PastaCriada, None, None);
        assert!(bate_com_o_registro(&pasta, &estado(None, true)));
        assert!(!bate_com_o_registro(&pasta, &estado(None, false)));
    }

    #[test]
    fn caminho_com_pai_e_recusado() {
        assert!(caminho_no_cofre(Path::new("C:\\cofre"), "../fora.md").is_err());
    }

    fn registro_com(mudancas: Vec<MudancaNoCofre>) -> RegistroDoAgente {
        RegistroDoAgente {
            id: "ag-teste".to_string(),
            quando: "2026-11-17 10:00:00".to_string(),
            pedido: "teste".to_string(),
            provedor: "Antigravity".to_string(),
            resposta: "feito".to_string(),
            mudancas,
            desfeito: false,
        }
    }

    fn pasta_de_teste(nome: &str) -> PathBuf {
        let raiz = std::env::temp_dir().join(format!("segundo-cerebro-{nome}-{}", std::process::id()));
        fs::create_dir_all(raiz.join("tarefas")).unwrap();
        raiz
    }

    #[test]
    fn desfazer_remove_nota_criada_e_volta_nota_alterada() {
        let raiz = pasta_de_teste("desfazer-ok");
        fs::write(raiz.join("tarefas").join("nova.md"), "oi").unwrap();
        fs::write(raiz.join("n.md"), "depois").unwrap();
        let registro = registro_com(vec![
            MudancaNoCofre {
                caminho: "tarefas/nova.md".to_string(),
                tipo: TipoDeMudanca::Criado,
                antes: None,
                depois: Some("oi".to_string()),
            },
            MudancaNoCofre {
                caminho: "n.md".to_string(),
                tipo: TipoDeMudanca::Alterado,
                antes: Some("antes".to_string()),
                depois: Some("depois".to_string()),
            },
        ]);

        desfazer_registro(&raiz, &registro).unwrap();

        assert!(!raiz.join("tarefas").join("nova.md").exists());
        assert_eq!(fs::read_to_string(raiz.join("n.md")).unwrap(), "antes");
        fs::remove_dir_all(&raiz).ok();
    }

    #[test]
    fn desfazer_para_sem_mexer_quando_o_usuario_mudou_depois() {
        let raiz = pasta_de_teste("desfazer-conflito");
        fs::write(raiz.join("tarefas").join("nova.md"), "outro texto").unwrap();
        fs::write(raiz.join("n.md"), "depois").unwrap();
        let registro = registro_com(vec![
            MudancaNoCofre {
                caminho: "tarefas/nova.md".to_string(),
                tipo: TipoDeMudanca::Criado,
                antes: None,
                depois: Some("oi".to_string()),
            },
            MudancaNoCofre {
                caminho: "n.md".to_string(),
                tipo: TipoDeMudanca::Alterado,
                antes: Some("antes".to_string()),
                depois: Some("depois".to_string()),
            },
        ]);

        assert!(desfazer_registro(&raiz, &registro).is_err());
        assert_eq!(fs::read_to_string(raiz.join("n.md")).unwrap(), "depois");
        assert!(raiz.join("tarefas").join("nova.md").exists());
        fs::remove_dir_all(&raiz).ok();
    }
}
