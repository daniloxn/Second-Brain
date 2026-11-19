//! Comandos Tauri do agente. Cada comando junta as peças: foto do cofre antes e depois,
//! prompt, provedor, histórico e desfazer. A lógica de cada peça fica no próprio módulo.

use super::aquecimento::{iniciar_aquecimento, AquecimentoDoAgente, EstadoDoAquecimento};
use super::aviso::avisar_fim_da_rodada;
use super::contrato::{EstadoDoProvedor, FalaDaConversa, ModoDeExecucao, RegistroDoAgente, TrechoDeResposta, NOME_DO_EVENTO_DE_PASSO, NOME_DO_EVENTO_DE_RESPOSTA_PARCIAL};
use super::desfazer::desfazer_registro;
use super::diferencas::comparar_fotos;
use super::executar::{comando_esta_instalado, executar_provedor};
use super::foto::tirar_foto;
use super::historico::{anexar_registro, ler_registros, montar_registro, regravar_registro};
use super::prompt::{montar_prompt, validar_pedido};
use super::provedor::{id_do_provedor_ativo, provedor_ativo, selecionar_provedor};
use crate::cofre::caminhos::{raiz_do_cofre_ativo, texto_para_frontend};
use std::fs;
use std::path::Path;
use tauri::Emitter;

/// Tamanho máximo das regras do cofre levadas ao prompt. Evita um AGENTS.md enorme estourar o pedido.
const LIMITE_DAS_REGRAS_EM_CARACTERES: usize = 8000;

/// Provedor de agente e se o programa dele está instalado (no PATH).
#[tauri::command]
pub fn estado_do_agente() -> EstadoDoProvedor {
    let provedor = provedor_ativo();
    EstadoDoProvedor {
        id: id_do_provedor_ativo(),
        nome: provedor.nome().to_string(),
        comando: provedor.comando().to_string(),
        instalado: comando_esta_instalado(provedor.comando()),
    }
}

/// Troca o provedor ativo nesta sessão e devolve o estado atualizado.
#[tauri::command]
pub fn definir_provedor_do_agente(id: String) -> Result<EstadoDoProvedor, String> {
    selecionar_provedor(&id)?;
    Ok(estado_do_agente())
}

/// Situação do aquecimento: se o agente já respondeu à saudação feita ao abrir o app.
#[tauri::command]
pub fn estado_do_aquecimento(estado: tauri::State<'_, EstadoDoAquecimento>) -> AquecimentoDoAgente {
    estado.atual()
}

/// Refaz o aquecimento, por exemplo depois de uma falha. Não faz nada se já estiver rodando.
#[tauri::command]
pub fn aquecer_agente(app: tauri::AppHandle) {
    iniciar_aquecimento(&app);
}

/// Envia um pedido ao agente. Roda em outra thread, porque uma rodada pode levar minutos.
/// `modo` diz se a rodada pode editar o cofre. `plano_aprovado` vai junto na rodada de execução do plano.
/// Devolve o registro do pedido, ou o erro do agente (as mudanças feitas ficam no histórico mesmo assim).
#[tauri::command]
pub async fn enviar_pedido_ao_agente(
    app: tauri::AppHandle,
    pedido: String,
    conversa_anterior: Vec<FalaDaConversa>,
    id_da_rodada: String,
    modo: ModoDeExecucao,
    plano_aprovado: Option<String>,
) -> Result<RegistroDoAgente, String> {
    let pedido_validado = validar_pedido(&pedido)?;
    let resultado = tauri::async_runtime::spawn_blocking(move || {
        executar_pedido(&app, &pedido_validado, &conversa_anterior, &id_da_rodada, modo, plano_aprovado.as_deref())
    }).await;
    resultado.map_err(|erro| erro.to_string())?
}

/// Histórico de pedidos e mudanças, do mais recente para o mais antigo.
#[tauri::command]
pub fn listar_historico_do_agente(app: tauri::AppHandle) -> Result<Vec<RegistroDoAgente>, String> {
    let raiz = raiz_do_cofre_ativo(&app)?;
    ler_registros(&raiz)
}

