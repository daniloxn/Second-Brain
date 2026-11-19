//! Comandos Tauri da funcionalidade cofre. Cada comando só recebe os dados, chama a lógica
//! do módulo certo e devolve o resultado.

use super::arvore::{montar_arvore_padrao, NoArvore};
use super::caminhos::{caminho_dentro_do_cofre, texto_para_frontend};
use super::criacao::{criar_cofre_novo, criar_pasta_no_cofre_ativo};
use super::estrutura::{criar_estrutura_padrao_do_cofre, e_cofre_do_app};
use crate::configuracao::{gravar_configuracao, ler_configuracao, Configuracao};
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Resposta de `abrir_cofre`. Se `precisa_confirmar` for verdadeiro, a pasta não é um cofre do app
/// e nada foi feito: o frontend precisa pedir a confirmação do usuário antes de repetir a chamada.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AberturaDoCofreDto {
    pub caminho: Option<String>,
    pub precisa_confirmar: bool,
}

/// Caminho do cofre salvo, se ele ainda for um cofre do app. Se a pasta sumiu ou
/// não tem mais a marca do app, devolve nulo para a tela inicial aparecer.
#[tauri::command]
pub fn cofre_atual(app: tauri::AppHandle) -> Result<Option<String>, String> {
    let cofre_salvo = ler_configuracao(&app)?.cofre;
    Ok(cofre_salvo.filter(|caminho| e_cofre_do_app(Path::new(caminho))))
}

/// Abre uma pasta como cofre. Pasta que não é do app só é usada se `converter_pasta` for verdadeiro,
/// e mesmo assim a conversão só cria as pastas que faltam, sem apagar nem mover nada.
#[tauri::command]
pub fn abrir_cofre(app: tauri::AppHandle, caminho: String, converter_pasta: bool) -> Result<AberturaDoCofreDto, String> {
    let pasta_escolhida = PathBuf::from(&caminho);
    if !pasta_escolhida.is_dir() {
        return Err("a pasta escolhida não existe".into());
    }
    if !e_cofre_do_app(&pasta_escolhida) && !converter_pasta {
        return Ok(AberturaDoCofreDto {
            caminho: None,
            precisa_confirmar: true,
        });
    }
    criar_estrutura_padrao_do_cofre(&pasta_escolhida)?;
    let caminho_canonico = texto_para_frontend(&pasta_escolhida.canonicalize().map_err(|erro| erro.to_string())?);
    gravar_cofre_ativo(&app, &caminho_canonico)?;
    Ok(AberturaDoCofreDto {
        caminho: Some(caminho_canonico),
        precisa_confirmar: false,
    })
}

/// Cria um cofre novo (pasta `nome` dentro de `pasta_pai`), com a estrutura padrão, e o deixa ativo.
#[tauri::command]
pub fn criar_cofre(app: tauri::AppHandle, pasta_pai: String, nome: String) -> Result<String, String> {
    let cofre_criado = criar_cofre_novo(Path::new(&pasta_pai), &nome)?;
    let caminho_canonico = texto_para_frontend(&cofre_criado.canonicalize().map_err(|erro| erro.to_string())?);
    gravar_cofre_ativo(&app, &caminho_canonico)?;
    Ok(caminho_canonico)
}

/// Cria uma pasta dentro do cofre ativo. Aceita vários níveis, como `projetos/app`.
#[tauri::command]
pub fn criar_pasta_no_cofre(app: tauri::AppHandle, caminho_relativo: String) -> Result<String, String> {
    criar_pasta_no_cofre_ativo(&app, &caminho_relativo)
}

/// Árvore de arquivos de uma pasta dentro do cofre ativo.
#[tauri::command]
pub fn listar_arvore(app: tauri::AppHandle, caminho: String) -> Result<NoArvore, String> {
    let pasta_alvo = caminho_dentro_do_cofre(&app, &caminho)?;
    if !pasta_alvo.is_dir() {
        return Err(format!("{} não existe", pasta_alvo.display()));
    }
    montar_arvore_padrao(&pasta_alvo)
}

/// Grava na configuração qual é o cofre ativo. Lê a configuração antes, para não apagar os outros campos (atalhos).
fn gravar_cofre_ativo(app: &tauri::AppHandle, caminho_do_cofre: &str) -> Result<(), String> {
    let mut configuracao: Configuracao = ler_configuracao(app)?;
    configuracao.cofre = Some(caminho_do_cofre.to_string());
    gravar_configuracao(app, &configuracao)?;
    let _ = tauri::Emitter::emit(app, "cofre-alterado", caminho_do_cofre);
    Ok(())
}
