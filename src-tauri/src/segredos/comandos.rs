//! Comandos Tauri do cofre de segredos: inicializar, listar, salvar, copiar e apagar.
//! Cada operação que abre dados exige a senha mestra e não a guarda.

use super::contrato::{CofreDeSegredos, NovoSegredo, ResumoDeSegredo, SegredoArmazenado};
use super::repositorio::{
    criar_cofre_de_segredos as inicializar_arquivo,
    existe_cofre_de_segredos,
    ler_cofre_de_segredos,
    salvar_cofre_de_segredos,
};
use std::sync::Mutex;
use tauri::State;
use zeroize::Zeroizing;

const TAMANHO_DA_SENHA_MESTRA: usize = 6;

/// Serializa leituras e gravações para dois comandos do app não sobrescreverem mudanças entre si.
#[derive(Default)]
pub struct EstadoDoCofreDeSegredos {
    trava: Mutex<()>,
}

/// Informa apenas se o arquivo cifrado existe, sem tentar abri-lo.
#[tauri::command]
pub fn estado_do_cofre_de_segredos(app: tauri::AppHandle, estado: State<'_, EstadoDoCofreDeSegredos>) -> Result<bool, String> {
    let _guarda = estado.trava.lock().map_err(|_| "o cofre de segredos está ocupado".to_string())?;
    existe_cofre_de_segredos(&app)
}

/// Cria o arquivo cifrado vazio. A nova senha mestra precisa ter exatamente seis caracteres.
#[tauri::command]
pub fn criar_cofre_de_segredos(app: tauri::AppHandle, senha_mestra: String, estado: State<'_, EstadoDoCofreDeSegredos>) -> Result<(), String> {
    let _guarda = estado.trava.lock().map_err(|_| "o cofre de segredos está ocupado".to_string())?;
    let senha_mestra = Zeroizing::new(senha_mestra);
    validar_senha_mestra(&senha_mestra)?;
    inicializar_arquivo(&app, &senha_mestra)
}

/// Desbloqueia o cofre e devolve só títulos, usuários e anotações, nunca as senhas.
#[tauri::command]
pub fn listar_segredos(app: tauri::AppHandle, senha_mestra: String, estado: State<'_, EstadoDoCofreDeSegredos>) -> Result<Vec<ResumoDeSegredo>, String> {
    let _guarda = estado.trava.lock().map_err(|_| "o cofre de segredos está ocupado".to_string())?;
    let senha_mestra = Zeroizing::new(senha_mestra);
    let cofre = ler_cofre_de_segredos(&app, &senha_mestra)?;
    Ok(resumos_dos_segredos(&cofre))
}

/// Adiciona uma credencial ao cofre depois de validar os campos obrigatórios.
#[tauri::command]
pub fn criar_segredo(
    app: tauri::AppHandle,
    senha_mestra: String,
    entrada: NovoSegredo,
    estado: State<'_, EstadoDoCofreDeSegredos>,
) -> Result<Vec<ResumoDeSegredo>, String> {
    let _guarda = estado.trava.lock().map_err(|_| "o cofre de segredos está ocupado".to_string())?;
    let senha_mestra = Zeroizing::new(senha_mestra);
    let NovoSegredo { titulo, usuario, senha, anotacoes } = entrada;
    let usuario = Zeroizing::new(usuario);
    let senha = Zeroizing::new(senha);
    let anotacoes = Zeroizing::new(anotacoes);
    let titulo = titulo.trim();
    if titulo.is_empty() || titulo.len() > 160 {
        return Err("o título precisa ter entre 1 e 160 caracteres".into());
    }
    if usuario.len() > 512 || senha.len() > 8192 || anotacoes.len() > 8192 {
        return Err("usuário, senha ou anotações excedem o limite permitido".into());
    }
    if senha.is_empty() {
        return Err("a senha do segredo não pode ficar vazia".into());
    }

    let mut cofre = ler_cofre_de_segredos(&app, &senha_mestra)?;
    let segredo = SegredoArmazenado {
        id: gerar_id_do_segredo()?,
        titulo: titulo.to_string(),
        usuario: usuario.trim().to_string(),
        senha: senha.to_string(),
        anotacoes: anotacoes.trim().to_string(),
    };
    cofre.segredos.push(segredo);
    salvar_cofre_de_segredos(&app, &cofre, &senha_mestra)?;
    Ok(resumos_dos_segredos(&cofre))
}

/// Recupera uma senha apenas para a ação explícita de copiá-la.
#[tauri::command]
pub fn copiar_senha_do_segredo(
    app: tauri::AppHandle,
    id: String,
    senha_mestra: String,
    estado: State<'_, EstadoDoCofreDeSegredos>,
) -> Result<String, String> {
    let _guarda = estado.trava.lock().map_err(|_| "o cofre de segredos está ocupado".to_string())?;
    let senha_mestra = Zeroizing::new(senha_mestra);
    let cofre = ler_cofre_de_segredos(&app, &senha_mestra)?;
    cofre
        .segredos
        .iter()
        .find(|segredo| segredo.id == id)
        .map(|segredo| segredo.senha.clone())
        .ok_or_else(|| "segredo não encontrado".into())
}

/// Apaga uma credencial e grava novamente o conteúdo cifrado.
#[tauri::command]
pub fn apagar_segredo(
    app: tauri::AppHandle,
    id: String,
    senha_mestra: String,
    estado: State<'_, EstadoDoCofreDeSegredos>,
) -> Result<Vec<ResumoDeSegredo>, String> {
    let _guarda = estado.trava.lock().map_err(|_| "o cofre de segredos está ocupado".to_string())?;
    let senha_mestra = Zeroizing::new(senha_mestra);
    let mut cofre = ler_cofre_de_segredos(&app, &senha_mestra)?;
    let tamanho_anterior = cofre.segredos.len();
    cofre.segredos.retain(|segredo| segredo.id != id);
    if cofre.segredos.len() == tamanho_anterior {
        return Err("segredo não encontrado".into());
    }
    salvar_cofre_de_segredos(&app, &cofre, &senha_mestra)?;
    Ok(resumos_dos_segredos(&cofre))
}

/// Exige seis caracteres antes de criar o primeiro cofre.
fn validar_senha_mestra(senha_mestra: &str) -> Result<(), String> {
    if senha_mestra.chars().count() != TAMANHO_DA_SENHA_MESTRA {
        return Err(format!("a senha mestra precisa ter exatamente {TAMANHO_DA_SENHA_MESTRA} caracteres"));
    }
    Ok(())
}

/// Cria um identificador aleatório que não revela título, usuário nem senha.
fn gerar_id_do_segredo() -> Result<String, String> {
    let mut bytes = [0; 16];
    getrandom::fill(&mut bytes).map_err(|erro| format!("não foi possível gerar o identificador: {erro}"))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

/// Remove as senhas dos itens antes de qualquer resposta de listagem.
fn resumos_dos_segredos(cofre: &CofreDeSegredos) -> Vec<ResumoDeSegredo> {
    cofre.segredos.iter().map(ResumoDeSegredo::from).collect()
}