/// Desfaz um pedido do histórico e o marca como desfeito. Devolve o registro atualizado.
#[tauri::command]
pub fn desfazer_acao_do_agente(app: tauri::AppHandle, id: String) -> Result<RegistroDoAgente, String> {
    let raiz = raiz_do_cofre_ativo(&app)?;
    let mut registro = ler_registros(&raiz)?
        .into_iter()
        .find(|registro_lido| registro_lido.id == id)
        .ok_or("ação não encontrada no histórico")?;
    if registro.desfeito {
        return Err("esta ação já foi desfeita".into());
    }
    if registro.mudancas.is_empty() {
        return Err("este pedido não mudou nada no cofre".into());
    }
    desfazer_registro(&raiz, &registro)?;
    registro.desfeito = true;
    regravar_registro(&raiz, &registro)?;
    Ok(registro)
}

/// Pedido de ponta a ponta: foto de antes, rodada do agente, foto de depois, registro no log.
/// O registro é gravado mesmo quando o agente falha, para o que ele mudou não se perder.
fn executar_pedido(
    app: &tauri::AppHandle,
    pedido: &str,
    conversa_anterior: &[FalaDaConversa],
    id_da_rodada: &str,
    modo: ModoDeExecucao,
    plano_aprovado: Option<&str>,
) -> Result<RegistroDoAgente, String> {
    // Recusa antes de qualquer coisa: sem web, a consulta responderia sem pesquisar e sem avisar.
    if modo == ModoDeExecucao::Consultar && !provedor_ativo().suporta_consulta_com_web() {
        return Err(format!(
            "{} ainda não consulta a web. Escolha o Claude Code para consultar, ou outro modo.",
            provedor_ativo().nome()
        ));
    }
    let raiz = raiz_do_cofre_ativo(app)?;
    let pasta_do_cofre = texto_para_frontend(&raiz);
    let foto_de_antes = tirar_foto(&raiz)?;

    let provedor = provedor_ativo();
    let prompt = montar_prompt(
        pedido,
        conversa_anterior,
        ler_regras_do_cofre(&raiz).as_deref(),
        &agora_local(),
        &pasta_do_cofre,
        modo,
        plano_aprovado,
    );
    let id_do_evento = id_da_rodada.to_string();
    let id_do_passo = id_da_rodada.to_string();
    let inicio_da_rodada = std::time::Instant::now();
    let resultado = executar_provedor(
        provedor.as_ref(),
        &prompt,
        &pasta_do_cofre,
        modo,
        |trecho| {
            let _ = app.emit(
                NOME_DO_EVENTO_DE_RESPOSTA_PARCIAL,
                TrechoDeResposta { id_da_rodada: id_do_evento.clone(), texto: trecho.to_string() },
            );
        },
        |passo| {
            let _ = app.emit(
                NOME_DO_EVENTO_DE_PASSO,
                TrechoDeResposta { id_da_rodada: id_do_passo.clone(), texto: passo.to_string() },
            );
        },
    );

    let foto_de_depois = tirar_foto(&raiz)?;
    let mudancas = comparar_fotos(&foto_de_antes, &foto_de_depois);
    let mut registro = montar_registro(id_novo(), agora_local(), pedido, provedor.nome(), &resultado, mudancas, modo);
    registro.duracao_ms = u64::try_from(inicio_da_rodada.elapsed().as_millis()).unwrap_or(u64::MAX);
    anexar_registro(&raiz, &registro)?;
    avisar_fim_da_rodada(app, &resultado);

    resultado.map(|_| registro)
}

/// Regras escritas pelo usuário em `_sistema/AGENTS.md` (ou, em cofres antigos, `AGENTS.md` na raiz),
/// cortadas no limite. None se não houver arquivo em nenhum dos dois lugares.
fn ler_regras_do_cofre(raiz_do_cofre: &Path) -> Option<String> {
    let texto = fs::read_to_string(raiz_do_cofre.join("_sistema").join("AGENTS.md"))
        .or_else(|_| fs::read_to_string(raiz_do_cofre.join("AGENTS.md")))
        .ok()?;
    Some(texto.chars().take(LIMITE_DAS_REGRAS_EM_CARACTERES).collect())
}

/// Data e hora locais no formato que vai para o prompt e para o histórico.
fn agora_local() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Id do registro: a hora com milissegundos. Dois pedidos no mesmo milissegundo não acontecem na tela.
fn id_novo() -> String {
    format!("ag-{}", chrono::Local::now().format("%Y%m%d%H%M%S%3f"))
}
