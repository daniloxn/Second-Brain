//! Situação de uma tarefa em relação à data de hoje: atrasada, hoje, próxima, sem prazo ou concluída.
//! O backend calcula isso e o frontend só exibe. Função pura: recebe o "agora" de fora.

use super::dominio::Tarefa;
use super::valores::PrazoTarefa;
use chrono::NaiveDateTime;

/// Posição de uma tarefa em relação à data de hoje. Faz parte do contrato com o frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SituacaoTarefa {
    /// Prazo já passou e a tarefa não está concluída.
    Atrasada,
    /// Prazo é hoje.
    Hoje,
    /// Prazo é depois de hoje.
    Proxima,
    /// Não tem prazo.
    SemPrazo,
    /// Feita ou arquivada.
    Concluida,
}

impl SituacaoTarefa {
    /// Texto enviado ao frontend. Faz parte do contrato, não mudar sem versionar.
    pub fn como_texto(self) -> &'static str {
        match self {
            Self::Atrasada => "atrasada",
            Self::Hoje => "hoje",
            Self::Proxima => "proxima",
            Self::SemPrazo => "sem_prazo",
            Self::Concluida => "concluida",
        }
    }
}

/// Situação da tarefa no instante `agora`.
pub fn situacao_da_tarefa(tarefa: &Tarefa, agora: NaiveDateTime) -> SituacaoTarefa {
    if tarefa.status.esta_concluida() {
        return SituacaoTarefa::Concluida;
    }
    let Some(prazo) = &tarefa.prazo else {
        return SituacaoTarefa::SemPrazo;
    };
    let hoje = agora.date();
    match prazo {
        // Prazo só com data vence no fim do dia, então só fica atrasado no dia seguinte.
        PrazoTarefa::SoData(dia) if *dia < hoje => SituacaoTarefa::Atrasada,
        PrazoTarefa::DataEHora(momento) if *momento < agora => SituacaoTarefa::Atrasada,
        _ if prazo.dia() == hoje => SituacaoTarefa::Hoje,
        _ => SituacaoTarefa::Proxima,
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::tarefas::valores::{PrioridadeTarefa, StatusTarefa};
    use chrono::NaiveDate;

    /// Data e hora fixas, para o resultado não depender do relógio.
    fn momento(ano: i32, mes: u32, dia: u32, hora: u32, minuto: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(ano, mes, dia)
            .unwrap()
            .and_hms_opt(hora, minuto, 0)
            .unwrap()
    }

    fn tarefa_com_prazo(prazo: Option<&str>) -> Tarefa {
        Tarefa {
            id: "x.md".to_string(),
            titulo: "x".to_string(),
            descricao: String::new(),
            status: StatusTarefa::Aberta,
            prioridade: PrioridadeTarefa::Media,
            prazo: prazo.and_then(PrazoTarefa::do_texto),
            origem: String::new(),
            tags: Vec::new(),
            recorrencia_diaria: false,
            ultima_conclusao: None,
            campos_extras: Vec::new(),
        }
    }

    #[test]
    fn atrasada_quando_prazo_com_hora_ja_passou() {
        let tarefa = tarefa_com_prazo(Some("2026-11-18 10:00"));
        assert_eq!(situacao_da_tarefa(&tarefa, momento(2026, 11, 18, 11, 0)), SituacaoTarefa::Atrasada);
    }

    #[test]
    fn prazo_so_data_do_dia_de_hoje_nao_fica_atrasado_durante_o_dia() {
        let tarefa = tarefa_com_prazo(Some("2026-11-18"));
        assert_eq!(situacao_da_tarefa(&tarefa, momento(2026, 11, 18, 23, 0)), SituacaoTarefa::Hoje);
    }

    #[test]
    fn prazo_so_data_de_ontem_fica_atrasado() {
        let tarefa = tarefa_com_prazo(Some("2026-11-17"));
        assert_eq!(situacao_da_tarefa(&tarefa, momento(2026, 11, 18, 8, 0)), SituacaoTarefa::Atrasada);
    }

    #[test]
    fn concluida_e_sem_prazo_tem_situacao_propria() {
        let mut feita = tarefa_com_prazo(Some("2020-01-01"));
        feita.status = StatusTarefa::Feita;
        assert_eq!(situacao_da_tarefa(&feita, momento(2026, 1, 1, 8, 0)), SituacaoTarefa::Concluida);

        let sem_prazo = tarefa_com_prazo(None);
        assert_eq!(situacao_da_tarefa(&sem_prazo, momento(2026, 1, 1, 8, 0)), SituacaoTarefa::SemPrazo);
    }
}
