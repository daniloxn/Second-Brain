//! Repositório de tarefas: é o único módulo que lê e grava arquivos de tarefa.
//! Cada tarefa é um arquivo `.md` direto na pasta `tarefas/` do cofre.
//! Recebe a raiz do cofre (já canonizada) em vez do app, para o servidor MCP usar o mesmo código.

use super::dominio::Tarefa;
use crate::cofre::caminhos::caminho_dentro_da_raiz;
use std::fs;
use std::path::{Path, PathBuf};

/// Nome da pasta de tarefas dentro do cofre (ver PLANO.md, seção 3).
const NOME_DA_PASTA_DE_TAREFAS: &str = "tarefas";

/// Pasta `tarefas/` do cofre em `raiz`. Cria a pasta se ela ainda não existir.
pub(crate) fn pasta_das_tarefas(raiz: &Path) -> Result<PathBuf, String> {
    let pasta = raiz.join(NOME_DA_PASTA_DE_TAREFAS);
    fs::create_dir_all(&pasta).map_err(|erro| erro.to_string())?;
    Ok(pasta)
}

/// Todas as tarefas da pasta `tarefas/`. Arquivos que não são `.md` e arquivos ocultos são ignorados.
pub(crate) fn ler_todas_as_tarefas(raiz: &Path) -> Result<Vec<Tarefa>, String> {
    let pasta = pasta_das_tarefas(raiz)?;
    let entradas = fs::read_dir(&pasta).map_err(|erro| erro.to_string())?;

    let mut tarefas = Vec::new();
    for entrada in entradas.flatten() {
        let nome_do_arquivo = entrada.file_name().to_string_lossy().into_owned();
        if nome_do_arquivo.starts_with('.') || !nome_do_arquivo.to_lowercase().ends_with(".md") {
            continue;
        }
        let Ok(texto) = fs::read_to_string(entrada.path()) else {
            continue;
        };
        tarefas.push(Tarefa::do_arquivo(&nome_do_arquivo, &texto));
    }
    tarefas.sort_by(|primeira, segunda| primeira.id.cmp(&segunda.id));
    Ok(tarefas)
}

/// Lê uma tarefa pelo id (nome do arquivo). Recusa ids com separador de pasta.
pub(crate) fn ler_tarefa(raiz: &Path, id: &str) -> Result<Tarefa, String> {
    let caminho = caminho_da_tarefa(raiz, id)?;
    let texto = fs::read_to_string(&caminho).map_err(|erro| format!("tarefa não encontrada: {erro}"))?;
    Ok(Tarefa::do_arquivo(id, &texto))
}

/// Grava a tarefa substituindo o arquivo existente.
pub(crate) fn gravar_tarefa(raiz: &Path, tarefa: &Tarefa) -> Result<(), String> {
    let caminho = caminho_da_tarefa(raiz, &tarefa.id)?;
    fs::write(&caminho, tarefa.como_arquivo()).map_err(|erro| erro.to_string())
}

/// Grava uma tarefa nova. Recusa se já existir um arquivo com o mesmo id.
pub(crate) fn gravar_tarefa_nova(raiz: &Path, tarefa: &Tarefa) -> Result<(), String> {
    let caminho = caminho_da_tarefa(raiz, &tarefa.id)?;
    if caminho.exists() {
        return Err("já existe uma tarefa com esse nome".into());
    }
    fs::write(&caminho, tarefa.como_arquivo()).map_err(|erro| erro.to_string())
}

/// Remove uma tarefa existente depois de validar seu id e o caminho dentro do cofre.
pub(crate) fn apagar_tarefa(raiz: &Path, id: &str) -> Result<(), String> {
    let caminho = caminho_da_tarefa(raiz, id)?;
    fs::remove_file(caminho).map_err(|erro| format!("não foi possível apagar a tarefa: {erro}"))
}

/// Id livre para um título: o slug do título, ou o slug com número se já existir arquivo igual.
pub(crate) fn gerar_id_livre(raiz: &Path, slug_base: &str) -> Result<String, String> {
    let pasta = pasta_das_tarefas(raiz)?;
    let mut numero = 1;
    loop {
        let id_candidato = if numero == 1 {
            format!("{slug_base}.md")
        } else {
            format!("{slug_base}-{numero}.md")
        };
        if !pasta.join(&id_candidato).exists() {
            return Ok(id_candidato);
        }
        numero += 1;
    }
}

/// Caminho do arquivo de uma tarefa, com a checagem de que ele fica dentro de `tarefas/`.
/// O id não pode trazer pasta (`/`, `\`), não pode ser oculto e precisa terminar em `.md`.
fn caminho_da_tarefa(raiz: &Path, id: &str) -> Result<PathBuf, String> {
    if id.contains('/') || id.contains('\\') || id.starts_with('.') || !id.to_lowercase().ends_with(".md") {
        return Err(format!("identificador de tarefa inválido: \"{id}\""));
    }
    let pasta = pasta_das_tarefas(raiz)?;
    let caminho = pasta.join(id);
    // Confere também pelo cofre, como as outras funcionalidades.
    caminho_dentro_da_raiz(raiz, &caminho)?;
    Ok(caminho)
}
