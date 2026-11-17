//! Contrato entre o backend e o frontend para tarefas. Este arquivo define o formato que o
//! frontend conhece: nomes em camelCase e textos fixos para status, prioridade e situação.
//!
//! Regras para mudar este arquivo:
//! - Mudança compatível (campo novo opcional): atualizar `src/tarefas/contratoTarefas.ts`.
//! - Mudança que quebra quem lê (renomear ou remover campo): subir `VERSAO_DO_CONTRATO_DE_TAREFAS`
//!   e atualizar o frontend na mesma mudança.
//! - O teste `formato_da_tarefa_segue_o_contrato` precisa continuar passando.
//!
//! A conversão do que o frontend envia para o domínio fica em `entrada_do_contrato.rs`.

use super::dominio::Tarefa;
use super::situacao::{situacao_da_tarefa, SituacaoTarefa};
use super::valores::PrazoTarefa;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Versão atual do contrato. O frontend confere esse número antes de usar a lista.
pub const VERSAO_DO_CONTRATO_DE_TAREFAS: u32 = 1;

/// Resposta de `listar_tarefas`: a versão do contrato e as tarefas já ordenadas por urgência.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListaDeTarefasDto {
    pub versao: u32,
    pub tarefas: Vec<TarefaDto>,
}

/// Uma tarefa como o frontend a recebe. `situacao` já vem calculada pelo backend.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TarefaDto {
    pub id: String,
    pub titulo: String,
    pub descricao: String,
    /// "aberta" | "fazendo" | "feita" | "arquivada"
    pub status: String,
    /// "alta" | "media" | "baixa"
    pub prioridade: String,
    /// "AAAA-MM-DD" ou "AAAA-MM-DD HH:MM"; nulo quando não há prazo.
    pub prazo: Option<String>,
    pub origem: String,
    pub tags: Vec<String>,
    /// "atrasada" | "hoje" | "proxima" | "sem_prazo" | "concluida"
    pub situacao: String,
}

impl TarefaDto {
    /// Converte uma tarefa do domínio para o formato do contrato, calculando a situação em `agora`.
    pub fn de_tarefa(tarefa: &Tarefa, agora: NaiveDateTime) -> Self {
        let situacao: SituacaoTarefa = situacao_da_tarefa(tarefa, agora);
        Self {
            id: tarefa.id.clone(),
            titulo: tarefa.titulo.clone(),
            descricao: tarefa.descricao.trim().to_string(),
            status: tarefa.status.como_texto().to_string(),
            prioridade: tarefa.prioridade.como_texto().to_string(),
            prazo: tarefa.prazo.as_ref().map(PrazoTarefa::como_texto),
            origem: tarefa.origem.clone(),
            tags: tarefa.tags.clone(),
            situacao: situacao.como_texto().to_string(),
        }
    }
}

/// Dados enviados por `criar_tarefa`. Campos opcionais podem faltar no JSON.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NovaTarefaDto {
    pub titulo: String,
    #[serde(default)]
    pub descricao: String,
    #[serde(default)]
    pub prioridade: Option<String>,
    #[serde(default)]
    pub prazo: Option<String>,
    #[serde(default)]
    pub origem: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Dados enviados por `atualizar_tarefa`. Campo ausente significa "não mudar".
/// Para remover o prazo, envie `prazo: ""`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlteracaoTarefaDto {
    pub titulo: Option<String>,
    pub descricao: Option<String>,
    pub status: Option<String>,
    pub prioridade: Option<String>,
    pub prazo: Option<String>,
    pub origem: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[cfg(test)]
mod testes {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn formato_da_tarefa_segue_o_contrato() {
        let tarefa = Tarefa::do_arquivo("relatorio.md", "---\ntitulo: \"Relatório\"\nprazo: 2026-11-18\n---\nCorpo");
        let agora = NaiveDate::from_ymd_opt(2026, 11, 17).unwrap().and_hms_opt(8, 0, 0).unwrap();
        let json = serde_json::to_value(TarefaDto::de_tarefa(&tarefa, agora)).unwrap();

        // Estes nomes são o contrato. Se algum mudar, o frontend quebra.
        for chave in ["id", "titulo", "descricao", "status", "prioridade", "prazo", "origem", "tags", "situacao"] {
            assert!(json.get(chave).is_some(), "falta o campo {chave} no contrato");
        }
        assert_eq!(json["prazo"], "2026-11-18");
        assert_eq!(json["situacao"], "proxima");
        assert_eq!(json["status"], "aberta");
    }

    #[test]
    fn lista_inclui_versao_do_contrato() {
        let lista = ListaDeTarefasDto {
            versao: VERSAO_DO_CONTRATO_DE_TAREFAS,
            tarefas: Vec::new(),
        };
        let json = serde_json::to_value(lista).unwrap();
        assert_eq!(json["versao"], 1);
        assert!(json["tarefas"].is_array());
    }
}
