use super::Database;
// Standard Modules
use std::path::Path;
use std::fs;
// Crate
use crate::db::{FileEntry};



impl Database {
    pub fn delete_file(&mut self, name: &str) -> bool {
        // Find the file entry in meta
        let entry_index = match self.meta.files.iter().position(|f| f.name == name) {
            Some(idx) => idx,
            None => {
                eprintln!("⚠️ File not found in database: {}", name);
                return false;
            }
        };

        // Remove the entry from main meta and get ownership
        let mut entry = self.meta.files.remove(entry_index);

        // Get the actual file path
        let path = Path::new(&entry.file_path);

        let dest_path = format!("{}/{}", self.recycle_dir, entry.file_name);

        // Move the file
        if path.exists() {
            if let Err(err) = fs::rename(path, &dest_path) {
                eprintln!("⚠️ Failed to move file to recycle bin: {}", err);
                return false;
            }
        }

        // Update entry file_path and status, push to trash_meta
        entry.file_path = dest_path;
        entry.is_recycled = true;
        self.trash_meta.files.push(entry);

        // Save both metas
        self.save_trash_meta();
        self.save_meta();
        println!("♻️ File '{}' moved to recycle bin", name);
        true
    }

    pub fn delete_password(&mut self, name: &str) -> bool {
         let entry_index = match self.meta.passwords.iter().position(|p| p.name == name) {
            Some(idx) => idx,
            None => {
                eprintln!("⚠️ Password not found in database: {}", name);
                return false;
            }
        };

        let entry = self.meta.passwords.remove(entry_index);
        self.trash_meta.passwords.push(entry);
        // Save both metas
        self.save_trash_meta();
        self.save_meta();
        println!("🗑️ Password '{}' moved to temp password file", name);
        true
    }

    pub fn delete_all_passwords(&mut self) {
        let mut passwords = std::mem::take(&mut self.meta.passwords);
        for entry in &mut passwords {
            entry.is_recycled = true;
        }
        self.trash_meta.passwords.extend(passwords);
        self.save_meta();
        self.save_trash_meta();
        println!("🗑️ All passwords moved to trash");
    }
    
    pub fn delete_all_files(&mut self) {
        let mut still_meta: Vec<FileEntry> = vec![];
    
        for entry in self.meta.files.drain(..) {
            let file_path = Path::new(&entry.file_path);
    
            if !file_path.exists() {
                eprintln!("⚠️ File does not exist on disk: {}", entry.file_path);
                // keep in meta since file is missing
                still_meta.push(entry);
                continue;
            }
    
            // Ensure unique filename in recycle bin
            let file_name = match file_path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => {
                    eprintln!("⚠️ Invalid file path: {}", entry.file_path);
                    still_meta.push(entry);
                    continue;
                }
            };
    
            let mut dest_path = Path::new(&self.recycle_dir).join(&file_name);
            // If file exists in recycle bin, append timestamp
            if dest_path.exists() {
                let timestamp = chrono::Local::now().timestamp();
                let new_name = format!("{}_{}", timestamp, file_name);
                dest_path = Path::new(&self.recycle_dir).join(new_name);
            }
    
            if let Err(err) = fs::rename(&file_path, &dest_path) {
                eprintln!("⚠️ Failed to move file to recycle bin: {}", err);
                still_meta.push(entry); // keep metadata intact
                continue;
            }
    
            // Update entry and push to trash_meta
            let mut new_entry = entry;
            new_entry.file_path = dest_path.to_string_lossy().to_string();
            new_entry.is_recycled = true;
            self.trash_meta.files.push(new_entry);
        }
    
        // Restore any files that couldn’t be moved back to meta
        self.meta.files.extend(still_meta);
    
        // Save meta files
        self.save_meta();
        self.save_trash_meta();
    
        println!("🗑️ All files moved to recycle bin");
    }


    pub fn empty_recycle_bin(&mut self) -> (usize, usize) {
        let password_count = self.trash_meta.passwords.len();
        let file_count = self.trash_meta.files.len();

        // Delete files on disk
        for file_entry in &self.trash_meta.files {
            let path = std::path::Path::new(&file_entry.file_path);
            if path.exists() {
                if let Err(e) = std::fs::remove_file(path) {
                    eprintln!("⚠️ Failed to delete file {}: {}", file_entry.file_path, e);
                }
            }
        }

        // Clear metadata
        self.trash_meta.passwords.clear();
        self.trash_meta.files.clear();

        // Save updated trash_meta
        self.save_trash_meta();

        (password_count, file_count) // return deleted counts
    }


    pub fn restore_password(&mut self, name: &str) -> bool {
        if let Some(pos) = self.trash_meta.passwords.iter().position(|p| p.name == name) {
            let mut entry = self.trash_meta.passwords.remove(pos);
            entry.is_recycled = false;
            self.meta.passwords.push(entry); // move back to active passwords
            self.save_trash_meta();
            self.save_meta();
            true
        } else {
            false // password not found
        }
    }

    pub fn restore_file(&mut self, name: &str) -> bool {
        if let Some(pos) = self.trash_meta.files.iter().position(|f| f.name == name) {
            let mut entry = self.trash_meta.files.remove(pos); // always remove from trash_meta
            let subfolder = match entry.extension.as_str() {
                "jpg" | "jpeg" | "png" | "gif" => "photos",
                "mp4" | "mkv" | "avi"          => "videos",
                "pdf" | "docx" | "txt"         => "documents",
                _ => &entry.extension,
            };

           
            let src_path = Path::new(&entry.file_path);
            let dst_str = format!("{}/{}/{}", self.decrypted_files_dir, subfolder, entry.file_name);
            let dst_path = Path::new(&dst_str);
           
            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent).expect("⚠️ Failed to create subfolder");
            }

            entry.is_recycled = false;
            let restored = match fs::rename(src_path, dst_path) {
                Ok(_) => {
                    self.meta.files.push(entry);
                    println!("✅ Restored file: {}", name);
                    true
                }
                Err(e) => {
                    eprintln!("⚠️ Failed to restore file '{}': {} (file may not exist)", entry.file_name, e);
                    false
                }
            };
           
            // Always update trash_meta even if restore fails
            self.save_trash_meta();
            self.save_meta();
           
            restored
        } else {
            eprintln!("⚠️ File '{}' not found in recycle bin.", name);
            false
        }
    }

}
