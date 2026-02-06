use crate::services::key_storage::KeyStorage;
use tauri::State;

#[tauri::command]
pub fn save_api_key(
    state: State<'_, KeyStorage>,
    provider_id: String,
    key: String,
) -> Result<(), String> {
    state.set(&provider_id, &key)
}

#[tauri::command]
pub fn get_api_key(
    state: State<'_, KeyStorage>,
    provider_id: String,
) -> Result<Option<String>, String> {
    Ok(state.get(&provider_id))
}

#[tauri::command]
pub fn delete_api_key(
    state: State<'_, KeyStorage>,
    provider_id: String,
) -> Result<(), String> {
    state.delete(&provider_id)
}
