//! Comandos Tauri da funcionalidade notas. Cada comando valida a entrada,
//! chama a lógica do módulo certo e devolve o resultado.

use super::backlinks::{procurar_backlinks, Backlink};
use super::conflito::gravar_copia_do_conflito;
use super::indice::{e_arquivo_de_nota, listar_notas_do_cofre, NotaIndice};
use super::nome_da_nota::validar_nome_da_nota;
use super::versao::versao_do_arquivo;
use crate::cofre::caminhos::{caminho_dentro_do_cofre, texto_para_frontend};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

/// Texto de uma nota e a versão do arquivo no momento da leitura.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NotaLida {
    pub texto: String,
    pub versao: u64,
}

/// Resultado de um salvamento. `gravado: false` quer dizer que a nota mudou no disco
/// desde a versão conhecida: nada foi escrito, e `versao` é a versão atual do disco.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultadoDoSalvamento {
    pub gravado: bool,
    pub versao: u64,
}

/// Todas as notas do cofre ativo, para o índice e para resolver wikilinks.
#[tauri::command]
pub fn listar_notas(app: tauri::AppHandle) -> Result<Vec<NotaIndice>, String> {
    listar_notas_do_cofre(&app)
}

/// Texto completo de uma nota, com a versão do arquivo. Recusa arquivos que não são .md.
#[tauri::command]
pub fn ler_nota(app: tauri::AppHandle, caminho: String) -> Result<NotaLida, String> {
    let caminho_da_nota = caminho_dentro_do_cofre(&app, &caminho)?;
    if !e_arquivo_de_nota(&caminho_da_nota) {
        return Err("não é uma nota .md".into());
    }
    let texto = fs::read_to_string(&caminho_da_nota).map_err(|e| e.to_string())?;
    let versao = versao_do_arquivo(&caminho_da_nota)?;
    Ok(NotaLida { texto, versao })
}

/// Grava o texto inteiro de uma nota existente. Só grava se a versão do disco ainda for
/// a `versao_conhecida`; caso contrário devolve `gravado: false` e não toca no arquivo.
#[tauri::command]
pub fn salvar_nota(
    app: tauri::AppHandle,
    caminho: String,
    conteudo: String,
    versao_conhecida: u64,
) -> Result<ResultadoDoSalvamento, String> {
    let caminho_da_nota = caminho_dentro_do_cofre(&app, &caminho)?;
    if !e_arquivo_de_nota(&caminho_da_nota) {
        return Err("não é uma nota .md".into());
    }
    let versao_no_disco = versao_do_arquivo(&caminho_da_nota)?;
    if versao_no_disco != versao_conhecida {
        return Ok(ResultadoDoSalvamento { gravado: false, versao: versao_no_disco });
    }
    fs::write(&caminho_da_nota, conteudo).map_err(|e| e.to_string())?;
    let versao_depois_de_gravar = versao_do_arquivo(&caminho_da_nota)?;
    Ok(ResultadoDoSalvamento { gravado: true, versao: versao_depois_de_gravar })
}

/// Guarda em `_sistema/conflitos/` um texto que não pôde ser salvo na nota. Devolve o caminho da cópia.
#[tauri::command]
pub fn guardar_copia_do_conflito(app: tauri::AppHandle, caminho: String, conteudo: String) -> Result<String, String> {
    gravar_copia_do_conflito(&app, &caminho, &conteudo)
}

/// Cria uma nota nova com título inicial dentro de `pasta`. Recusa nome inválido
/// e nome que já existe nessa pasta. Devolve o caminho do arquivo criado.
#[tauri::command]
pub fn criar_nota(app: tauri::AppHandle, pasta: String, nome: String) -> Result<String, String> {
    let nome_validado = validar_nome_da_nota(&nome)?;

    let pasta_de_destino = caminho_dentro_do_cofre(&app, &pasta)?;
    if !pasta_de_destino.is_dir() {
        return Err("a pasta de destino não existe".into());
    }

    let caminho_da_nova_nota: PathBuf = pasta_de_destino.join(format!("{nome_validado}.md"));
    if caminho_da_nova_nota.exists() {
        return Err("já existe uma nota com esse nome nesta pasta".into());
    }
    fs::write(&caminho_da_nova_nota, format!("# {nome_validado}\n\n")).map_err(|e| e.to_string())?;
    Ok(texto_para_frontend(&caminho_da_nova_nota))
}

/// Notas que apontam por wikilink para a nota em `caminho`.
#[tauri::command]
pub fn backlinks(app: tauri::AppHandle, caminho: String) -> Result<Vec<Backlink>, String> {
    procurar_backlinks(&app, &caminho)
}
