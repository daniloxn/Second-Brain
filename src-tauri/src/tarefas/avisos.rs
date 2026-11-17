//! Agendador de avisos: a cada minuto confere as tarefas e mostra uma notificação do sistema
//! quando o prazo está perto ou já passou. Cada tarefa avisa uma vez por prazo: se o prazo
//! mudar, o aviso volta a valer.

use super::dominio::Tarefa;
use super::valores::PrazoTarefa;
use super::situacao::{situacao_da_tarefa, SituacaoTarefa};
use super::regras::{deve_avisar, ANTECEDENCIA_DO_AVISO_EM_MINUTOS};
use super::repositorio::ler_todas_as_tarefas;
use chrono::{Local, NaiveDateTime};
use std::collections::HashSet;
use std::thread;
use std::time::Duration;
use tauri_plugin_notification::NotificationExt;

/// Intervalo entre duas conferências das tarefas.
const INTERVALO_DE_VERIFICACAO: Duration = Duration::from_secs(60);

/// Inicia o agendador em uma thread de fundo. Chamado uma vez, na montagem do app.
pub fn iniciar_avisos(app: tauri::AppHandle) {
    thread::spawn(move || {
        // Guarda os avisos já enviados nesta sessão, para não repetir a cada minuto.
        let mut avisos_ja_enviados: HashSet<String> = HashSet::new();
        loop {
            conferir_tarefas_uma_vez(&app, &mut avisos_ja_enviados);
            thread::sleep(INTERVALO_DE_VERIFICACAO);
        }
    });
}

/// Confere todas as tarefas no instante atual e envia os avisos que ainda não saíram.
fn conferir_tarefas_uma_vez(app: &tauri::AppHandle, avisos_ja_enviados: &mut HashSet<String>) {
    let agora = Local::now().naive_local();
    // Sem cofre aberto a leitura falha; nesse caso não há o que avisar.
    let Ok(tarefas) = ler_todas_as_tarefas(app) else {
        return;
    };

    for tarefa in &tarefas {
        if !deve_avisar(tarefa, agora, ANTECEDENCIA_DO_AVISO_EM_MINUTOS) {
            continue;
        }
        // `insert` devolve falso se a chave já estava lá: aviso já enviado para este prazo.
        if !avisos_ja_enviados.insert(chave_do_aviso(tarefa)) {
            continue;
        }
        mostrar_notificacao(app, tarefa, agora);
    }
}

/// Chave que identifica um aviso: a tarefa e o prazo dela. Muda o prazo, muda a chave.
fn chave_do_aviso(tarefa: &Tarefa) -> String {
    let prazo_texto = tarefa.prazo.as_ref().map(PrazoTarefa::como_texto).unwrap_or_default();
    format!("{}|{prazo_texto}", tarefa.id)
}

/// Envia a notificação do sistema. Falha ao notificar não derruba o app.
fn mostrar_notificacao(app: &tauri::AppHandle, tarefa: &Tarefa, agora: NaiveDateTime) {
    let (titulo_do_aviso, corpo_do_aviso) = texto_do_aviso(tarefa, agora);
    let _ = app
        .notification()
        .builder()
        .title(titulo_do_aviso)
        .body(corpo_do_aviso)
        .show();
}

/// Título e corpo da notificação, de acordo com a situação da tarefa no instante `agora`.
pub(crate) fn texto_do_aviso(tarefa: &Tarefa, agora: NaiveDateTime) -> (String, String) {
    let prazo_texto = tarefa.prazo.as_ref().map(PrazoTarefa::como_texto).unwrap_or_default();
    match situacao_da_tarefa(tarefa, agora) {
        SituacaoTarefa::Atrasada => (
            format!("Atrasada: {}", tarefa.titulo),
            format!("O prazo era {prazo_texto}"),
        ),
        _ => (
            format!("Vence em breve: {}", tarefa.titulo),
            format!("Prazo: {prazo_texto}"),
        ),
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::tarefas::valores::PrioridadeTarefa;
    use chrono::NaiveDate;

    #[test]
    fn aviso_de_tarefa_atrasada_fala_em_atraso() {
        let mut tarefa = Tarefa::do_arquivo("pagar.md", "---\ntitulo: \"Pagar conta\"\nprazo: 2026-11-10\n---\n");
        tarefa.prioridade = PrioridadeTarefa::Alta;
        let agora = NaiveDate::from_ymd_opt(2026, 11, 12).unwrap().and_hms_opt(8, 0, 0).unwrap();
        let (titulo, corpo) = texto_do_aviso(&tarefa, agora);
        assert_eq!(titulo, "Atrasada: Pagar conta");
        assert_eq!(corpo, "O prazo era 2026-11-10");
    }

    #[test]
    fn chave_muda_quando_o_prazo_muda() {
        let antes = Tarefa::do_arquivo("x.md", "---\nprazo: 2026-11-10\n---\n");
        let depois = Tarefa::do_arquivo("x.md", "---\nprazo: 2026-11-11\n---\n");
        assert_ne!(chave_do_aviso(&antes), chave_do_aviso(&depois));
    }
}
