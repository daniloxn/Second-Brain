//! Roda o CLI de um provedor e confere se ele está instalado. É o único módulo do agente
//! que cria processos. Não interpreta o pedido: só executa e entrega a saída.

use super::provedor::ProvedorAgente;
use std::io::Write;
use std::process::{Command, Stdio};

/// Flag do Windows que impede a abertura de uma janela de console para o processo filho.
#[cfg(windows)]
const SEM_JANELA_DE_CONSOLE: u32 = 0x0800_0000;

/// Confere se o comando existe no PATH. Usa `where` no Windows e `which` nos outros sistemas.
pub fn comando_esta_instalado(comando: &str) -> bool {
    let localizador = if cfg!(windows) { "where" } else { "which" };
    let mut processo = Command::new(localizador);
    processo.arg(comando).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    sem_janela_de_console(&mut processo);
    processo.status().map(|situacao| situacao.success()).unwrap_or(false)
}

/// Roda o provedor uma vez dentro de `pasta_do_cofre` e devolve o texto da resposta.
/// O prompt segue pela entrada padrão, então não sofre com limite de tamanho nem com aspas da linha de comando.
pub fn executar_provedor(provedor: &dyn ProvedorAgente, prompt: &str, pasta_do_cofre: &str) -> Result<String, String> {
    let mut argumentos = provedor.montar_argumentos(pasta_do_cofre);
    if provedor.prompt_como_argumento() {
        argumentos.push(prompt.to_string());
    }
    let mut processo = Command::new(provedor.comando());
    processo
        .args(&argumentos)
        .current_dir(pasta_do_cofre)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if provedor.prompt_como_argumento() {
        processo.stdin(Stdio::null());
    } else {
        processo.stdin(Stdio::piped());
    }
    if let Some(configuracao) = provedor.configuracao_de_execucao() {
        processo.env("OPENCODE_CONFIG_CONTENT", configuracao);
    }
    sem_janela_de_console(&mut processo);

    let mut processo_iniciado = processo
        .spawn()
        .map_err(|erro| format!("não foi possível iniciar {}: {erro}", provedor.nome()))?;
    if let Some(mut entrada) = processo_iniciado.stdin.take() {
        entrada
            .write_all(prompt.as_bytes())
            .map_err(|erro| format!("não foi possível enviar o pedido para {}: {erro}", provedor.nome()))?;
        // Ao sair deste bloco a entrada fecha, e o CLI entende que o prompt terminou.
    }

    let saida = processo_iniciado
        .wait_with_output()
        .map_err(|erro| format!("{} não terminou: {erro}", provedor.nome()))?;
    if !saida.status.success() {
        // O motivo mais útil costuma vir no JSON do stdout; o stderr é a alternativa.
        let motivo = provedor
            .ler_resposta(&String::from_utf8_lossy(&saida.stdout))
            .err()
            .unwrap_or_else(|| String::from_utf8_lossy(&saida.stderr).trim().to_string());
        return Err(format!("{} falhou: {motivo}", provedor.nome()));
    }
    provedor.ler_resposta(&String::from_utf8_lossy(&saida.stdout))
}

/// No Windows, um processo de console abriria uma janela preta. Esta flag evita isso.
fn sem_janela_de_console(processo: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        processo.creation_flags(SEM_JANELA_DE_CONSOLE);
    }
    #[cfg(not(windows))]
    {
        let _ = processo;
    }
}
