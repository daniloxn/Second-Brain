//! Comando Tauri do resumo semanal. Junta o histórico do agente e as datas de conclusão das tarefas,
//! e entrega a conta pronta para a tela.

use super::calculo::resumir_semana;
use super::contrato::ResumoDaSemana;
use crate::agente::historico::ler_registros;
use crate::cofre::caminhos::raiz_do_cofre_ativo;
use crate::tarefas::servico::datas_de_conclusao_do_cofre;
use chrono::Local;

/// Números da última semana (sete dias, até hoje) para a tela de resumo.
#[tauri::command]
pub fn resumo_da_semana(app: tauri::AppHandle) -> Result<ResumoDaSemana, String> {
    let raiz = raiz_do_cofre_ativo(&app)?;
    let registros = ler_registros(&raiz)?;
    let datas_de_conclusao = datas_de_conclusao_do_cofre(&raiz)?;
    Ok(resumir_semana(&registros, &datas_de_conclusao, Local::now().date_naive()))
}
