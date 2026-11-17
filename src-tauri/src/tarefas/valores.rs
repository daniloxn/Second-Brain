//! Valores que uma tarefa pode ter: status, prioridade e prazo.
//! Cada um sabe ler o próprio texto (do arquivo ou do contrato) e escrever de volta.

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

/// Hora assumida quando o prazo tem só data. Serve de referência para o aviso.
const HORA_DO_PRAZO_SEM_HORARIO: u32 = 9;

/// Estado de andamento da tarefa.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusTarefa {
    Aberta,
    Fazendo,
    Feita,
    Arquivada,
}

impl StatusTarefa {
    /// Lê o status a partir do texto do arquivo. Devolve None para valor desconhecido.
    pub fn do_texto(texto: &str) -> Option<Self> {
        match texto.trim().to_lowercase().as_str() {
            "aberta" => Some(Self::Aberta),
            "fazendo" => Some(Self::Fazendo),
            "feita" => Some(Self::Feita),
            "arquivada" => Some(Self::Arquivada),
            _ => None,
        }
    }

    /// Texto gravado no arquivo e enviado ao frontend.
    pub fn como_texto(self) -> &'static str {
        match self {
            Self::Aberta => "aberta",
            Self::Fazendo => "fazendo",
            Self::Feita => "feita",
            Self::Arquivada => "arquivada",
        }
    }

    /// Feita e arquivada contam como concluídas: saem da fila de urgência.
    pub fn esta_concluida(self) -> bool {
        matches!(self, Self::Feita | Self::Arquivada)
    }
}

/// Prioridade da tarefa. A ordem de declaração define a urgência: Alta vem primeiro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrioridadeTarefa {
    Alta,
    Media,
    Baixa,
}

impl PrioridadeTarefa {
    /// Lê a prioridade a partir do texto. Devolve None para valor desconhecido.
    pub fn do_texto(texto: &str) -> Option<Self> {
        match texto.trim().to_lowercase().as_str() {
            "alta" => Some(Self::Alta),
            "media" => Some(Self::Media),
            "baixa" => Some(Self::Baixa),
            _ => None,
        }
    }

    /// Texto gravado no arquivo e enviado ao frontend.
    pub fn como_texto(self) -> &'static str {
        match self {
            Self::Alta => "alta",
            Self::Media => "media",
            Self::Baixa => "baixa",
        }
    }
}

/// Prazo da tarefa: só a data, ou data e hora.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrazoTarefa {
    SoData(NaiveDate),
    DataEHora(NaiveDateTime),
}

impl PrazoTarefa {
    /// Aceita `AAAA-MM-DD`, `AAAA-MM-DD HH:MM` ou `AAAA-MM-DDTHH:MM`.
    pub fn do_texto(texto: &str) -> Option<Self> {
        let limpo = texto.trim();
        if let Ok(data) = NaiveDate::parse_from_str(limpo, "%Y-%m-%d") {
            return Some(Self::SoData(data));
        }
        for formato_com_hora in ["%Y-%m-%d %H:%M", "%Y-%m-%dT%H:%M"] {
            if let Ok(data_e_hora) = NaiveDateTime::parse_from_str(limpo, formato_com_hora) {
                return Some(Self::DataEHora(data_e_hora));
            }
        }
        None
    }

    /// Texto gravado no arquivo e enviado ao frontend. Sempre `AAAA-MM-DD` ou `AAAA-MM-DD HH:MM`.
    pub fn como_texto(&self) -> String {
        match self {
            Self::SoData(data) => data.format("%Y-%m-%d").to_string(),
            Self::DataEHora(data_e_hora) => data_e_hora.format("%Y-%m-%d %H:%M").to_string(),
        }
    }

    /// Dia do prazo, sem a hora.
    pub fn dia(&self) -> NaiveDate {
        match self {
            Self::SoData(data) => *data,
            Self::DataEHora(data_e_hora) => data_e_hora.date(),
        }
    }

    /// Momento usado para avisar: a hora do prazo, ou 9h quando o prazo é só data.
    pub fn momento_do_aviso(&self) -> NaiveDateTime {
        match self {
            Self::SoData(data) => {
                let hora = NaiveTime::from_hms_opt(HORA_DO_PRAZO_SEM_HORARIO, 0, 0).unwrap_or(NaiveTime::MIN);
                data.and_time(hora)
            }
            Self::DataEHora(data_e_hora) => *data_e_hora,
        }
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn prazo_aceita_data_e_data_com_hora() {
        assert_eq!(
            PrazoTarefa::do_texto("2026-11-18"),
            Some(PrazoTarefa::SoData(NaiveDate::from_ymd_opt(2026, 11, 18).unwrap()))
        );
        assert!(matches!(PrazoTarefa::do_texto("2026-11-18 14:30"), Some(PrazoTarefa::DataEHora(_))));
        assert!(matches!(PrazoTarefa::do_texto("2026-11-18T14:30"), Some(PrazoTarefa::DataEHora(_))));
    }

    #[test]
    fn prazo_recusa_texto_fora_do_formato() {
        assert!(PrazoTarefa::do_texto("amanhã").is_none());
        assert!(PrazoTarefa::do_texto("18/11/2026").is_none());
    }

    #[test]
    fn prazo_so_data_avisa_as_nove_da_manha() {
        let prazo = PrazoTarefa::do_texto("2026-11-18").unwrap();
        assert_eq!(prazo.momento_do_aviso().format("%H:%M").to_string(), "09:00");
    }

    #[test]
    fn prioridade_alta_vem_antes_na_ordenacao() {
        assert!(PrioridadeTarefa::Alta < PrioridadeTarefa::Media);
        assert!(PrioridadeTarefa::Media < PrioridadeTarefa::Baixa);
    }

    #[test]
    fn status_desconhecido_nao_e_lido() {
        assert!(StatusTarefa::do_texto("talvez").is_none());
        assert!(StatusTarefa::Feita.esta_concluida());
        assert!(!StatusTarefa::Fazendo.esta_concluida());
    }
}
