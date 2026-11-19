//! Janela do chat: pequena no topo, independente da janela de trabalho.
use tauri::{Emitter, Manager, PhysicalPosition, PhysicalSize};

fn posicionar(janela: &tauri::WebviewWindow, expandida: bool) -> Result<(), String> {
    let monitor = janela.current_monitor().map_err(|e| e.to_string())?
        .or(janela.primary_monitor().map_err(|e| e.to_string())?)
        .ok_or("Não foi possível localizar o monitor da ilha")?;
    let area = monitor.work_area();
    let escala = monitor.scale_factor();
    let largura = ((if expandida { 460.0 } else { 248.0 }) * escala) as u32;
    let altura = ((if expandida { 580.0 } else { 48.0 }) * escala) as u32;
    let margem = (12.0 * escala) as u32;
    let largura = largura.min(area.size.width.saturating_sub(margem * 2).max(1));
    let altura = altura.min(area.size.height.saturating_sub(margem * 2).max(1));
    janela.set_size(PhysicalSize::new(largura, altura)).map_err(|e| e.to_string())?;
    janela.set_position(PhysicalPosition::new(
        area.position.x + (area.size.width as i32 - largura as i32) / 2,
        area.position.y + margem as i32,
    )).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn dimensionar_ilha(app: tauri::AppHandle, expandida: bool) -> Result<(), String> {
    let janela = app.get_webview_window("ilha").ok_or("Ilha indisponível")?;
    posicionar(&janela, expandida)?;
    janela.show().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn abrir_chat_na_ilha(app: tauri::AppHandle, pedido: Option<String>) -> Result<(), String> {
    dimensionar_ilha(app.clone(), true)?;
    let janela = app.get_webview_window("ilha").ok_or("Ilha indisponível")?;
    janela.emit("ilha-abrir", pedido).map_err(|e| e.to_string())?;
    janela.set_focus().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn abrir_janela_principal(app: tauri::AppHandle) -> Result<(), String> {
    let janela = app.get_webview_window("main").ok_or("Janela indisponível")?;
    janela.show().map_err(|e| e.to_string())?;
    janela.unminimize().map_err(|e| e.to_string())?;
    janela.set_focus().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn encerrar_aplicativo(app: tauri::AppHandle) { app.exit(0); }
