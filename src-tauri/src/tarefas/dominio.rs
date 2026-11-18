//! Modelo de uma tarefa como vive na memória, e a conversão entre o arquivo `.md` e esse modelo.
//! Não sabe onde a tarefa está salva nem como ela aparece na tela.

use super::frontmatter::{self, citar, descitar, Frontmatter};
use super::valores::{PrazoTarefa, PrioridadeTarefa, StatusTarefa};
use chrono::NaiveDate;

/// Chaves do frontmatter que o app interpreta. Qualquer outra chave é guardada e devolvida tal como está.
const CHAVES_INTERPRETADAS: &[&str] = &["titulo", "status", "prioridade", "prazo", "origem", "tags", "recorrencia", "ultima_conclusao"];

/// Tarefa completa. O `id` é o nome do arquivo `.md` dentro de `tarefas/`.
#[derive(Debug, Clone, PartialEq)]
pub struct Tarefa {
    pub id: String,
    pub titulo: String,
    /// Corpo do Markdown, abaixo do frontmatter.
    pub descricao: String,
    pub status: StatusTarefa,
    pub prioridade: PrioridadeTarefa,
    pub prazo: Option<PrazoTarefa>,
    pub origem: String,
    pub tags: Vec<String>,
    pub recorrencia_diaria: bool,
    pub ultima_conclusao: Option<NaiveDate>,
    /// Chaves que o app não interpreta. São gravadas de volta sem alteração.
    pub campos_extras: Vec<(String, String)>,
}

/// Dados para criar uma tarefa. Já vêm convertidos para os tipos do domínio.
#[derive(Debug, Clone, Default)]
pub struct NovaTarefa {
    pub titulo: String,
    pub descricao: String,
    pub prioridade: Option<PrioridadeTarefa>,
    pub prazo: Option<PrazoTarefa>,
    pub origem: String,
    pub tags: Vec<String>,
    pub recorrencia_diaria: bool,
}

/// Alterações parciais de uma tarefa. Campo `None` significa "não mudar".
/// Em `prazo`, `Some(None)` significa "remover o prazo".
#[derive(Debug, Clone, Default)]
pub struct AlteracaoDeTarefa {
    pub titulo: Option<String>,
    pub descricao: Option<String>,
    pub status: Option<StatusTarefa>,
    pub prioridade: Option<PrioridadeTarefa>,
    pub prazo: Option<Option<PrazoTarefa>>,
    pub origem: Option<String>,
    pub tags: Option<Vec<String>>,
    pub recorrencia_diaria: Option<bool>,
}

impl Tarefa {
    /// Lê uma tarefa a partir do texto de um arquivo `.md`.
    /// Valores inválidos viram o padrão; sem título, usa o nome do arquivo.
    pub fn do_arquivo(id: &str, texto_do_arquivo: &str) -> Self {
        let separado: Frontmatter = frontmatter::separar(texto_do_arquivo);
        let valor_de = |chave: &str| -> Option<&str> {
            separado
                .pares
                .iter()
                .find(|(chave_do_par, _)| chave_do_par == chave)
                .map(|(_, valor)| valor.as_str())
        };

        let titulo_do_arquivo = valor_de("titulo")
            .map(descitar)
            .filter(|titulo| !titulo.is_empty())
            .unwrap_or_else(|| id.trim_end_matches(".md").to_string());

        let campos_extras = separado
            .pares
            .iter()
            .filter(|(chave, _)| !CHAVES_INTERPRETADAS.contains(&chave.as_str()))
            .cloned()
            .collect();

        Self {
            id: id.to_string(),
            titulo: titulo_do_arquivo,
            descricao: separado.corpo,
            status: valor_de("status")
                .and_then(StatusTarefa::do_texto)
                .unwrap_or(StatusTarefa::Aberta),
            prioridade: valor_de("prioridade")
                .and_then(PrioridadeTarefa::do_texto)
                .unwrap_or(PrioridadeTarefa::Media),
            prazo: valor_de("prazo").and_then(PrazoTarefa::do_texto),
            origem: valor_de("origem").map(descitar).unwrap_or_default(),
            tags: valor_de("tags").map(ler_lista_de_tags).unwrap_or_default(),
            recorrencia_diaria: valor_de("recorrencia").is_some_and(|valor| valor == "diaria"),
            ultima_conclusao: valor_de("ultima_conclusao").and_then(|valor| NaiveDate::parse_from_str(valor, "%Y-%m-%d").ok()),
            campos_extras,
        }
    }

