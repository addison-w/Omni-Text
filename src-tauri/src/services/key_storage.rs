use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, AeadCore, Key,
};
use hkdf::Hkdf;
use sha2::Sha256;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

const HKDF_SALT: &[u8] = b"com.addison.omni-text.keys.v1";
const KEYS_FILENAME: &str = "api_keys.enc";
const NONCE_LEN: usize = 12;

pub struct KeyStorage {
    keys: Mutex<HashMap<String, String>>,
    file_path: PathBuf,
    cipher: Aes256Gcm,
}

fn get_machine_uuid() -> Result<String, String> {
    let output = Command::new("ioreg")
        .args(["-rd1", "-c", "IOPlatformExpertDevice"])
        .output()
        .map_err(|e| format!("Failed to run ioreg: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("IOPlatformUUID") {
            if let Some(uuid) = line.split('"').nth(3) {
                return Ok(uuid.to_string());
            }
        }
    }
    Err("IOPlatformUUID not found in ioreg output".to_string())
}

fn derive_key(uuid: &str) -> Key<Aes256Gcm> {
    let hk = Hkdf::<Sha256>::new(Some(HKDF_SALT), uuid.as_bytes());
    let mut key_bytes = [0u8; 32];
    hk.expand(b"encryption-key", &mut key_bytes)
        .expect("HKDF expand failed");
    key_bytes.into()
}

impl KeyStorage {
    pub fn new(app_dir: PathBuf) -> Self {
        let uuid = get_machine_uuid().expect("Failed to get machine UUID");
        let key = derive_key(&uuid);
        let cipher = Aes256Gcm::new(&key);
        let file_path = app_dir.join(KEYS_FILENAME);

        let keys = if file_path.exists() {
            match Self::load_from_file(&file_path, &cipher) {
                Ok(map) => map,
                Err(_) => HashMap::new(),
            }
        } else {
            HashMap::new()
        };

        KeyStorage {
            keys: Mutex::new(keys),
            file_path,
            cipher,
        }
    }

    fn load_from_file(
        path: &PathBuf,
        cipher: &Aes256Gcm,
    ) -> Result<HashMap<String, String>, String> {
        let data = fs::read(path).map_err(|e| format!("Failed to read keys file: {}", e))?;

        if data.len() < NONCE_LEN {
            return Err("Keys file too short".to_string());
        }

        let (nonce_bytes, ciphertext) = data.split_at(NONCE_LEN);
        let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);

        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;

        let json_str = String::from_utf8(plaintext)
            .map_err(|e| format!("Invalid UTF-8 in decrypted data: {}", e))?;

        serde_json::from_str(&json_str).map_err(|e| format!("Invalid JSON in keys file: {}", e))
    }

    fn save_to_file(&self, keys: &HashMap<String, String>) -> Result<(), String> {
        let json = serde_json::to_string(keys)
            .map_err(|e| format!("Failed to serialize keys: {}", e))?;

        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, json.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;

        let mut data = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        data.extend_from_slice(&nonce);
        data.extend_from_slice(&ciphertext);

        // Atomic write: write to .tmp then rename
        let tmp_path = self.file_path.with_extension("tmp");
        fs::write(&tmp_path, &data).map_err(|e| format!("Failed to write keys file: {}", e))?;
        fs::rename(&tmp_path, &self.file_path)
            .map_err(|e| format!("Failed to rename keys file: {}", e))?;

        Ok(())
    }

    pub fn get(&self, provider_id: &str) -> Option<String> {
        let keys = self.keys.lock().unwrap();
        keys.get(provider_id).cloned()
    }

    pub fn set(&self, provider_id: &str, key: &str) -> Result<(), String> {
        let mut keys = self.keys.lock().unwrap();
        keys.insert(provider_id.to_string(), key.to_string());
        self.save_to_file(&keys)
    }

    pub fn delete(&self, provider_id: &str) -> Result<(), String> {
        let mut keys = self.keys.lock().unwrap();
        keys.remove(provider_id);
        self.save_to_file(&keys)
    }
}
