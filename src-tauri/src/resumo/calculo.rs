//! Conta a semana a partir do histórico do agente e das datas de conclusão das tarefas.
//! Função pura: recebe os dados prontos, não lê o disco nem o relógio, então é testada direto.

use super::contrato::ResumoDaSemana;
use crate::agente::contrato::{RegistroDoAgente, TipoDeMudanca};
use chrono::{Duration, NaiveDate};
use std::collections::HashSet;

/// Quantos dias a janela do resumo tem, contando hoje.
const DIAS_DA_SEMANA: i64 = 7;

/// Resume a janela de sete dias que termina em `hoje`.
pub fn resumir_semana(registros: &[RegistroDoAgente], datas_de_conclusao: &[NaiveDate], hoje: NaiveDate) -> ResumoDaSemana {
    let inicio = hoje - Duration::days(DIAS_DA_SEMANA - 1);
    let mut resumo = ResumoDaSemana {
        inicio: inicio.format("%Y-%m-%d").to_string(),
        fim: hoje.format("%Y-%m-%d").to_string(),
        pedidos_do_agente: 0,
        pedidos_com_tempo_medido: 0,
        tempo_do_agente_em_ms: 0,
        linhas_alteradas_pelo_agente: 0,
        notas_criadas_pelo_agente: 0,
        notas_alteradas_pelo_agente: 0,
        tarefas_criadas_pelo_agente: 0,
        tarefas_concluidas: datas_de_conclusao.iter().filter(|data| dentro_da_janela(**data, inicio, hoje)).count(),
    };
    for registro in registros {
        let Some(dia) = dia_do_registro(registro) else { continue };
        if !dentro_da_janela(dia, inicio, hoje) {
            continue;
        }
        resumo.pedidos_do_agente += 1;
        if registro.duracao_ms > 0 {
            resumo.pedidos_com_tempo_medido += 1;
            resumo.tempo_do_agente_em_ms += registro.duracao_ms;
        }
        // Pedido desfeito já não conta como mudança feita pelo agente.
        if !registro.desfeito {
            somar_mudancas(&mut resumo, registro);
        }
    }
    resumo
}

/// Soma as linhas e os arquivos mudados por um registro.
fn somar_mudancas(resumo: &mut ResumoDaSemana, registro: &RegistroDoAgente) {
    for mudanca in &registro.mudancas {
        let e_tarefa = mudanca.caminho.starts_with("tarefas/");
        match mudanca.tipo {
            TipoDeMudanca::Criado if e_tarefa => resumo.tarefas_criadas_pelo_agente += 1,
            TipoDeMudanca::Criado if mudanca.caminho.ends_with(".md") => resumo.notas_criadas_pelo_agente += 1,
            TipoDeMudanca::Alterado if mudanca.caminho.ends_with(".md") && !e_tarefa => resumo.notas_alteradas_pelo_agente += 1,
            _ => {}
        }
        if matches!(mudanca.tipo, TipoDeMudanca::Criado | TipoDeMudanca::Alterado | TipoDeMudanca::Apagado) {
            resumo.linhas_alteradas_pelo_agente += linhas_trocadas(mudanca.antes.as_deref(), mudanca.depois.as_deref());
        }
    }
}

/// Quantas linhas saíram de `antes` e quantas entraram em `depois`. Linha repetida conta uma vez só.
fn linhas_trocadas(antes: Option<&str>, depois: Option<&str>) -> usize {
    let linhas_antes: HashSet<&str> = antes.unwrap_or("").lines().collect();
    let linhas_depois: HashSet<&str> = depois.unwrap_or("").lines().collect();
    linhas_antes.difference(&linhas_depois).count() + linhas_depois.difference(&linhas_antes).count()
}

/// Dia do registro. O campo `quando` começa com `AAAA-MM-DD`. Registro sem data legível fica de fora.
fn dia_do_registro(registro: &RegistroDoAgente) -> Option<NaiveDate> {
    registro.quando.get(..10).and_then(|dia| NaiveDate::parse_from_str(dia, "%Y-%m-%d").ok())
}

