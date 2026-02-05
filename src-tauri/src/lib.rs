mod commands;
mod models;
mod services;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tauri_plugin_positioner::{Position, WindowExt as PosWindowExt};
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use commands::{
    history::*,
    hotkeys::*,
    keychain::*,
    llm_provider::*,
    text_interaction::*,
};

#[tauri::command]
fn quit_app(app: AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(
                    "sqlite:omni_text_history.db",
                    vec![tauri_plugin_sql::Migration {
                        version: 1,
                        description: "create history table",
                        sql: "CREATE TABLE IF NOT EXISTS history (
                            id TEXT PRIMARY KEY,
                            timestamp TEXT NOT NULL,
                            action_name TEXT NOT NULL,
                            app_name TEXT NOT NULL DEFAULT '',
                            original_text TEXT NOT NULL,
                            result_text TEXT NOT NULL,
                            provider TEXT NOT NULL,
                            model TEXT NOT NULL,
                            duration_ms INTEGER NOT NULL DEFAULT 0,
                            tokens_used INTEGER
                        )",
                        kind: tauri_plugin_sql::MigrationKind::Up,
                    }],
                )
                .build(),
        )
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            get_selected_text,
            replace_selected_text,
            check_accessibility_permission,
            request_accessibility_permission,
            call_llm,
            test_connection,
            save_api_key,
            get_api_key,
            delete_api_key,
            register_hotkey,
            unregister_hotkey,
            unregister_all_hotkeys,
            init_history_db,
            add_history_entry,
            search_history,
            delete_history_entry,
            clear_history,
            quit_app,
        ])
        .setup(|app| {
            // Hide from Dock — menu bar only app
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Apply vibrancy to main window
            if let Some(window) = app.get_webview_window("main") {
                #[cfg(target_os = "macos")]
                let _ = apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None);
            }

            // Build tray icon
            let quit_item =
                MenuItem::with_id(app, "quit", "Quit Omni Text", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(
                    app.default_window_icon()
                        .cloned()
                        .expect("No default window icon set"),
                )
                .tooltip("Omni Text")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.move_window(Position::TrayCenter);
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
