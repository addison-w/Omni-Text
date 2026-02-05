use crate::models::HistoryEntry;
use tauri::AppHandle;

// History operations are handled via the SQL plugin from the frontend.
// These commands serve as stubs for the command handler registration
// and could be expanded with Rust-side logic if needed.

#[tauri::command]
pub async fn init_history_db(_app: AppHandle) -> Result<(), String> {
    // The SQL plugin handles DB creation via migrations registered in lib.rs
    Ok(())
}

#[tauri::command]
pub async fn add_history_entry(_app: AppHandle, _entry: HistoryEntry) -> Result<(), String> {
    // History inserts are done from the frontend via the SQL plugin
    Ok(())
}

#[tauri::command]
pub async fn search_history(
    _app: AppHandle,
    _query: String,
    _limit: i32,
) -> Result<Vec<HistoryEntry>, String> {
    // History queries are handled from the frontend via the SQL plugin
    Ok(vec![])
}

#[tauri::command]
pub async fn delete_history_entry(_app: AppHandle, _id: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn clear_history(_app: AppHandle) -> Result<(), String> {
    Ok(())
}
