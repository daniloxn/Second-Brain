//! Regras de negócio das tarefas: ordem de urgência, quando avisar, e como montar ou alterar
//! uma tarefa. Funções puras: recebem o "agora" de fora, então podem ser testadas sem relógio nem disco.

use super::dominio::{AlteracaoDeTarefa, NovaTarefa, Tarefa};
use super::valores::{PrazoTarefa, PrioridadeTarefa, StatusTarefa};
use super::validacao::{validar_tags, validar_titulo};
use chrono::{Duration, NaiveDateTime};
use std::cmp::Ordering;

/// Quantos minutos antes do prazo o aviso dispara.
pub const ANTECEDENCIA_DO_AVISO_EM_MINUTOS: i64 = 60;

/// Ordena a lista do mais urgente para o menos urgente.
/// Ordem: concluídas por último; depois prioridade (alta antes); depois prazo (mais cedo antes,
/// sem prazo por último); por fim o título, para o resultado ser sempre o mesmo.
pub fn ordenar_por_urgencia(tarefas: &mut [Tarefa]) {
    tarefas.sort_by(comparar_urgencia);
}

fn comparar_urgencia(primeira: &Tarefa, segunda: &Tarefa) -> Ordering {
    primeira
        .status
        .esta_concluida()
        .cmp(&segunda.status.esta_concluida())
        .then_with(|| primeira.prioridade.cmp(&segunda.prioridade))
        .then_with(|| comparar_prazos(&primeira.prazo, &segunda.prazo))
        .then_with(|| primeira.titulo.to_lowercase().cmp(&segunda.titulo.to_lowercase()))
}

