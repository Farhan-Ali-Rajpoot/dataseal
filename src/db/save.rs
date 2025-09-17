use super::{Database, DatabaseMeta};
use std::{fs,fs::{OpenOptions, read_to_string}};
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;


impl Database {
    pub fn save_meta(&self) {
        if let Some(parent) = Path::new(&self.meta_file).parent() {
            fs::create_dir_all(parent).unwrap(); // create dirs if missing
        }

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.meta_file)
            .unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", serde_json::to_string_pretty(&self.meta).unwrap()).unwrap();
    }

    pub fn save_trash_meta(&self) {
        if let Some(parent) = Path::new(&self.trash_meta_file).parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.trash_meta_file)
            .unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", serde_json::to_string_pretty(&self.trash_meta).unwrap()).unwrap();
    }

    pub fn save_encrypted_meta(&self) {
        if let Some(parent) = Path::new(&self.encrypted_meta_file).parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.encrypted_meta_file)
            .unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", serde_json::to_string_pretty(&self.encrypted_meta).unwrap()).unwrap();
    }

    pub fn load_meta(&mut self) {
        // Helper closure to read a JSON file or return default
        let load_json = |path: &str| -> DatabaseMeta {
            if Path::new(path).exists() {
                let data = read_to_string(path).unwrap_or_default();
                println!("{}: {}",path,data);
                if data.trim().is_empty() {
                    DatabaseMeta::default()
                } else {
                    serde_json::from_str(&data).unwrap_or_default()
                }
            } else {
                DatabaseMeta::default()
            }
        };

        self.meta = load_json(&self.meta_file);
        self.trash_meta = load_json(&self.trash_meta_file);
        self.encrypted_meta = load_json(&self.encrypted_meta_file);
    }

}