    /// Texto completo do arquivo `.md`: frontmatter gravado e corpo.
    pub fn como_arquivo(&self) -> String {
        frontmatter::montar(&Frontmatter {
            pares: self.pares_do_frontmatter(),
            corpo: self.descricao.clone(),
        })
    }

    /// Pares na ordem em que são gravados: campos interpretados primeiro, depois os extras.
    fn pares_do_frontmatter(&self) -> Vec<(String, String)> {
        let mut pares = vec![
            ("titulo".to_string(), citar(&self.titulo)),
            ("status".to_string(), self.status.como_texto().to_string()),
            ("prioridade".to_string(), self.prioridade.como_texto().to_string()),
        ];
        if let Some(prazo) = &self.prazo {
            pares.push(("prazo".to_string(), prazo.como_texto()));
        }
        if !self.origem.is_empty() {
            pares.push(("origem".to_string(), citar(&self.origem)));
        }
        if !self.tags.is_empty() {
            pares.push(("tags".to_string(), escrever_lista_de_tags(&self.tags)));
        }
        if self.recorrencia_diaria {
            pares.push(("recorrencia".to_string(), "diaria".to_string()));
        }
        if let Some(data) = self.ultima_conclusao {
            pares.push(("ultima_conclusao".to_string(), data.format("%Y-%m-%d").to_string()));
        }
        pares.extend(self.campos_extras.iter().cloned());
        pares
    }
}

/// Lê uma lista no formato `[a, b, c]`. Itens vazios são descartados.
fn ler_lista_de_tags(bruto: &str) -> Vec<String> {
    bruto
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(descitar)
        .filter(|item| !item.is_empty())
        .collect()
}

/// Escreve a lista no formato `[a, b, c]`.
fn escrever_lista_de_tags(tags: &[String]) -> String {
    format!("[{}]", tags.join(", "))
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn arquivo_sem_frontmatter_usa_nome_do_arquivo_e_valores_padrao() {
        let tarefa = Tarefa::do_arquivo("pagar-conta.md", "Corpo livre");
        assert_eq!(tarefa.titulo, "pagar-conta");
        assert_eq!(tarefa.status, StatusTarefa::Aberta);
        assert_eq!(tarefa.prioridade, PrioridadeTarefa::Media);
        assert!(tarefa.prazo.is_none());
        assert_eq!(tarefa.descricao, "Corpo livre");
    }

    #[test]
    fn valor_invalido_de_status_vira_padrao() {
        let tarefa = Tarefa::do_arquivo("x.md", "---\nstatus: talvez\n---\n");
        assert_eq!(tarefa.status, StatusTarefa::Aberta);
    }

    #[test]
    fn chave_desconhecida_sobrevive_a_ida_e_volta() {
        let original = "---\ntitulo: \"Relatório\"\nstatus: fazendo\nautor: maria\n---\nCorpo";
        let tarefa = Tarefa::do_arquivo("relatorio.md", original);
        let gravado = tarefa.como_arquivo();
        assert!(gravado.contains("autor: maria"));
        assert!(gravado.contains("status: fazendo"));
        assert!(gravado.ends_with("Corpo"));
    }

    #[test]
    fn tags_fazem_ida_e_volta() {
        let tarefa = Tarefa::do_arquivo("x.md", "---\ntags: [oficina, dados]\n---\n");
        assert_eq!(tarefa.tags, vec!["oficina", "dados"]);
        assert!(tarefa.como_arquivo().contains("tags: [oficina, dados]"));
    }
}