/// Prazo mais cedo vem antes. Sem prazo fica depois de qualquer prazo.
fn comparar_prazos(primeiro: &Option<PrazoTarefa>, segundo: &Option<PrazoTarefa>) -> Ordering {
    match (primeiro, segundo) {
        (Some(prazo_um), Some(prazo_dois)) => prazo_um.momento_do_aviso().cmp(&prazo_dois.momento_do_aviso()),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

/// Verdadeiro quando a tarefa merece aviso agora: não está concluída, tem prazo,
/// e o instante de aviso (prazo menos a antecedência) já passou.
/// Tarefas já atrasadas também retornam verdadeiro; quem evita repetir o aviso é o agendador.
pub fn deve_avisar(tarefa: &Tarefa, agora: NaiveDateTime, antecedencia_em_minutos: i64) -> bool {
    if tarefa.status.esta_concluida() {
        return false;
    }
    let Some(prazo) = &tarefa.prazo else {
        return false;
    };
    let momento_do_aviso = prazo.momento_do_aviso() - Duration::minutes(antecedencia_em_minutos);
    agora >= momento_do_aviso
}

/// Monta uma tarefa nova a partir dos dados digitados. Valida título e tags.
pub fn montar_nova_tarefa(id: String, entrada: NovaTarefa) -> Result<Tarefa, String> {
    Ok(Tarefa {
        id,
        titulo: validar_titulo(&entrada.titulo)?,
        descricao: entrada.descricao,
        status: StatusTarefa::Aberta,
        prioridade: entrada.prioridade.unwrap_or(PrioridadeTarefa::Media),
        prazo: entrada.prazo,
        origem: entrada.origem.trim().to_string(),
        tags: validar_tags(&entrada.tags)?,
        // Todo arquivo de tarefa leva `tipo: tarefa`, como no PLANO.md.
        campos_extras: vec![("tipo".to_string(), "tarefa".to_string())],
    })
}

/// Aplica uma alteração parcial sobre a tarefa. Só os campos enviados mudam.
pub fn aplicar_alteracao(tarefa: &mut Tarefa, alteracao: AlteracaoDeTarefa) -> Result<(), String> {
    if let Some(titulo_novo) = alteracao.titulo {
        tarefa.titulo = validar_titulo(&titulo_novo)?;
    }
    if let Some(descricao_nova) = alteracao.descricao {
        tarefa.descricao = descricao_nova;
    }
    if let Some(status_novo) = alteracao.status {
        tarefa.status = status_novo;
    }
    if let Some(prioridade_nova) = alteracao.prioridade {
        tarefa.prioridade = prioridade_nova;
    }
    if let Some(prazo_novo) = alteracao.prazo {
        tarefa.prazo = prazo_novo;
    }
    if let Some(origem_nova) = alteracao.origem {
        tarefa.origem = origem_nova.trim().to_string();
    }
    if let Some(tags_novas) = alteracao.tags {
        tarefa.tags = validar_tags(&tags_novas)?;
    }
    Ok(())
}

#[cfg(test)]
mod testes {
    use super::*;
    use chrono::NaiveDate;

    fn momento(ano: i32, mes: u32, dia: u32, hora: u32, minuto: u32) -> NaiveDateTime {
        NaiveDate::from_ymd_opt(ano, mes, dia)
            .unwrap()
            .and_hms_opt(hora, minuto, 0)
            .unwrap()
    }

    fn tarefa_de_teste(titulo: &str, prazo: Option<&str>, prioridade: PrioridadeTarefa) -> Tarefa {
        Tarefa {
            id: format!("{titulo}.md"),
            titulo: titulo.to_string(),
            descricao: String::new(),
            status: StatusTarefa::Aberta,
            prioridade,
            prazo: prazo.and_then(PrazoTarefa::do_texto),
            origem: String::new(),
            tags: Vec::new(),
            campos_extras: Vec::new(),
        }
    }

    #[test]
    fn ordena_por_prioridade_depois_prazo_e_concluidas_por_ultimo() {
        let mut lista = vec![
            tarefa_de_teste("baixa-cedo", Some("2026-01-01"), PrioridadeTarefa::Baixa),
            tarefa_de_teste("alta-tarde", Some("2026-03-01"), PrioridadeTarefa::Alta),
            tarefa_de_teste("alta-cedo", Some("2026-02-01"), PrioridadeTarefa::Alta),
            tarefa_de_teste("media-sem-prazo", None, PrioridadeTarefa::Media),
        ];
        let mut concluida = tarefa_de_teste("feita", Some("2025-01-01"), PrioridadeTarefa::Alta);
        concluida.status = StatusTarefa::Feita;
        lista.push(concluida);

        ordenar_por_urgencia(&mut lista);
        let ordem: Vec<&str> = lista.iter().map(|tarefa| tarefa.titulo.as_str()).collect();
        assert_eq!(ordem, vec!["alta-cedo", "alta-tarde", "media-sem-prazo", "baixa-cedo", "feita"]);
    }

    #[test]
    fn avisa_na_antecedencia_e_nao_antes() {
        let tarefa = tarefa_de_teste("a", Some("2026-11-18 14:00"), PrioridadeTarefa::Alta);
        assert!(!deve_avisar(&tarefa, momento(2026, 11, 18, 12, 59), ANTECEDENCIA_DO_AVISO_EM_MINUTOS));
        assert!(deve_avisar(&tarefa, momento(2026, 11, 18, 13, 0), ANTECEDENCIA_DO_AVISO_EM_MINUTOS));
    }

    #[test]
    fn tarefa_concluida_nunca_avisa() {
        let mut tarefa = tarefa_de_teste("a", Some("2026-11-18 14:00"), PrioridadeTarefa::Alta);
        tarefa.status = StatusTarefa::Arquivada;
        assert!(!deve_avisar(&tarefa, momento(2026, 11, 19, 0, 0), ANTECEDENCIA_DO_AVISO_EM_MINUTOS));
    }

    #[test]
    fn alteracao_so_muda_os_campos_enviados() {
        let mut tarefa = tarefa_de_teste("original", Some("2026-11-18"), PrioridadeTarefa::Media);
        let alteracao = AlteracaoDeTarefa {
            status: Some(StatusTarefa::Fazendo),
            ..Default::default()
        };
        aplicar_alteracao(&mut tarefa, alteracao).unwrap();
        assert_eq!(tarefa.status, StatusTarefa::Fazendo);
        assert_eq!(tarefa.titulo, "original");
        assert!(tarefa.prazo.is_some());
    }

    #[test]
    fn alteracao_pode_remover_o_prazo() {
        let mut tarefa = tarefa_de_teste("a", Some("2026-11-18"), PrioridadeTarefa::Media);
        let alteracao = AlteracaoDeTarefa {
            prazo: Some(None),
            ..Default::default()
        };
        aplicar_alteracao(&mut tarefa, alteracao).unwrap();
        assert!(tarefa.prazo.is_none());
    }

    #[test]
    fn alteracao_recusa_titulo_vazio() {
        let mut tarefa = tarefa_de_teste("a", None, PrioridadeTarefa::Media);
        let alteracao = AlteracaoDeTarefa {
            titulo: Some("   ".to_string()),
            ..Default::default()
        };
        assert!(aplicar_alteracao(&mut tarefa, alteracao).is_err());
    }

    #[test]
    fn nova_tarefa_comeca_aberta_com_prioridade_media_por_padrao() {
        let entrada = NovaTarefa {
            titulo: "Entregar relatório".to_string(),
            ..Default::default()
        };
        let tarefa = montar_nova_tarefa("entregar-relatorio.md".to_string(), entrada).unwrap();
        assert_eq!(tarefa.status, StatusTarefa::Aberta);
        assert_eq!(tarefa.prioridade, PrioridadeTarefa::Media);
        assert_eq!(tarefa.campos_extras, vec![("tipo".to_string(), "tarefa".to_string())]);
    }
}