/// Janela fechada: o início e o fim contam.
fn dentro_da_janela(dia: NaiveDate, inicio: NaiveDate, fim: NaiveDate) -> bool {
    dia >= inicio && dia <= fim
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::agente::contrato::{ModoDeExecucao, MudancaNoCofre};

    fn dia(texto: &str) -> NaiveDate {
        NaiveDate::parse_from_str(texto, "%Y-%m-%d").unwrap()
    }

    fn registro(quando: &str, duracao_ms: u64, mudancas: Vec<MudancaNoCofre>) -> RegistroDoAgente {
        RegistroDoAgente {
            id: format!("ag-{quando}"),
            quando: format!("{quando} 10:00:00"),
            pedido: "pedido".into(),
            provedor: "OpenCode".into(),
            resposta: "ok".into(),
            mudancas,
            desfeito: false,
            modo: ModoDeExecucao::Executar,
            duracao_ms,
        }
    }

    fn mudanca(caminho: &str, tipo: TipoDeMudanca, antes: Option<&str>, depois: Option<&str>) -> MudancaNoCofre {
        MudancaNoCofre { caminho: caminho.into(), tipo, antes: antes.map(String::from), depois: depois.map(String::from) }
    }

    #[test]
    fn janela_tem_sete_dias_contando_hoje() {
        let hoje = dia("2026-11-19");
        let registros = vec![
            registro("2026-11-13", 1000, vec![]),
            registro("2026-11-12", 1000, vec![]),
        ];
        let resumo = resumir_semana(&registros, &[], hoje);
        assert_eq!(resumo.inicio, "2026-11-13");
        assert_eq!(resumo.fim, "2026-11-19");
        assert_eq!(resumo.pedidos_do_agente, 1);
    }

    #[test]
    fn tempo_so_soma_rodadas_com_medicao() {
        let hoje = dia("2026-11-19");
        let registros = vec![registro("2026-11-18", 2500, vec![]), registro("2026-11-17", 0, vec![])];
        let resumo = resumir_semana(&registros, &[], hoje);
        assert_eq!(resumo.pedidos_do_agente, 2);
        assert_eq!(resumo.pedidos_com_tempo_medido, 1);
        assert_eq!(resumo.tempo_do_agente_em_ms, 2500);
    }

    #[test]
    fn conta_notas_tarefas_e_linhas_trocadas() {
        let hoje = dia("2026-11-19");
        let registros = vec![registro(
            "2026-11-18",
            0,
            vec![
                mudanca("notas/ideia.md", TipoDeMudanca::Criado, None, Some("a\nb")),
                mudanca("notas/horta.md", TipoDeMudanca::Alterado, Some("a\nb"), Some("a\nc")),
                mudanca("tarefas/pagar.md", TipoDeMudanca::Criado, None, Some("x")),
            ],
        )];
        let resumo = resumir_semana(&registros, &[], hoje);
        assert_eq!(resumo.notas_criadas_pelo_agente, 1);
        assert_eq!(resumo.notas_alteradas_pelo_agente, 1);
        assert_eq!(resumo.tarefas_criadas_pelo_agente, 1);
        // Criada: 2 linhas entraram. Alterada: "b" saiu e "c" entrou.
        assert_eq!(resumo.linhas_alteradas_pelo_agente, 2 + 2 + 1);
    }

    #[test]
    fn pedido_desfeito_nao_conta_mudanca() {
        let hoje = dia("2026-11-19");
        let mut desfeito = registro("2026-11-18", 0, vec![mudanca("notas/x.md", TipoDeMudanca::Criado, None, Some("a"))]);
        desfeito.desfeito = true;
        let resumo = resumir_semana(&[desfeito], &[], hoje);
        assert_eq!(resumo.notas_criadas_pelo_agente, 0);
        assert_eq!(resumo.pedidos_do_agente, 1);
    }

    #[test]
    fn tarefas_concluidas_so_contam_dentro_da_janela() {
        let hoje = dia("2026-11-19");
        let datas = [dia("2026-11-19"), dia("2026-11-13"), dia("2026-11-12")];
        let resumo = resumir_semana(&[], &datas, hoje);
        assert_eq!(resumo.tarefas_concluidas, 2);
    }
}
