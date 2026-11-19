//! Aquecimento do agente: ao abrir o app, uma rodada com uma saudação roda em segundo plano,
//! para confirmar que o provedor responde. O resultado fica guardado, e a tela do agente o lê
//! quando abre. A rodada roda numa pasta temporária, fora do cofre, e não entra no histórico.

use super::contrato::ModoDeExecucao;
use super::executar::{comando_esta_instalado, executar_provedor};
use super::provedor::provedor_ativo;
use serde::Serialize;
use std::fs;
use std::sync::Mutex;
use tauri::{Emitter, Manager};

/// Nome do evento que avisa a tela quando a situação do aquecimento muda.
pub const NOME_DO_EVENTO_DE_AQUECIMENTO: &str = "agente-aquecimento";

/// Saudação enviada ao agente. Pede resposta curta e proíbe alterar arquivos.
const PEDIDO_DE_AQUECIMENTO: &str =
    "Olá! Confirme em uma frase curta, em português, que você está pronto para ajudar com o Segundo Cérebro. Não crie nem altere nenhum arquivo.";

/// Pasta da rodada de aquecimento, dentro da pasta temporária do sistema.
const NOME_DA_PASTA_DE_AQUECIMENTO: &str = "segundo-cerebro-aquecimento";

/// Etapa em que o aquecimento está.
#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SituacaoDoAquecimento {
    Aguardando,
    Aquecendo,
    Pronto,
    Falhou,
    NaoInstalado,
}

/// Situação do aquecimento. `mensagem` traz a resposta do agente quando está pronto,
/// e o motivo quando falhou ou quando o programa não está instalado.
#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AquecimentoDoAgente {
    pub situacao: SituacaoDoAquecimento,
    pub mensagem: Option<String>,
}

impl Default for AquecimentoDoAgente {
    fn default() -> Self {
        Self {
            situacao: SituacaoDoAquecimento::Aguardando,
            mensagem: None,
        }
    }
}

/// Último aquecimento conhecido. Fica no estado do Tauri, então a tela lê mesmo que
/// a rodada tenha terminado antes de ela abrir.
#[derive(Default)]
pub struct EstadoDoAquecimento(Mutex<AquecimentoDoAgente>);

impl EstadoDoAquecimento {
    /// Cópia da situação atual.
    pub fn atual(&self) -> AquecimentoDoAgente {
        self.0.lock().unwrap_or_else(|erro| erro.into_inner()).clone()
    }

    /// Marca como "aquecendo" se nenhuma rodada estiver rodando. Devolve false se já houver uma.
    fn marcar_como_aquecendo_se_livre(&self) -> bool {
        let mut situacao = self.0.lock().unwrap_or_else(|erro| erro.into_inner());
        if situacao.situacao == SituacaoDoAquecimento::Aquecendo {
            return false;
        }
        *situacao = AquecimentoDoAgente {
            situacao: SituacaoDoAquecimento::Aquecendo,
            mensagem: None,
        };
        true
    }

    fn guardar(&self, novo_aquecimento: &AquecimentoDoAgente) {
        *self.0.lock().unwrap_or_else(|erro| erro.into_inner()) = novo_aquecimento.clone();
    }
}

/// Inicia a rodada de aquecimento em segundo plano. Se já houver uma rodada, não faz nada.
/// Cada mudança de situação é avisada à tela pelo evento `NOME_DO_EVENTO_DE_AQUECIMENTO`.
pub fn iniciar_aquecimento(app: &tauri::AppHandle) {
    let estado = app.state::<EstadoDoAquecimento>();
    if !estado.marcar_como_aquecendo_se_livre() {
        return;
    }
    publicar(app, &estado.atual());

    let app_da_rodada = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let aquecimento = rodar_aquecimento();
        app_da_rodada.state::<EstadoDoAquecimento>().guardar(&aquecimento);
        publicar(&app_da_rodada, &aquecimento);
    });
}

/// Avisa a tela da nova situação. Falha ao avisar não atrapalha o aquecimento.
fn publicar(app: &tauri::AppHandle, aquecimento: &AquecimentoDoAgente) {
    let _ = app.emit(NOME_DO_EVENTO_DE_AQUECIMENTO, aquecimento);
}

/// Roda a saudação e devolve a situação final. Bloqueia a thread: chamar só fora da principal.
fn rodar_aquecimento() -> AquecimentoDoAgente {
    let provedor = provedor_ativo();
    if !comando_esta_instalado(provedor.comando()) {
        return AquecimentoDoAgente {
            situacao: SituacaoDoAquecimento::NaoInstalado,
            mensagem: Some(format!("o comando \"{}\" não foi encontrado no PATH", provedor.comando())),
        };
    }
    let resultado = pasta_de_aquecimento()
        // A saudação só responde: roda em modo de planejamento, sem permissão de edição.
        .and_then(|pasta| executar_provedor(provedor.as_ref(), PEDIDO_DE_AQUECIMENTO, &pasta, ModoDeExecucao::Planejar, |_| {}, |_| {}));
    interpretar_resultado(resultado)
}

/// Transforma o resultado da saudação na situação mostrada. Resposta vazia também conta como falha:
/// o objetivo é confirmar que o agente responde de verdade.
pub fn interpretar_resultado(resultado: Result<String, String>) -> AquecimentoDoAgente {
    match resultado {
        Ok(texto) if !texto.trim().is_empty() => AquecimentoDoAgente {
            situacao: SituacaoDoAquecimento::Pronto,
            mensagem: Some(texto.trim().to_string()),
        },
        Ok(_) => AquecimentoDoAgente {
            situacao: SituacaoDoAquecimento::Falhou,
            mensagem: Some("o agente respondeu sem texto".to_string()),
        },
        Err(motivo) => AquecimentoDoAgente {
            situacao: SituacaoDoAquecimento::Falhou,
            mensagem: Some(motivo),
        },
    }
}

/// Cria (se faltar) a pasta temporária da rodada e devolve o caminho dela.
fn pasta_de_aquecimento() -> Result<String, String> {
    let pasta = std::env::temp_dir().join(NOME_DA_PASTA_DE_AQUECIMENTO);
    fs::create_dir_all(&pasta).map_err(|erro| erro.to_string())?;
    Ok(pasta.to_string_lossy().into_owned())
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn resposta_com_texto_deixa_o_agente_pronto() {
        let aquecimento = interpretar_resultado(Ok("  Estou pronto.\n".to_string()));
        assert_eq!(aquecimento.situacao, SituacaoDoAquecimento::Pronto);
        assert_eq!(aquecimento.mensagem.as_deref(), Some("Estou pronto."));
    }

    #[test]
    fn resposta_vazia_conta_como_falha() {
        let aquecimento = interpretar_resultado(Ok("   ".to_string()));
        assert_eq!(aquecimento.situacao, SituacaoDoAquecimento::Falhou);
    }

    #[test]
    fn erro_do_provedor_vira_falha_com_o_motivo() {
        let aquecimento = interpretar_resultado(Err("sem capacidade".to_string()));
        assert_eq!(aquecimento.situacao, SituacaoDoAquecimento::Falhou);
        assert_eq!(aquecimento.mensagem.as_deref(), Some("sem capacidade"));
    }

    #[test]
    fn so_inicia_uma_rodada_por_vez() {
        let estado = EstadoDoAquecimento::default();
        assert!(estado.marcar_como_aquecendo_se_livre());
        assert!(!estado.marcar_como_aquecendo_se_livre());
    }
}
