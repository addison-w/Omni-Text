use tauri::{AppHandle, Emitter};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

#[tauri::command]
pub fn register_hotkey(app: AppHandle, id: String, shortcut: String) -> Result<(), String> {
    let shortcut_parsed: Shortcut = shortcut
        .parse()
        .map_err(|e| format!("Invalid shortcut '{}': {:?}", shortcut, e))?;

    let action_id = id.clone();
    let app_handle = app.clone();

    app.global_shortcut()
        .on_shortcut(shortcut_parsed, move |_app, _shortcut, event| {
            if event.state == ShortcutState::Pressed {
                let _ = app_handle.emit("hotkey-triggered", action_id.clone());
            }
        })
        .map_err(|e| format!("Failed to register shortcut: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn unregister_hotkey(app: AppHandle, shortcut: String) -> Result<(), String> {
    let shortcut_parsed: Shortcut = shortcut
        .parse()
        .map_err(|e| format!("Invalid shortcut '{}': {:?}", shortcut, e))?;

    app.global_shortcut()
        .unregister(shortcut_parsed)
        .map_err(|e| format!("Failed to unregister shortcut: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn unregister_all_hotkeys(app: AppHandle) -> Result<(), String> {
    app.global_shortcut()
        .unregister_all()
        .map_err(|e| format!("Failed to unregister all shortcuts: {}", e))?;

    Ok(())
}
