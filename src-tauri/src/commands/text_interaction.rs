use crate::services::{accessibility, clipboard};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub async fn get_selected_text(app: tauri::AppHandle) -> Result<String, String> {
    // Check if focused field is secure
    if accessibility::is_secure_field() {
        return Err("Cannot read from secure/password fields".into());
    }

    // Try accessibility API first
    match accessibility::get_selected_text_ax() {
        Ok(text) if !text.is_empty() => return Ok(text),
        _ => {}
    }

    // Fall back to clipboard simulation
    // Save current clipboard contents
    let saved_clipboard = app
        .clipboard()
        .read_text()
        .ok();

    // Simulate Cmd+C
    clipboard::simulate_copy()
        .map_err(|e| format!("Failed to simulate copy: {}", e))?;

    // Read the new clipboard content
    let selected = app
        .clipboard()
        .read_text()
        .map_err(|e| format!("Failed to read clipboard: {}", e))?;

    // Restore original clipboard
    if let Some(saved) = saved_clipboard {
        let _ = app.clipboard().write_text(saved);
    }

    if !selected.is_empty() {
        Ok(selected)
    } else {
        Err("No text selected".into())
    }
}

#[tauri::command]
pub async fn replace_selected_text(app: tauri::AppHandle, text: String) -> Result<(), String> {
    // Write replacement text to clipboard
    app.clipboard()
        .write_text(&text)
        .map_err(|e| format!("Failed to write to clipboard: {}", e))?;

    // Simulate Cmd+V to paste
    clipboard::simulate_paste()
        .map_err(|e| format!("Failed to simulate paste: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn check_accessibility_permission() -> bool {
    accessibility::check_permission()
}

#[tauri::command]
pub fn request_accessibility_permission() {
    accessibility::request_permission();
}
