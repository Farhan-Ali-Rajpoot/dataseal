use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::Path;

use rand::rngs::ThreadRng;
use rand::RngCore;

use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // AEAD
use aes_gcm_siv::aead::{Aead, KeyInit};

use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use base64::{engine::general_purpose, Engine as _};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db_version: String,
    pub kdf_salt_b64: String,
    pub verifier_b64: String,
    pub max_file_size_mb: u64,
    pub path: Option<String>,
}

impl Config {
    /// Generate a random 128-bit salt, base64 encoded
    pub fn generate_salt() -> String {
        let mut salt = [0u8; 16];
        ThreadRng::default().fill_bytes(&mut salt);
        general_purpose::STANDARD.encode(&salt)
    }

    /// Default configuration
    pub fn default() -> Self {
        Self {
            db_version: "1.0.0".to_string(),
            kdf_salt_b64: Self::generate_salt(),
            verifier_b64: "".to_string(),
            max_file_size_mb: 100,
            path: None,
        }
    }

    /// Generate a random 96-bit nonce
    pub fn generate_nonce() -> [u8; 12] {
        let mut nonce = [0u8; 12];
        ThreadRng::default().fill_bytes(&mut nonce);
        nonce
    }

    /// Derive a 32-byte master key from password + salt using PBKDF2
    pub fn derive_master_key(&self, master_password: &str) -> [u8; 32] {
        let salt_bytes = general_purpose::STANDARD.decode(&self.kdf_salt_b64)
            .expect("Failed to decode salt");
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(master_password.as_bytes(), &salt_bytes, 100_000, &mut key);
        key
    }

    /// Encrypt the verifier string ("verify") with master password
    pub fn encrypt_verifier(&self, master_password: &str) -> (String, [u8; 32]) {
        let key = self.derive_master_key(master_password);
        let cipher = Aes256GcmSiv::new(Key::<Aes256GcmSiv>::from_slice(&key));
        let nonce = Self::generate_nonce();

        let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), b"verify" as &[u8])
            .expect("Encryption failed");

        let mut result = nonce.to_vec();
        result.extend(ciphertext);
        (general_purpose::STANDARD.encode(&result), key)
    }

    /// Check if provided password can decrypt verifier
    pub fn check_verifier(&self, master_password: &str) -> bool {
        let key = self.derive_master_key(master_password);
        let data = match general_purpose::STANDARD.decode(&self.verifier_b64) {
            Ok(d) => d,
            Err(_) => return false,
        };
        if data.len() < 12 { return false; }

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let cipher = Aes256GcmSiv::new(Key::<Aes256GcmSiv>::from_slice(&key));
        match cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext) {
            Ok(pt) => pt == b"verify",
            Err(_) => false,
        }
    }

    // Change password
    pub fn change_master_password(&mut self, old_password: &str, new_password: &str) -> Option<[u8; 32]> {
        // Step 1: Check old password
        if !self.check_verifier(old_password) {
            return None;
        }

        // Step 2: Generate new verifier with new password
        let (new_verifier, master_key) = self.encrypt_verifier(new_password);
        self.verifier_b64 = new_verifier;


        if let Some(path) = &self.path {
            let _ = fs::write(path, serde_json::to_string_pretty(&self).unwrap())
                .map_err(|e| format!("Failed to write updated config: {e}"));
            Some(master_key)
        } else {
            None
        }
    }

    /// Load existing config or create new one; verify master password
    pub fn load_or_create(path: &str, master_password: &str) -> Result<Self, String> {
        if !Path::new(path).exists() {
            // Create new config
            if let Some(parent) = Path::new(path).parent() {
                fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent directory: {e}"))?;
            }
            let mut cfg = Self::default();
            let (verifier_b64, _) = cfg.encrypt_verifier(master_password);
            cfg.verifier_b64 = verifier_b64;
            cfg.path = Some(path.to_string());
            fs::write(path, serde_json::to_string_pretty(&cfg).unwrap())
                .map_err(|e| format!("Failed to write config: {e}"))?;
            println!("Singned up!");
            Ok(cfg)
        } else {
            // Load existing config
            let data = fs::read_to_string(path)
                .map_err(|e| format!("Failed to read config: {e}"))?;
            let cfg: Self = serde_json::from_str(&data)
                .map_err(|e| format!("Failed to parse config: {e}"))?;
            if cfg.check_verifier(master_password) {
                Ok(cfg)
            } else {
                Err("❌ Wrong master password! Exiting...".to_string())
            }
        }
    }
}
