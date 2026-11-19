//! Casos de uso de tarefas compartilhados pelo app (comandos do Tauri) e pelo servidor MCP:
//! listar e criar. Recebem a raiz do cofre e o "agora" de fora, então não dependem do Tauri
//! nem do relógio, e podem ser testados com uma pasta temporária.

use super::contrato::{ListaDeTarefasDto, NovaTarefaDto, TarefaDto, VERSAO_DO_CONTRATO_DE_TAREFAS};
use super::regras::{montar_nova_tarefa, ordenar_por_urgencia};
use super::repositorio::{gerar_id_livre, gravar_tarefa_nova, ler_todas_as_tarefas};
use super::validacao::slug_do_titulo;
use chrono::{NaiveDate, NaiveDateTime};
use std::path::Path;

/// Dias em que as tarefas do cofre foram concluídas, uma entrada por tarefa que tem a data.
/// Usado pelo resumo semanal, que não precisa conhecer o resto da tarefa.
pub(crate) fn datas_de_conclusao_do_cofre(raiz: &Path) -> Result<Vec<NaiveDate>, String> {
    Ok(ler_todas_as_tarefas(raiz)?
        .iter()
        .filter_map(|tarefa| tarefa.concluida_em)
        .collect())
}

/// Todas as tarefas do cofre em `raiz`, ordenadas por urgência e com a situação calculada em `agora`.
pub(crate) fn listar_tarefas_do_cofre(raiz: &Path, agora: NaiveDateTime) -> Result<ListaDeTarefasDto, String> {
    let mut tarefas = ler_todas_as_tarefas(raiz)?;
    ordenar_por_urgencia(&mut tarefas);
    Ok(ListaDeTarefasDto {
        versao: VERSAO_DO_CONTRATO_DE_TAREFAS,
        tarefas: tarefas.iter().map(|tarefa| TarefaDto::de_tarefa(tarefa, agora)).collect(),
    })
}

/// Cria uma tarefa nova em `tarefas/`, com id derivado do título. Devolve a tarefa criada.
pub(crate) fn criar_tarefa_no_cofre(raiz: &Path, entrada: NovaTarefaDto, agora: NaiveDateTime) -> Result<TarefaDto, String> {
    let nova_tarefa = entrada.para_nova_tarefa()?;
    let id_livre = gerar_id_livre(raiz, &slug_do_titulo(&nova_tarefa.titulo))?;
    let tarefa = montar_nova_tarefa(id_livre, nova_tarefa)?;
    gravar_tarefa_nova(raiz, &tarefa)?;
    Ok(TarefaDto::de_tarefa(&tarefa, agora))
}

#[cfg(test)]
mod testes {
    use super::*;
    use chrono::NaiveDate;
    use std::fs;

    fn cofre_de_teste(nome: &str) -> std::path::PathBuf {
        let raiz = std::env::temp_dir().join(format!("segundo-cerebro-servico-{nome}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&raiz);
        fs::create_dir_all(raiz.join("_sistema")).unwrap();
        raiz.canonicalize().unwrap()
    }

    fn entrada_com_titulo(titulo: &str) -> NovaTarefaDto {
        NovaTarefaDto {
            titulo: titulo.into(),
            descricao: String::new(),
            prioridade: Some("alta".into()),
            prazo: Some("2026-11-18".into()),
            origem: None,
            tags: Vec::new(),
            recorrencia: None,
        }
    }

    #[test]
    fn tarefa_criada_aparece_na_listagem() {
        let raiz = cofre_de_teste("ida-e-volta");
        let agora = NaiveDate::from_ymd_opt(2026, 11, 17).unwrap().and_hms_opt(8, 0, 0).unwrap();

        criar_tarefa_no_cofre(&raiz, entrada_com_titulo("Entregar relatório"), agora).unwrap();
        let lista = listar_tarefas_do_cofre(&raiz, agora).unwrap();

        assert_eq!(lista.tarefas.len(), 1);
        assert_eq!(lista.tarefas[0].titulo, "Entregar relatório");
        assert_eq!(lista.tarefas[0].situacao, "proxima");
    }

    #[test]
    fn titulo_repetido_ganha_numero_no_id() {
        let raiz = cofre_de_teste("id-repetido");
        let agora = NaiveDate::from_ymd_opt(2026, 11, 17).unwrap().and_hms_opt(8, 0, 0).unwrap();

        let primeira = criar_tarefa_no_cofre(&raiz, entrada_com_titulo("Pagar conta"), agora).unwrap();
        let segunda = criar_tarefa_no_cofre(&raiz, entrada_com_titulo("Pagar conta"), agora).unwrap();

        assert_ne!(primeira.id, segunda.id);
        assert!(segunda.id.ends_with("-2.md"));
    }
}
