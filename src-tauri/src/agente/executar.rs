//! Roda o CLI de um provedor e confere se ele está instalado. É o único módulo do agente
//! que cria processos. Não interpreta o pedido: só executa e entrega a saída.

use super::provedor::ProvedorAgente;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};

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

/// Roda o provedor no cofre, encaminha trechos incrementais e devolve a resposta completa.
pub fn executar_provedor(
    provedor: &dyn ProvedorAgente,
    prompt: &str,
    pasta_do_cofre: &str,
    mut ao_receber_trecho: impl FnMut(&str),
) -> Result<String, String> {
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

    if provedor.transmite_resposta() {
        executar_com_transmissao(provedor, processo_iniciado, &mut ao_receber_trecho)
    } else {
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
}

/// Lê o stdout evento por evento enquanto drena stderr em paralelo para não travar o processo.
fn executar_com_transmissao(
    provedor: &dyn ProvedorAgente,
    mut processo: Child,
    ao_receber_trecho: &mut dyn FnMut(&str),
) -> Result<String, String> {
    let stdout = processo.stdout.take().ok_or("stdout do agente indisponível")?;
    let stderr = processo.stderr.take().ok_or("stderr do agente indisponível")?;
    let leitor_de_erros = std::thread::spawn(move || {
        let mut conteudo = Vec::new();
        let resultado = stderr.take(2 * 1024 * 1024).read_to_end(&mut conteudo);
        (resultado, conteudo)
    });

    let mut leitor = BufReader::new(stdout);
    let mut linha = String::new();
    let mut saida_completa = String::new();
    let mut erro_do_evento = None;
    loop {
        linha.clear();
        match leitor.read_line(&mut linha) {
            Ok(0) => break,
            Ok(_) => {
                saida_completa.push_str(&linha);
                match provedor.ler_trecho_de_resposta(&linha) {
                    Ok(Some(trecho)) => ao_receber_trecho(&trecho),
                    Ok(None) => {}
                    Err(erro) => {
                        erro_do_evento.get_or_insert(erro);
                    }
                };
            }
            Err(erro) => {
                let _ = processo.kill();
                let _ = processo.wait();
                return Err(format!("não foi possível ler a resposta incremental do agente: {erro}"));
            }
        }
    }

    let status = processo.wait().map_err(|erro| format!("{} não terminou: {erro}", provedor.nome()))?;
    let (leitura_de_erros, conteudo_de_erros) = leitor_de_erros
        .join()
        .unwrap_or_else(|_| (Err(std::io::Error::other("leitor de erros interrompido")), Vec::new()));
    let stderr = String::from_utf8_lossy(&conteudo_de_erros).trim().to_string();
    if let Err(erro) = leitura_de_erros {
        return Err(format!("não foi possível ler o erro do agente: {erro}"));
    }
    if let Some(erro) = erro_do_evento {
        return Err(format!("{} falhou: {erro}", provedor.nome()));
    }
    if !status.success() {
        let motivo = provedor.ler_resposta(&saida_completa).err().unwrap_or(stderr);
        return Err(format!("{} falhou: {motivo}", provedor.nome()));
    }
    provedor.ler_resposta(&saida_completa)
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
