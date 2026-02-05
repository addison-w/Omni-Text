use keyring::Entry;

const SERVICE_NAME: &str = "com.addison.omni-text";

fn get_entry(provider_id: &str) -> Result<Entry, String> {
    Entry::new(SERVICE_NAME, provider_id)
        .map_err(|e| format!("Failed to access keychain: {}", e))
}

#[tauri::command]
pub fn save_api_key(provider_id: String, key: String) -> Result<(), String> {
    let entry = get_entry(&provider_id)?;
    entry
        .set_password(&key)
        .map_err(|e| format!("Failed to save API key: {}", e))
}

#[tauri::command]
pub fn get_api_key(provider_id: String) -> Result<Option<String>, String> {
    let entry = get_entry(&provider_id)?;
    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to read API key: {}", e)),
    }
}

#[tauri::command]
pub fn delete_api_key(provider_id: String) -> Result<(), String> {
    let entry = get_entry(&provider_id)?;
    match entry.delete_credential() {
        Ok(_) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
        Err(e) => Err(format!("Failed to delete API key: {}", e)),
    }
}
