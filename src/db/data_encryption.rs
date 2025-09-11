use super::Database;
use std::fs;

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use base64::{engine::general_purpose, Engine as _};

use crate::db::enc_keys::{generate_nonce};

impl Database {
    

    /// Encrypt a string and return Base64
    pub fn encrypt_string(&self, plaintext: &str, key: &[u8]) -> Option<String> {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));

        let nonce_bytes = generate_nonce();
        let ciphertext = match cipher.encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_ref()) {
            Ok(c) => c,
            Err(_) => return None, // encryption failed
        };

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);

        Some(general_purpose::STANDARD.encode(&result))
    }

    /// Decrypt a Base64 string
    pub fn decrypt_string(&self, encrypted_b64: &str, key: &[u8]) -> Option<String> {
        let data = general_purpose::STANDARD.decode(encrypted_b64).ok()?;
        let (nonce_bytes, ciphertext) = data.split_at(12);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        let plaintext = cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext).ok()?;
        String::from_utf8(plaintext).ok()
    }

    /// Encrypt a file
    pub fn encrypt_file_data(&self, input_path: &str, output_path: &str, key: &[u8]) -> bool {
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));

        let plaintext = match fs::read(input_path) {
            Ok(d) => d,
            Err(e) => {
                println!("❌ Failed to read file {}: {}", input_path, e);
                return false;
            }
        };

        let nonce_bytes = generate_nonce();
        let ciphertext = match cipher.encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_ref()) {
            Ok(c) => c,
            Err(e) => {
                println!("❌ Encryption failed: {}", e);
                return false;
            }
        };

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        if let Err(e) = fs::write(output_path, result) {
            println!("❌ Failed to write encrypted file {}: {}", output_path, e);
            return false;
        }

        true
    }

    /// Decrypt a file
    pub fn decrypt_file_data(&self, encrypted_path: &str, output_path: &str, key: &[u8]) -> bool {
        let data = match fs::read(encrypted_path) {
            Ok(d) => d,
            Err(e) => {
                println!("❌ Failed to read file {}: {}", encrypted_path, e);
                return false;
            }
        };

        let (nonce_bytes, ciphertext) = data.split_at(12);
        let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
        match  cipher.decrypt(Nonce::from_slice(nonce_bytes), ciphertext) {
            Ok(plaintext) => {
                if let Err(e) = fs::write(output_path, plaintext) {
                    eprintln!("❌ Failed to write decrypted file {}: {}", output_path, e);
                    return false;
                }
                true
            }
            Err(_) => {
                eprintln!("❌ Wrong password or corrupted file: {}", encrypted_path);
                return false;
            }
        }
    }
}