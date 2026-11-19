//! Comandos Tauri da funcionalidade tarefas. Cada comando descobre a pasta do cofre ativo,
//! converte a entrada, chama as regras e o repositório, e devolve o contrato.
//! Nenhuma regra de negócio mora aqui.

use super::contrato::{AlteracaoTarefaDto, ListaDeTarefasDto, NovaTarefaDto, TarefaDto};
use super::regras::aplicar_alteracao;
use super::repositorio::{apagar_tarefa as apagar_arquivo_da_tarefa, gravar_tarefa, ler_tarefa};
use super::servico::{criar_tarefa_no_cofre, listar_tarefas_do_cofre};
use crate::cofre::caminhos::raiz_do_cofre_ativo;
use chrono::Local;

/// Todas as tarefas do cofre, já ordenadas por urgência e com a situação calculada.
#[tauri::command]
pub fn listar_tarefas(app: tauri::AppHandle) -> Result<ListaDeTarefasDto, String> {
    listar_tarefas_do_cofre(&raiz_do_cofre_ativo(&app)?, Local::now().naive_local())
}

/// Cria uma tarefa nova em `tarefas/`, com id derivado do título. Devolve a tarefa criada.
#[tauri::command]
pub fn criar_tarefa(app: tauri::AppHandle, entrada: NovaTarefaDto) -> Result<TarefaDto, String> {
    criar_tarefa_no_cofre(&raiz_do_cofre_ativo(&app)?, entrada, Local::now().naive_local())
}

/// Aplica alterações parciais a uma tarefa existente e devolve a tarefa já atualizada.
#[tauri::command]
pub fn atualizar_tarefa(app: tauri::AppHandle, id: String, alteracao: AlteracaoTarefaDto) -> Result<TarefaDto, String> {
    let agora = Local::now().naive_local();
    let raiz = raiz_do_cofre_ativo(&app)?;
    let mut tarefa = ler_tarefa(&raiz, &id)?;
    aplicar_alteracao(&mut tarefa, alteracao.para_alteracao()?)?;
    if tarefa.recorrencia_diaria && tarefa.status == super::valores::StatusTarefa::Feita {
        tarefa.ultima_conclusao = Some(agora.date());
    }
    super::regras::registrar_conclusao(&mut tarefa, agora.date());
    gravar_tarefa(&raiz, &tarefa)?;
    Ok(TarefaDto::de_tarefa(&tarefa, agora))
}

/// Apaga uma tarefa pelo identificador do arquivo dentro da pasta `tarefas/`.
#[tauri::command]
pub fn apagar_tarefa(app: tauri::AppHandle, id: String) -> Result<(), String> {
    apagar_arquivo_da_tarefa(&raiz_do_cofre_ativo(&app)?, &id)
}
