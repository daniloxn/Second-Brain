//! Roda o CLI de um provedor e confere se ele está instalado. É o único módulo do agente
//! que cria processos. Não interpreta o pedido: só executa e entrega a saída.

use super::contrato::ModoDeExecucao;
use super::provedor::ProvedorAgente;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};

/// Flag do Windows que impede a abertura de uma janela de console para o processo filho.
#[cfg(windows)]
const SEM_JANELA_DE_CONSOLE: u32 = 0x0800_0000;

/// Confere se o comando tem um executável utilizável no PATH.
pub fn comando_esta_instalado(comando: &str) -> bool {
    resolver_executavel(comando).is_ok()
}

/// Caminhos encontrados para o comando no PATH, um por linha. Usa `where` no Windows e `which` nos outros.
fn localizar_no_path(comando: &str) -> Vec<String> {
    let localizador = if cfg!(windows) { "where" } else { "which" };
    let mut processo = Command::new(localizador);
    processo.arg(comando).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null());
    sem_janela_de_console(&mut processo);
    let Ok(saida) = processo.output() else { return Vec::new(); };
    if !saida.status.success() {
        return Vec::new();
    }
    String::from_utf8_lossy(&saida.stdout)
        .lines()
        .map(|linha| linha.trim().to_string())
        .filter(|linha| !linha.is_empty())
        .collect()
}

/// Escolhe o caminho que o sistema consegue executar. No Windows, o npm instala um script `sh`
/// sem extensão que `std::process::Command` não abre; por isso `.exe` vem primeiro, depois
/// `.cmd`/`.bat`, e o arquivo sem extensão fica de fora.
fn escolher_caminho_executavel(caminhos: &[String]) -> Option<&str> {
    if !cfg!(windows) {
        return caminhos.first().map(String::as_str);
    }
    let eh_executavel = |caminho: &str| {
        let minusculo = caminho.to_ascii_lowercase();
        minusculo.ends_with(".exe") || minusculo.ends_with(".cmd") || minusculo.ends_with(".bat")
    };
    caminhos
        .iter()
        .find(|caminho| caminho.to_ascii_lowercase().ends_with(".exe"))
        .or_else(|| caminhos.iter().find(|caminho| eh_executavel(caminho.as_str())))
        .map(String::as_str)
}

/// Caminho executável do comando. Necessário no Windows porque os CLIs instalados por npm têm
/// um atalho `.cmd` que `std::process::Command` não encontra quando recebe só o nome.
fn resolver_executavel(comando: &str) -> Result<String, String> {
    let caminhos = localizar_no_path(comando);
    // O shim npm usa cmd.exe, que não aceita prompts com quebras de linha.
    // Execute o binário nativo do pacote diretamente, preservando o texto literal.
    #[cfg(windows)]
    if comando == "opencode" {
        if let Some(exe) = caminhos.iter().find(|p| p.to_ascii_lowercase().ends_with(".exe")) {
            return Ok(exe.clone());
        }
        for caminho in &caminhos {
            if let Some(pasta) = std::path::Path::new(caminho).parent() {
                let exe = pasta.join("node_modules/@opencode/cli/bin/opencode.exe");
                if exe.is_file() {
                    return Ok(exe.to_string_lossy().into_owned());
                }
            }
        }
        return Err("OpenCode foi encontrado, mas seu executável nativo opencode.exe não está disponível. Repare a instalação do OpenCode para enviar pedidos com várias linhas.".into());
    }
    escolher_caminho_executavel(&caminhos)
        .map(str::to_string)
        .ok_or_else(|| format!("o comando \"{comando}\" não foi encontrado no PATH"))
}

/// Roda o provedor no cofre, encaminha trechos e passos conforme chegam e devolve a resposta completa.
pub fn executar_provedor(
    provedor: &dyn ProvedorAgente,
    prompt: &str,
    pasta_do_cofre: &str,
    modo: ModoDeExecucao,
    mut ao_receber_trecho: impl FnMut(&str),
    mut ao_receber_passo: impl FnMut(&str),
) -> Result<String, String> {
    let mut argumentos = provedor.montar_argumentos(pasta_do_cofre, modo);
    if provedor.prompt_como_argumento() {
        argumentos.push(format!("{}{prompt}", provedor.prefixo_do_argumento_do_prompt()));
    }
    // Resolve o binário nativo do OpenCode antes de passar o prompt como argumento.
    let executavel = resolver_executavel(provedor.comando())?;
    let mut processo = Command::new(executavel);
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
    if let Some(configuracao) = provedor.configuracao_de_execucao(modo) {
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
        executar_com_transmissao(provedor, processo_iniciado, &mut ao_receber_trecho, &mut ao_receber_passo)
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
    ao_receber_passo: &mut dyn FnMut(&str),
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
                if let Some(passo) = provedor.ler_passo_de_resposta(&linha) {
                    ao_receber_passo(&passo);
                }
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

#[cfg(test)]
mod testes {
    use super::*;

    /// Validação opt-in com a instalação real, pelo mesmo caminho usado pelo app.
    #[cfg(windows)]
    #[test]
    #[ignore = "requer OpenCode instalado e autenticado; faz uma chamada ao modelo"]
    fn opencode_real_recebe_prompt_multilinha() {
        let pasta = std::env::temp_dir().join(format!("segundo-cerebro-opencode-{}", std::process::id()));
        std::fs::create_dir_all(&pasta).unwrap();
        let provedor = super::super::opencode::AdaptadorOpenCode;
        let mut trechos = String::new();
        let resposta = executar_provedor(
            &provedor,
            "Responda apenas OK. Não use ferramentas nem leia ou altere arquivos.\r\nTexto literal: \"aspas\" & | < > %PATH% ! ^\nFim do teste.",
            pasta.to_str().unwrap(),
            ModoDeExecucao::Executar,
            |trecho| trechos.push_str(trecho),
            |_passo| {},
        ).unwrap();
        assert_eq!(resposta.trim(), "OK");
        assert_eq!(trechos.trim(), "OK");
        let _ = std::fs::remove_dir(&pasta);
    }

    #[cfg(windows)]
    #[test]
    fn escolhe_o_atalho_cmd_quando_o_nome_sem_extensao_e_um_script() {
        let caminhos = vec![
            r"C:\npm\opencode".to_string(),
            r"C:\npm\opencode.cmd".to_string(),
        ];
        assert_eq!(
            escolher_caminho_executavel(&caminhos),
            Some(r"C:\npm\opencode.cmd")
        );
    }

    #[cfg(windows)]
    #[test]
    fn prefere_o_exe_quando_o_cmd_tambem_existe() {
        let caminhos = vec![
            r"C:\npm\codex.cmd".to_string(),
            r"C:\Program Files\codex.exe".to_string(),
        ];
        assert_eq!(
            escolher_caminho_executavel(&caminhos),
            Some(r"C:\Program Files\codex.exe")
        );
    }

    #[cfg(windows)]
    #[test]
    fn script_sem_extensao_sozinho_nao_conta_como_instalado() {
        let caminhos = vec![r"C:\npm\opencode".to_string()];
        assert_eq!(escolher_caminho_executavel(&caminhos), None);
    }

    #[cfg(not(windows))]
    #[test]
    fn em_outros_sistemas_usa_o_primeiro_caminho() {
        let caminhos = vec![
            "/usr/local/bin/opencode".to_string(),
            "/outro/opencode".to_string(),
        ];
        assert_eq!(
            escolher_caminho_executavel(&caminhos),
            Some("/usr/local/bin/opencode")
        );
    }
}
