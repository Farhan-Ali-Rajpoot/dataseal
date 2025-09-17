use super::Database;
// Standard Modules
use std::path::Path;
use std::{fs, fs::{ remove_file }};
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

        let entry = &self.meta.files[entry_index]; // borrow first
        let path = Path::new(&entry.file_path);
        let dest_path = format!("{}/{}", self.recycle_dir, entry.file_name);

        if !path.exists() {
            eprintln!("⚠️ File does not exist on disk: {}", entry.file_path);
            return false;
        }

        if let Some(p) = path.to_str() {
            if !self.copy_file(p, &dest_path) {
                eprintln!("⚠️ Failed to move file to recycle bin");
                return false;
            }
        } else {
            eprintln!("⚠️ Invalid UTF-8 path: {}", entry.file_path);
            return false;
        }

        if let Err(e) = remove_file(path) {
            eprintln!("⚠️ Failed to delete original file: {}: {}", entry.file_path, e);
            return false;
        }

        // Remove from meta after successful copy
        let mut entry = self.meta.files.remove(entry_index);
        entry.file_path = dest_path;
        entry.is_recycled = true;
        self.trash_meta.files.push(entry);

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

    pub fn delete_all_passwords(&mut self) -> bool {
        let mut passwords = std::mem::take(&mut self.meta.passwords);
        for entry in &mut passwords {
            entry.is_recycled = true;
        }
        self.trash_meta.passwords.extend(passwords);
        self.save_meta();
        self.save_trash_meta();
        println!("🗑️ All passwords moved to trash");
        true
    }
    
    pub fn delete_all_files(&mut self) -> bool {
        let drained: Vec<FileEntry> = self.meta.files.drain(..).collect();
        let mut still_meta: Vec<FileEntry> = vec![];
        
        for entry in drained {
            let file_path = Path::new(&entry.file_path);
        
            if !file_path.exists() {
                eprintln!("⚠️ File does not exist on disk: {}", entry.file_path);
                still_meta.push(entry);
                continue; // don't return early, just skip
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
            if dest_path.exists() {
                let timestamp = chrono::Local::now().timestamp();
                let new_name = format!("{}_{}", timestamp, file_name);
                dest_path = Path::new(&self.recycle_dir).join(new_name);
            }
        
            if let (Some(src), Some(dst)) = (file_path.to_str(), dest_path.to_str()) {
                if !self.copy_file(src, dst) {
                    eprintln!("⚠️ Failed to move file to recycle bin");
                    still_meta.push(entry);
                    continue;
                }
            } else {
                eprintln!("⚠️ Path contains invalid UTF-8");
                still_meta.push(entry);
                continue;
            }
        
            if let Err(e) = remove_file(&file_path) {
                eprintln!("⚠️ Failed to delete original file: {}: {}", entry.file_path, e);
            }
        
            let mut new_entry = entry;
            new_entry.file_path = dest_path.to_string_lossy().to_string();
            new_entry.is_recycled = true;
            self.trash_meta.files.push(new_entry);
        }
    
        // Restore skipped entries
        self.meta.files.extend(still_meta);
    
        self.save_meta();
        self.save_trash_meta();
    
        println!("🗑️ All files moved to recycle bin");
        true
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
            entry.name = self.get_unique_password_name(&entry.name);
            self.meta.passwords.push(entry); // move back to active passwords
            self.save_trash_meta();
            self.save_meta();
            true
        } else {
            false // password not found
        }
    }

    pub fn restore_file(&mut self, name: &str) -> bool {
        // Find the file in trash
        let pos = match self.trash_meta.files.iter().position(|f| f.name == name) {
            Some(p) => p,
            None => {
                eprintln!("⚠️ File '{}' not found in recycle bin.", name);
                return false;
            }
        };

        let mut entry = self.trash_meta.files.remove(pos); // remove from trash_meta

        // Determine subfolder based on extension
        let subfolder = self.get_sub_folder(&entry.extension.as_str());

        let src_path = Path::new(&entry.file_path);

        // Only generate a unique file name if conflict exists
        entry.file_name = if self.meta.files.iter().any(|f| f.file_name == entry.file_name) {
            self.get_unique_file_name(&entry.file_name)
        } else {
            entry.file_name.clone()
        };

        let dst_str = format!("{}/{}/{}", self.decrypted_files_dir, subfolder, entry.file_name);
        let dst_path = Path::new(&dst_str);

        if let Some(parent) = dst_path.parent() {
            std::fs::create_dir_all(parent).expect("⚠️ Failed to create subfolder");
        }

        // Only generate a unique logical name if conflict exists
        entry.name = if self.meta.files.iter().any(|f| f.name == entry.name)
            || self.encrypted_meta.files.iter().any(|f| f.name == entry.name) {
            self.get_unique_name_for_file(&entry.name)
        } else {
            entry.name.clone()
        };

        // Copy file from recycle bin to decrypted folder
        let restored = if let (Some(src), Some(dst)) = (src_path.to_str(), dst_path.to_str()) {
            self.copy_file(src, dst)
        } else {
            false
        };

        if !restored {
            eprintln!("⚠️ Failed to restore file '{}' (file may not exist)", entry.file_name);
            return false;
        }

        entry.file_path = dst_str;
        entry.is_recycled = false;

        self.meta.files.push(entry);

        // Save both metas
        self.save_trash_meta();
        self.save_meta();

        println!("✅ Restored file: {}", name);
        true
    }
    
    pub fn restore_all_files(&mut self) -> bool {
        if self.trash_meta.files.is_empty() {
            println!("♻️ No files in recycle bin to restore.");
            return false;
        }

        let files_to_restore: Vec<FileEntry> = self.trash_meta.files.drain(..).collect();
        let mut restored_count = 0;

        for mut entry in files_to_restore {
            let subfolder = self.get_sub_folder(&entry.extension);
            let src_path = Path::new(&entry.file_path);

            // Generate unique file_name if conflict exists
            entry.file_name = if self.meta.files.iter().any(|f| f.file_name == entry.file_name) {
                self.get_unique_file_name(&entry.file_name)
            } else {
                entry.file_name.clone()
            };

            let dst_str = format!("{}/{}/{}", self.decrypted_files_dir, subfolder, entry.file_name);
            let dst_path = Path::new(&dst_str);

            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent).expect("⚠️ Failed to create subfolder");
            }

            // Generate unique logical name
            entry.name = if self.meta.files.iter().any(|f| f.name == entry.name)
                || self.encrypted_meta.files.iter().any(|f| f.name == entry.name) {
                self.get_unique_name_for_file(&entry.name)
            } else {
                entry.name.clone()
            };

            // Copy file from recycle bin to decrypted folder
            if let (Some(src), Some(dst)) = (src_path.to_str(), dst_path.to_str()) {
                if self.copy_file(src, dst) {
                    entry.file_path = dst_str;
                    entry.is_recycled = false;
                    self.meta.files.push(entry);
                    restored_count += 1;
                } else {
                    eprintln!("⚠️ Failed to restore file: {}", entry.file_name);
                }
            }
        }

        self.save_meta();
        self.save_trash_meta();

        println!("✅ Restored {} file(s) from recycle bin.", restored_count);
        true
    }

    /// Restore all passwords from recycle bin
    pub fn restore_all_passwords(&mut self) -> bool {
        if self.trash_meta.passwords.is_empty() {
            println!("♻️ No passwords in recycle bin to restore.");
            return false;
        }

        let passwords_to_restore = self.trash_meta.passwords.drain(..).collect::<Vec<_>>();
        let mut restored_count = 0;

        for mut entry in passwords_to_restore {
            entry.is_recycled = false;
            entry.name = self.get_unique_password_name(&entry.name);
            self.meta.passwords.push(entry);
            restored_count += 1;
        }

        self.save_meta();
        self.save_trash_meta();

        println!("✅ Restored {} password(s) from recycle bin.", restored_count);
        true
    }

    // Helper function
    pub fn get_unique_password_name(&self, name: &str) -> String {
        let mut counter = 0;
    
        loop {
            let current_name = if counter == 0 {
                name.to_string()
            } else {
                format!("{}{}", name, counter)
            };
        
            let exists_in_decrypted = self.meta.passwords.iter().any(|entry| entry.name == current_name);
            let exists_in_encrypted = self.encrypted_meta.passwords.iter().any(|entry| entry.name == current_name);
        
            if !exists_in_decrypted && !exists_in_encrypted {
                return current_name;
            }
        
            counter += 1;
        }
    }

    pub fn get_unique_name_for_file(&self, name: &str) -> String {
        let mut counter = 0;

        loop {
            let current_name = if counter == 0 {
                name.to_string()
            } else {
                format!("{}{}", name, counter)
            };

            let exists_in_files = self.meta.files.iter().any(|entry| entry.name == current_name);
            let exists_in_encrypted_files = self.encrypted_meta.files.iter().any(|entry| entry.name == current_name);

            if !exists_in_files && !exists_in_encrypted_files {
                return current_name;
            }

            counter += 1;
        }
    }

    
}
