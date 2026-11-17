//! Comandos Tauri da funcionalidade tarefas. Cada comando recebe o contrato,
//! converte para o domínio, chama as regras e o repositório, e devolve o contrato.
//! Nenhuma regra de negócio mora aqui.

use super::contrato::{
    AlteracaoTarefaDto, ListaDeTarefasDto, NovaTarefaDto, TarefaDto, VERSAO_DO_CONTRATO_DE_TAREFAS,
};
use super::validacao::slug_do_titulo;
use super::regras::{aplicar_alteracao, montar_nova_tarefa, ordenar_por_urgencia};
use super::repositorio::{gerar_id_livre, gravar_tarefa, gravar_tarefa_nova, ler_tarefa, ler_todas_as_tarefas};
use chrono::Local;

/// Todas as tarefas do cofre, já ordenadas por urgência e com a situação calculada.
#[tauri::command]
pub fn listar_tarefas(app: tauri::AppHandle) -> Result<ListaDeTarefasDto, String> {
    let agora = Local::now().naive_local();
    let mut tarefas = ler_todas_as_tarefas(&app)?;
    ordenar_por_urgencia(&mut tarefas);
    Ok(ListaDeTarefasDto {
        versao: VERSAO_DO_CONTRATO_DE_TAREFAS,
        tarefas: tarefas.iter().map(|tarefa| TarefaDto::de_tarefa(tarefa, agora)).collect(),
    })
}

/// Cria uma tarefa nova em `tarefas/`, com id derivado do título. Devolve a tarefa criada.
#[tauri::command]
pub fn criar_tarefa(app: tauri::AppHandle, entrada: NovaTarefaDto) -> Result<TarefaDto, String> {
    let agora = Local::now().naive_local();
    let nova_tarefa = entrada.para_nova_tarefa()?;
    let id_livre = gerar_id_livre(&app, &slug_do_titulo(&nova_tarefa.titulo))?;
    let tarefa = montar_nova_tarefa(id_livre, nova_tarefa)?;
    gravar_tarefa_nova(&app, &tarefa)?;
    Ok(TarefaDto::de_tarefa(&tarefa, agora))
}

/// Aplica alterações parciais a uma tarefa existente e devolve a tarefa já atualizada.
#[tauri::command]
pub fn atualizar_tarefa(app: tauri::AppHandle, id: String, alteracao: AlteracaoTarefaDto) -> Result<TarefaDto, String> {
    let agora = Local::now().naive_local();
    let mut tarefa = ler_tarefa(&app, &id)?;
    aplicar_alteracao(&mut tarefa, alteracao.para_alteracao()?)?;
    gravar_tarefa(&app, &tarefa)?;
    Ok(TarefaDto::de_tarefa(&tarefa, agora))
}
