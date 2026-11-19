//! Aviso quando o agente termina uma rodada: notificação do sistema. Se a ilha estiver em foco,
//! a pessoa já está lendo a resposta, então não há aviso. Este módulo não guarda nada.

use tauri::Manager;
use tauri_plugin_notification::NotificationExt;

/// Título da notificação. Fica igual nos avisos de fim de rodada.
const TITULO_DO_AVISO: &str = "Agente terminou";

/// Limite do trecho da resposta mostrado na notificação. O sistema corta textos longos de qualquer forma.
const LIMITE_DO_CORPO_EM_CARACTERES: usize = 120;

/// Notifica o fim da rodada. Falha ao notificar não atrapalha o pedido, então o erro é ignorado.
pub fn avisar_fim_da_rodada(app: &tauri::AppHandle, resultado: &Result<String, String>) {
    let ilha_em_foco = app
        .get_webview_window("ilha")
        .and_then(|janela| janela.is_focused().ok())
        .unwrap_or(false);
    if ilha_em_foco {
        return;
    }
    let _ = app
        .notification()
        .builder()
        .title(TITULO_DO_AVISO)
        .body(corpo_do_aviso(resultado))
        .show();
}

/// Primeira linha da resposta, cortada no limite. Se a rodada falhou, mostra o motivo.
pub(crate) fn corpo_do_aviso(resultado: &Result<String, String>) -> String {
    match resultado {
        Ok(texto) => {
            let primeira_linha: String = texto
                .trim()
                .lines()
                .next()
                .unwrap_or("")
                .chars()
                .take(LIMITE_DO_CORPO_EM_CARACTERES)
                .collect();
            if primeira_linha.is_empty() { "Resposta pronta.".into() } else { primeira_linha }
        }
        Err(erro) => format!("Falhou: {erro}"),
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn corpo_mostra_so_a_primeira_linha_da_resposta() {
        let resultado: Result<String, String> = Ok("Criei a tarefa.\nDetalhes abaixo.".into());
        assert_eq!(corpo_do_aviso(&resultado), "Criei a tarefa.");
    }

    #[test]
    fn corpo_de_falha_mostra_o_motivo() {
        let resultado: Result<String, String> = Err("sem resposta".into());
        assert_eq!(corpo_do_aviso(&resultado), "Falhou: sem resposta");
    }
}
