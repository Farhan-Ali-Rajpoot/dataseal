// Public Modules
pub mod files;
pub mod passwords;
pub mod recycle;
pub mod search;
pub mod save;
pub mod time;
pub mod structs;
pub mod config;
pub mod data_encryption;
pub mod enc_keys;
pub mod auth;
// Struct modules
use crate::db::enc_keys::{derive_key};
pub use structs::{FileEntry, PasswordEntry, DatabaseMeta, Database};
pub use config::Config;
// Standard Modules
use std::fs::{create_dir_all, read_to_string};
use std::path::Path;




impl Database {
    pub fn new(master_password: &str) -> Option<Self> {
        match Database::with_dir("src/_database", master_password) {
            Some(instance) => Some(instance),
            None => None
        }
    }

    pub fn with_dir(root_directory: &str, master_password: &str) -> Option<Self> {
        let root_directory = root_directory.trim_end_matches('/').to_string();

        // Main folders
        let decrypted_dir = format!("{}/decrypted", root_directory);
        let encrypted_dir = format!("{}/encrypted", root_directory);
        let recycle_dir = format!("{}/recycle_bin", root_directory);
        let decrypted_files_dir = format!("{}/files", decrypted_dir);

        // Meta and config files
        let meta_file = format!("{}/meta.json", decrypted_dir);
        let trash_meta_file = format!("{}/trash_meta.json", recycle_dir);
        let encrypted_meta_file = format!("{}/meta.json", encrypted_dir);
        let config_file = format!("{}/config.json", root_directory);

        // Load config safely
        let config = match Config::load_or_create(&config_file, master_password) {
            Ok(cfg) => cfg,
            Err(e) => {
                println!("{}", e);
                return None
            }
        };
        let master_key = derive_key(&config.kdf_salt_b64, master_password);

        // Create directories
        for dir in [
            &root_directory,
            &decrypted_dir,
            &encrypted_dir,
            &recycle_dir,
            &decrypted_files_dir,
        ] {
            if !Path::new(dir).exists() {
                create_dir_all(dir).expect(&format!("⚠️ Failed to create directory {}", dir));
            }
        }

        // Create extension subfolders inside decrypted/files
        let extensions = ["photos", "videos", "documents", "other"];
        for ext in &extensions {
            let ext_dir = format!("{}/{}", decrypted_files_dir, ext);
            if !Path::new(&ext_dir).exists() {
                create_dir_all(&ext_dir)
                    .expect(&format!("⚠️ Failed to create folder {}", ext_dir));
            }
        }

        // Create meta files if they don't exist
                // Helper to create a file with empty JSON array if it doesn't exist
        fn create_empty_json_file(path: &str) {
            if !Path::new(path).exists() {
                std::fs::write(path, "{}").expect(&format!("⚠️ Failed to create {}", path));
            }
        }

        // Create all three meta files if they don't exist
        create_empty_json_file(&meta_file);
        create_empty_json_file(&trash_meta_file);
        create_empty_json_file(&encrypted_meta_file);


        // Load meta data
        let meta = if Path::new(&meta_file).exists() {
            let data = read_to_string(&meta_file).unwrap_or_default();
            if data.trim().is_empty() {
                DatabaseMeta::default()
            } else {
                serde_json::from_str(&data).unwrap_or_default()
            }
        } else {
            DatabaseMeta::default()
        };
        // Load trash meta data
        let trash_meta = if Path::new(&trash_meta_file).exists() {
            let data = read_to_string(&trash_meta_file).unwrap_or_default();
            if data.trim().is_empty() {
                DatabaseMeta::default()
            } else {
                serde_json::from_str(&data).unwrap_or_default()
            }
        } else {
            DatabaseMeta::default()
        };
        // Load encrypted meta data
        let encrypted_meta = if Path::new(&encrypted_meta_file).exists() {
            let data = read_to_string(&encrypted_meta_file).unwrap_or_default();
            if data.trim().is_empty() {
                DatabaseMeta::default()
            } else {
                serde_json::from_str(&data).unwrap_or_default()
            }
        } else {
            DatabaseMeta::default()
        };

        Some(Database {
                root_directory,
                master_key,
                decrypted_dir,
                encrypted_dir,
                recycle_dir,
                decrypted_files_dir,
                meta_file,
                trash_meta_file,
                encrypted_meta_file,
                config_file,
                config,
                meta,
                trash_meta,
                encrypted_meta,
            })
    }
}