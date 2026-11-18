//! Lê e cria pastas de projeto no cofre, com modelos Markdown para o agente.
//! Arquivos ausentes são devolvidos vazios, sem serem criados na leitura.

use crate::cofre::caminhos::{caminho_dentro_do_cofre, raiz_do_cofre_ativo};
use crate::cofre::criacao::validar_nome_de_pasta;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

const LIMITE_DE_CADA_DOCUMENTO: u64 = 2 * 1024 * 1024;

/// Documentos de contexto de um projeto apresentados na interface.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjetoDoCofre {
    pub nome: String,
    pub caminho_relativo: String,
    pub projeto: String,
    pub mapa: String,
    pub log: String,
}

/// Lista pastas de projeto válidas e carrega seus três documentos Markdown.
pub(super) fn listar_projetos_do_cofre(app: &tauri::AppHandle) -> Result<Vec<ProjetoDoCofre>, String> {
    let pasta_dos_projetos = pasta_de_projetos(app)?;
    let entradas = fs::read_dir(&pasta_dos_projetos).map_err(|erro| erro.to_string())?;
    let mut projetos = Vec::new();

    for entrada in entradas.flatten() {
        let tipo = entrada.file_type().map_err(|erro| erro.to_string())?;
        let nome = entrada.file_name().to_string_lossy().into_owned();
        if nome.starts_with('.') || !tipo.is_dir() {
            continue;
        }
        let pasta_do_projeto = entrada.path();
        caminho_dentro_do_cofre(app, &pasta_do_projeto.to_string_lossy())?;
        projetos.push(ler_projeto(&pasta_do_projeto, &nome)?);
    }
    projetos.sort_by(|primeiro, segundo| primeiro.nome.to_lowercase().cmp(&segundo.nome.to_lowercase()));
    Ok(projetos)
}

/// Cria um projeto com três documentos de contexto e apaga só a pasta recém-criada se falhar.
pub(super) fn criar_projeto_no_cofre(app: &tauri::AppHandle, nome_digitado: &str) -> Result<ProjetoDoCofre, String> {
    let nome = validar_nome_de_pasta(nome_digitado)?;
    let pasta_dos_projetos = pasta_de_projetos(app)?;
    let pasta_do_projeto = pasta_dos_projetos.join(&nome);
    caminho_dentro_do_cofre(app, &pasta_do_projeto.to_string_lossy())?;
    fs::create_dir(&pasta_do_projeto).map_err(|erro| format!("não foi possível criar o projeto: {erro}"))?;

    let modelos = [
        ("PROJETO.md", modelo_do_projeto(&nome)),
        ("MAPA.md", modelo_do_mapa()),
        ("LOG.md", modelo_do_log()),
    ];
    for (nome_do_arquivo, conteudo) in modelos {
        if let Err(erro) = fs::write(pasta_do_projeto.join(nome_do_arquivo), conteudo) {
            let _ = fs::remove_dir_all(&pasta_do_projeto);
            return Err(format!("não foi possível criar os documentos iniciais: {erro}"));
        }
    }
    ler_projeto(&pasta_do_projeto, &nome)
}

/// Confere e devolve a pasta `projetos/` do cofre ativo.
fn pasta_de_projetos(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let raiz = raiz_do_cofre_ativo(app)?;
    let pasta = raiz.join("projetos");
    fs::create_dir_all(&pasta).map_err(|erro| erro.to_string())?;
    caminho_dentro_do_cofre(app, &pasta.to_string_lossy())
}

/// Carrega os documentos que já existem, sem criar arquivos durante a navegação.
fn ler_projeto(pasta: &Path, nome: &str) -> Result<ProjetoDoCofre, String> {
    Ok(ProjetoDoCofre {
        nome: nome.to_string(),
        caminho_relativo: format!("projetos/{nome}"),
        projeto: ler_documento(&pasta.join("PROJETO.md"))?,
        mapa: ler_documento(&pasta.join("MAPA.md"))?,
        log: ler_documento(&pasta.join("LOG.md"))?,
    })
}

/// Lê um documento de contexto até 2 MB; documento ausente fica vazio no painel.
fn ler_documento(caminho: &Path) -> Result<String, String> {
    if !caminho.exists() {
        return Ok(String::new());
    }
    let metadados = fs::metadata(caminho).map_err(|erro| erro.to_string())?;
    if metadados.len() > LIMITE_DE_CADA_DOCUMENTO {
        return Err(format!("{} excede o limite de 2 MB", caminho.display()));
    }
    fs::read_to_string(caminho).map_err(|erro| format!("{} não é um Markdown legível: {erro}", caminho.display()))
}

/// Conteúdo inicial do resumo de projeto que o agente pode manter atualizado.
fn modelo_do_projeto(nome: &str) -> String {
    format!("# {nome}\n\n## Objetivo\n\nDescreva o que este projeto deve entregar.\n\n## Estado atual\n\nEm planejamento.\n\n## Próximos passos\n\n- [ ] Definir o primeiro passo\n")
}

/// Conteúdo inicial do mapa técnico que o agente pode manter atualizado.
fn modelo_do_mapa() -> String {
    "# Mapa técnico\n\n## Estrutura\n\nAdicione aqui os diretórios e arquivos importantes.\n\n## Fluxo principal\n\nDescreva como as partes principais se conectam.\n".into()
}

/// Conteúdo inicial do registro cronológico de trabalho do agente.
fn modelo_do_log() -> String {
    format!("# Log do projeto\n\nCriado em {}.\n", chrono::Local::now().format("%Y-%m-%d"))
}
