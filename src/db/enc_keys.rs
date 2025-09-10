use super::{Database};
use rand::{RngCore, rngs::ThreadRng}; 
use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM
use aes_gcm::aead::{Aead, KeyInit};
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

impl Database {
    pub fn generate_item_key(&self) -> Vec<u8> {
        let mut key = [0u8; 32];
        ThreadRng::default().fill_bytes(&mut key);
        key.to_vec()
    } 

    pub fn wrap_item_key(&self, item_key: &[u8]) -> Option<String> {
        let encryptted = match self.encrypt_with_key(&self.master_key, item_key) {
            Some(e) => e,
            None => return None, // encryption failed
        };
        Some(base64::encode(encryptted))
    }

    pub fn unwrap_item_key(&self, encrypted_item_key: &str) -> Option<Vec<u8>> {
        let decoded = base64::decode(encrypted_item_key).ok()?;
        match self.decrypt_with_key(&self.master_key, &decoded) {
            Some(d) => Some(d),
            None => None, // decryption failed
        }
    }

    pub fn encrypt_with_key(&self, key: &[u8], plaintext: &[u8]) -> Option<Vec<u8>> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
        let nonce = Self::generate_nonce();

        let ciphertext = match cipher.encrypt(Nonce::from_slice(&nonce), plaintext) {
            Ok(c) => c,
            Err(_) => return None, // encryption failed
        };

        let mut result = nonce.to_vec();
        result.extend(ciphertext);
        Some(result)
    }

    pub fn decrypt_with_key(&self, key: &[u8], data: &[u8]) -> Option<Vec<u8>> {
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

        cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext).ok()
    }

    // Generate Nonce
    pub fn generate_nonce() -> [u8; 12] {
        let mut nonce = [0u8; 12];
        ThreadRng::default().fill_bytes(&mut nonce);
        nonce
    }

    /// Derive a 32-byte key from password + salt
    pub fn derive_key(&self, password: &str) -> [u8; 32] {
        let salt_bytes = general_purpose::STANDARD
            .decode(&self.config.kdf_salt_b64)
            .expect("Failed to decode salt");
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt_bytes, 100_000, &mut key);
        key
    }

    pub fn derive_key_static(kdf_salt_b64: &str, password: &str) -> [u8; 32] {
        let salt_bytes = general_purpose::STANDARD
            .decode(kdf_salt_b64)
            .expect("Failed to decode salt");
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), &salt_bytes, 100_000, &mut key);
        key
    }
}