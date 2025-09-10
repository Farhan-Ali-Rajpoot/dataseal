use super::{Database};

// Standard Module
use std::io::{BufWriter, Write};
use std::fs::{OpenOptions};





impl Database {
    pub fn save_meta(&self) {
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
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.encrypted_meta_file)
            .unwrap();
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", serde_json::to_string_pretty(&self.encrypted_meta).unwrap()).unwrap();
    }

}













