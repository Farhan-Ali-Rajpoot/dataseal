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
pub mod stats;
// Struct modules
use crate::db::enc_keys::{derive_key};
pub use structs::{FileEntry, PasswordEntry, DatabaseMeta, Database};
pub use config::Config;
// Standard Modules
use std::fs::{create_dir_all, read_to_string};
use std::path::{Path, PathBuf};
use std::env;
use dirs;


impl Database {
    pub fn new(master_password: &str) -> Option<Self> {
        // Detect where the binary is running from
        let exe_path = env::current_exe().ok()?;
        let exe_dir = exe_path.parent()?.to_path_buf();
        
        // Decide where to place the data folder
        let root_directory: PathBuf = if is_system_path(&exe_dir) {
            // If in a system path, use home directory (~/.dataseal)
            dirs::home_dir()?.join(".dataseal")
        } else {
            // Otherwise, create data folder next to binary
            exe_dir.join("data")
        };
        
        // Create the directory if it doesn't exist
        if !root_directory.exists() {
            create_dir_all(&root_directory).ok()?;
        }
        
        Database::with_dir(root_directory.to_str()?, master_password)
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
                eprintln!("Failed to load config: {}", e);
                return None;
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
                if let Err(e) = create_dir_all(dir) {
                    eprintln!("⚠️ Failed to create directory {}: {}", dir, e);
                    return None;
                }
            }
        }

        // Create extension subfolders inside decrypted/files
        let extensions = ["photos", "videos", "documents", "other"];
        for ext in &extensions {
            let ext_dir = format!("{}/{}", decrypted_files_dir, ext);
            if !Path::new(&ext_dir).exists() {
                if let Err(e) = create_dir_all(&ext_dir) {
                    eprintln!("⚠️ Failed to create folder {}: {}", ext_dir, e);
                    return None;
                }
            }
        }

        // Helper to create a file with empty JSON if not exists
        fn create_empty_json_file(path: &str) -> Result<(), std::io::Error> {
            if !Path::new(path).exists() {
                std::fs::write(path, "{}")?;
            }
            Ok(())
        }

        // Create empty JSON files with error handling
        if let Err(e) = create_empty_json_file(&meta_file) {
            eprintln!("⚠️ Failed to create {}: {}", meta_file, e);
            return None;
        }
        if let Err(e) = create_empty_json_file(&trash_meta_file) {
            eprintln!("⚠️ Failed to create {}: {}", trash_meta_file, e);
            return None;
        }
        if let Err(e) = create_empty_json_file(&encrypted_meta_file) {
            eprintln!("⚠️ Failed to create {}: {}", encrypted_meta_file, e);
            return None;
        }

        // Load meta data with better error handling
        let load_meta = |path: &str| -> DatabaseMeta {
            if Path::new(path).exists() {
                match read_to_string(path) {
                    Ok(data) if !data.trim().is_empty() => {
                        serde_json::from_str(&data).unwrap_or_default()
                    }
                    _ => DatabaseMeta::default(),
                }
            } else {
                DatabaseMeta::default()
            }
        };

        let meta = load_meta(&meta_file);
        let trash_meta = load_meta(&trash_meta_file);
        let encrypted_meta = load_meta(&encrypted_meta_file);

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

fn is_system_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy().to_lowercase();

    // -----------------
    // Linux / Unix
    // -----------------
    path_str.starts_with("/bin") ||
    path_str.starts_with("/sbin") ||
    path_str.starts_with("/lib") ||
    path_str.starts_with("/lib64") ||
    path_str.starts_with("/usr") ||     // /usr, /usr/bin, /usr/local, /usr/share...
    path_str.starts_with("/etc") ||
    path_str.starts_with("/var") ||
    path_str.starts_with("/opt") ||
    path_str.starts_with("/snap") ||

    // -----------------
    // Windows
    // -----------------
    path_str.contains("\\windows\\system32") ||
    path_str.contains("\\windows") ||
    path_str.contains("\\program files") ||
    path_str.contains("\\program files (x86)") ||
    path_str.contains("\\programdata") ||
    path_str.contains("\\appdata\\local\\programs") || // e.g. VSCode install dir
    path_str.contains("\\users\\default") ||           // default profile
    path_str.contains("\\users\\public") ||

    // -----------------
    // macOS
    // -----------------
    path_str.starts_with("/applications") ||
    path_str.starts_with("/system") ||
    path_str.starts_with("/library") ||
    path_str.starts_with("/usr") ||        // /usr/bin, /usr/local
    path_str.starts_with("/bin") ||
    path_str.starts_with("/sbin") ||
    path_str.starts_with("/opt") ||
    path_str.starts_with("/var")
}
