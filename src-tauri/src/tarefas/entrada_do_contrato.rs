//! Conversão do que o frontend envia (textos) para os tipos do domínio.
//! Texto inválido vira erro com mensagem clara; o domínio nunca recebe texto cru.

use super::contrato::{AlteracaoTarefaDto, NovaTarefaDto};
use super::dominio::{AlteracaoDeTarefa, NovaTarefa};
use super::valores::{PrazoTarefa, PrioridadeTarefa, StatusTarefa};

impl NovaTarefaDto {
    /// Converte para o domínio. Prioridade ou prazo inválidos viram erro com a mensagem a mostrar.
    pub fn para_nova_tarefa(self) -> Result<NovaTarefa, String> {
        let prioridade = match self.prioridade.as_deref() {
            None | Some("") => None,
            Some(texto) => Some(
                PrioridadeTarefa::do_texto(texto)
                    .ok_or_else(|| format!("prioridade inválida: \"{texto}\""))?,
            ),
        };
        Ok(NovaTarefa {
            titulo: self.titulo,
            descricao: self.descricao,
            prioridade,
            prazo: converter_prazo_opcional(self.prazo.as_deref())?,
            origem: self.origem.unwrap_or_default(),
            tags: self.tags,
        })
    }
}

impl AlteracaoTarefaDto {
    /// Converte para o domínio, validando status, prioridade e prazo.
    pub fn para_alteracao(self) -> Result<AlteracaoDeTarefa, String> {
        let status = match self.status {
            None => None,
            Some(texto) => Some(
                StatusTarefa::do_texto(&texto).ok_or_else(|| format!("status inválido: \"{texto}\""))?,
            ),
        };
        let prioridade = match self.prioridade {
            None => None,
            Some(texto) => Some(
                PrioridadeTarefa::do_texto(&texto)
                    .ok_or_else(|| format!("prioridade inválida: \"{texto}\""))?,
            ),
        };
        // `None` = não mudar; `Some(None)` = remover o prazo; `Some(Some(_))` = trocar o prazo.
        let prazo = match self.prazo.as_deref() {
            None => None,
            Some(texto) => Some(converter_prazo_opcional(Some(texto))?),
        };

        Ok(AlteracaoDeTarefa {
            titulo: self.titulo,
            descricao: self.descricao,
            status,
            prioridade,
            prazo,
            origem: self.origem,
            tags: self.tags,
        })
    }
}

/// Texto vazio ou ausente vira "sem prazo". Texto preenchido precisa estar no formato aceito.
fn converter_prazo_opcional(texto: Option<&str>) -> Result<Option<PrazoTarefa>, String> {
    match texto.map(str::trim) {
        None | Some("") => Ok(None),
        Some(valor) => PrazoTarefa::do_texto(valor)
            .map(Some)
            .ok_or_else(|| format!("prazo inválido: \"{valor}\". Use AAAA-MM-DD ou AAAA-MM-DD HH:MM")),
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    fn alteracao_so_com_prazo(prazo: &str) -> AlteracaoTarefaDto {
        AlteracaoTarefaDto {
            titulo: None,
            descricao: None,
            status: None,
            prioridade: None,
            prazo: Some(prazo.to_string()),
            origem: None,
            tags: None,
        }
    }

    #[test]
    fn nova_tarefa_recusa_prioridade_desconhecida() {
        let dto = NovaTarefaDto {
            titulo: "x".into(),
            descricao: String::new(),
            prioridade: Some("urgentíssima".into()),
            prazo: None,
            origem: None,
            tags: Vec::new(),
        };
        assert!(dto.para_nova_tarefa().is_err());
    }

    #[test]
    fn alteracao_com_prazo_vazio_remove_o_prazo() {
        let alteracao = alteracao_so_com_prazo("").para_alteracao().unwrap();
        assert_eq!(alteracao.prazo, Some(None));
    }

    #[test]
    fn alteracao_com_prazo_invalido_gera_mensagem_clara() {
        let erro = alteracao_so_com_prazo("amanhã").para_alteracao().unwrap_err();
        assert!(erro.contains("AAAA-MM-DD"));
    }
}
