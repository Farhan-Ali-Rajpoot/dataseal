use super::{Database, FileEntry};
// Standard Modules
use std::{fs, fs::{File,metadata,remove_file,create_dir_all}};
use std::path::{Path};
use std::io::{Read, Write, BufWriter};
use std::io::stdout;

use crate::db::time;
use crate::db::enc_keys::{unwrap_item_key, wrap_item_key, generate_item_key};


impl Database {
    pub fn encrypt_all_files(&mut self) -> bool {
        let mut success_count = 0;
        let mut failure_count = 0;

        // Collect files that need encryption
        let files_to_encrypt: Vec<FileEntry> = self.meta.files
            .iter()
            .filter(|f| !f.is_encrypted)
            .cloned()
            .collect();

        let total_to_process = files_to_encrypt.len();
        
        if total_to_process == 0 {
            println!("ℹ️  No unencrypted files found to encrypt.");
            return true;
        }

        println!("🔒 Encrypting {} files:", total_to_process);

        for (current, entry) in files_to_encrypt.iter().enumerate() {
            print!("  {}/{}: {}... ", current + 1, total_to_process, entry.name);
            
            // Check if file exists
            if !Path::new(&entry.file_path).exists() {
                println!("❌ (file missing)");
                failure_count += 1;
                continue;
            }

            // Encrypted file path
            let encrypted_path = format!("{}/{}.enc", self.encrypted_dir, entry.name);

            // Unwrap item key
            let key = match unwrap_item_key(&entry.encrypted_item_key, &self.master_key) {
                Some(k) => k,
                None => {
                    println!("❌ (key error)");
                    failure_count += 1;
                    continue;
                }
            };

            // Encrypt the file
            let is_encrypted = self.encrypt_file_data(&entry.file_path, &encrypted_path, &key);
            if !is_encrypted {
                if Path::new(&encrypted_path).exists() {
                    let _ = remove_file(&encrypted_path); // rollback
                }
                println!("❌ (encryption failed)");
                failure_count += 1;
                continue;
            }

            // Add entry to encrypted_meta
            self.encrypted_meta.files.push(FileEntry {
                name: entry.name.clone(),
                file_name: entry.file_name.clone(),
                file_path: encrypted_path.clone(),
                size: entry.size.clone(),
                extension: entry.extension.clone(),
                encrypted_item_key: entry.encrypted_item_key.clone(),
                is_encrypted: true,
                is_recycled: false,
                created_at: entry.created_at.clone(),
                updated_at: time::now(),
            });

            // Remove from meta
            self.meta.files.retain(|f| f.name != entry.name);

            // Delete original file
            if let Err(e) = remove_file(&entry.file_path) {
                eprintln!("⚠️ Failed to delete original file: {}", e);
            }

            success_count += 1;
        }

        // Save metadata if we had successful operations
        if success_count > 0 {
            self.save_encrypted_meta();
            self.save_meta();
        }

        // Log results
        println!("\n📊 Encryption Summary:");
        println!("   Total:    {}", total_to_process);
        println!("   Success:  {}", success_count);
        println!("   Failed:   {}", failure_count);
        
        if success_count > 0 && failure_count == 0 {
            println!("✅ Successfully encrypted all {} files.", success_count);
        } else if success_count > 0 {
            println!("⚠️  Encrypted {}/{} files. {} failed.", success_count, total_to_process, failure_count);
        } else {
            println!("❌ Failed to encrypt any files. {}/{} failed.", failure_count, total_to_process);
        }

        failure_count == 0
    }

    pub fn decrypt_all_files(&mut self) -> bool {
        let mut success_count = 0;
        let mut failure_count = 0;

        // Collect files that need decryption
        let files_to_decrypt: Vec<FileEntry> = self.encrypted_meta.files
            .iter()
            .filter(|f| f.is_encrypted)
            .cloned()
            .collect();

        let total_to_process = files_to_decrypt.len();
        
        if total_to_process == 0 {
            println!("ℹ️  No encrypted files found to decrypt.");
            return true;
        }

        println!("🔓 Decrypting {} files:", total_to_process);

        for (current, entry) in files_to_decrypt.iter().enumerate() {
            print!("  {}/{}: {}... ", current + 1, total_to_process, entry.name);
            
            // Check if encrypted file exists
            if !Path::new(&entry.file_path).exists() {
                println!("❌ (encrypted file missing)");
                failure_count += 1;
                continue;
            }

            let subfolder = self.get_sub_folder(&entry.extension);
            let decrypted_path = format!("{}/{}/{}", self.decrypted_files_dir, subfolder, entry.file_name);

            // Ensure target directory exists
            let target_dir = format!("{}/{}", self.decrypted_files_dir, subfolder);
            if !Path::new(&target_dir).exists() {
                if let Err(e) = create_dir_all(&target_dir) {
                    println!("❌ Failed to create directory: {}",e);
                    failure_count += 1;
                    continue;
                }
            }

            // Decrypt the file
            let key = match unwrap_item_key(&entry.encrypted_item_key, &self.master_key) {
                Some(k) => k,
                None => {
                    println!("❌ (key error)");
                    failure_count += 1;
                    continue;
                }
            };

            let is_decrypted = self.decrypt_file_data(&entry.file_path, &decrypted_path, &key);
            if !is_decrypted {
                if Path::new(&decrypted_path).exists() {
                    let _ = remove_file(&decrypted_path); // rollback
                }
                println!("❌ (decryption failed)");
                failure_count += 1;
                continue;
            }

            // Add entry back to meta
            self.meta.files.push(FileEntry {
                name: entry.name.clone(),
                file_name: entry.file_name.clone(),
                encrypted_item_key: entry.encrypted_item_key.clone(),
                file_path: decrypted_path.clone(),
                size: entry.size.clone(),
                extension: entry.extension.clone(),
                is_encrypted: false,
                is_recycled: false,
                created_at: entry.created_at.clone(),
                updated_at: time::now(),
            });

            // Remove from encrypted_meta
            self.encrypted_meta.files.retain(|f| f.name != entry.name);

            // Delete encrypted file
            if let Err(e) = remove_file(&entry.file_path) {
                eprintln!("⚠️ Failed to delete encrypted file: {}", e);
            }

            success_count += 1;
        }

        // Save metadata if we had successful operations
        if success_count > 0 {
            self.save_encrypted_meta();
            self.save_meta();
        }

        // Log results
        println!("\n📊 Decryption Summary:");
        println!("   Total:    {}", total_to_process);
        println!("   Success:  {}", success_count);
        println!("   Failed:   {}", failure_count);
        
        if success_count > 0 && failure_count == 0 {
            println!("✅ Successfully decrypted all {} files.", success_count);
        } else if success_count > 0 {
            println!("⚠️  Decrypted {}/{} files. {} failed.", success_count, total_to_process, failure_count);
        } else {
            println!("❌ Failed to decrypt any files. {}/{} failed.", failure_count, total_to_process);
        }

        failure_count == 0
    }
    // Add file
    pub fn cut_add_file(&mut self, name: &str, file_path: &str) -> bool {
        let path = Path::new(file_path);

        // ✅ Ensure source path exists
        if !path.exists() {
            println!("❌ Source path does not exist: {}", file_path);
            return false;
        }

        // ✅ Reject folders
        if !path.is_file() {
            println!("❌ '{}' is a directory, only files can be added", file_path);
            return false;
        }

        // Detect extension
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_lowercase();

        // Select subfolder based on extension
        let subfolder = self.get_sub_folder(&extension);

        // Ensure subfolder exists
        let target_dir = format!("{}/{}", self.decrypted_files_dir, subfolder);
        if !Path::new(&target_dir).exists() {
            fs::create_dir(&target_dir).expect("⚠️ Failed to create subfolder");
        }

        // Destination path
        let temp_file_name = path.file_name().unwrap().to_str().unwrap();
        let file_name = self.get_unique_file_name(temp_file_name);
        let dest_path = format!("{}/{}", target_dir, file_name);

        // Check size limit
        let metadata = fs::metadata(path).expect("⚠️ Failed to read file metadata");
        let file_size_bytes = metadata.len();
        let file_size_mb = file_size_bytes as f64 / (1024.0 * 1024.0);
        let max_size_bytes = (self.config.max_file_size_mb * 1024 * 1024) as u64;

        if file_size_bytes > max_size_bytes {
            println!(
                "❌ File '{}' exceeds max size of {} MB",
                file_name, self.config.max_file_size_mb
            );
            return false;
        }

        // Prevent duplicates
        if self.meta.files.iter().any(|f| f.name == name) {
            println!("❌ File with name '{}' already exists in meta!", name);
            return false;
        }
        if self.encrypted_meta.files.iter().any(|f| f.name == name) {
            println!("❌ File with name '{}' already exists in encrypted files!", name);
            return false;
        }

        let item_key = generate_item_key();
        let encrypted_item_key = match wrap_item_key(&item_key, &self.master_key) {
            Some(eik) => eik,
            None => {
                println!("❌ Failed to generate encrypted item key for file: {}", name);
                return false;
            }
        };

        // Try restore if missing
        if let Some(index) = self.meta.files.iter().position(|f| f.name == name) {
            let file_missing = !Path::new(&self.meta.files[index].file_path).exists();
                
            if file_missing {
        
                // Now safely borrow mutably after copy is done
                let entry = &mut self.meta.files[index];
                entry.file_path = dest_path.clone();
                entry.file_name = file_name.to_string();
                entry.encrypted_item_key = encrypted_item_key;
                entry.size = file_size_mb.to_string();
                entry.extension = extension.clone();
                entry.is_recycled = false;
                entry.created_at = time::now();
                entry.updated_at = 0.to_string();

                if let Some(src) = path.to_str() {
                    if !self.copy_file(src, dest_path.as_str()) {
                        println!("Failed to copy file!");
                        return false;
                    }else {
                        self.save_meta();
                    } 
                } else {
                    println!("Invalid path (not UTF-8)!");
                    return false;
                }
            
                println!("♻️ Restored missing file for '{}'", name);
                return true;
            }
        }

        self.meta.files.push(FileEntry {
            name: name.to_string(),
            file_name: file_name.to_string(),
            encrypted_item_key,
            file_path: dest_path.clone(),
            size: file_size_mb.to_string(),
            extension: extension.clone(),
            is_recycled: false,
            is_encrypted: false,
            created_at: time::now(),
            updated_at: 0.to_string(),
        });

        if let Some(src) = path.to_str() {
            if !self.copy_file(src, dest_path.as_str()) {
                println!("Failed to copy file!");
                return false;
            }else {
                self.save_meta();
            }
            if let Err(e) = remove_file(src) {
                println!("❌ Failed to delete original file: {}",e);
                return false;
            }
        } else {
            println!("❌ Invalid path (not UTF-8)!");
            return false;
        }

        println!(
            "✅ File added: {} (.{}) size <{} MB> (..cutted..)",
            name, extension, file_size_mb
        );
        true
    }

    pub fn add_file(&mut self, name: &str, file_path: &str) -> bool {
        let path = Path::new(file_path);

        // ✅ Ensure source path exists
        if !path.exists() {
            println!("❌ Source path does not exist: {}", file_path);
            return false;
        }

        // ✅ Reject folders
        if !path.is_file() {
            println!("❌ '{}' is a directory, only files can be added", file_path);
            return false;
        }

        // Detect extension
        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("unknown")
            .to_lowercase();

        // Select subfolder based on extension
        let subfolder = self.get_sub_folder(&extension);

        // Ensure subfolder exists
        let target_dir = format!("{}/{}", self.decrypted_files_dir, subfolder);
        if !Path::new(&target_dir).exists() {
            fs::create_dir(&target_dir).expect("⚠️ Failed to create subfolder");
        }

        // Destination path
        let temp_file_name = path.file_name().unwrap().to_str().unwrap();
        let file_name = self.get_unique_file_name(temp_file_name);
        let dest_path = format!("{}/{}", target_dir, file_name);

        // Check size limit
        let metadata = fs::metadata(path).expect("⚠️ Failed to read file metadata");
        let file_size_bytes = metadata.len();
        let file_size_mb = file_size_bytes as f64 / (1024.0 * 1024.0);
        let max_size_bytes = (self.config.max_file_size_mb * 1024 * 1024) as u64;

        if file_size_bytes > max_size_bytes {
            println!(
                "❌ File '{}' exceeds max size of {} MB",
                file_name, self.config.max_file_size_mb
            );
            return false;
        }

        // Prevent duplicates
        if self.meta.files.iter().any(|f| f.name == name) {
            println!("❌ File with name '{}' already exists in meta!", name);
            return false;
        }
        if self.encrypted_meta.files.iter().any(|f| f.name == name) {
            println!("❌ File with name '{}' already exists in encrypted files!", name);
            return false;
        }

        let item_key = generate_item_key();
        let encrypted_item_key = match wrap_item_key(&item_key, &self.master_key) {
            Some(eik) => eik,
            None => {
                println!("❌ Failed to generate encrypted item key for file: {}", name);
                return false;
            }
        };

        // Try restore if missing
        if let Some(index) = self.meta.files.iter().position(|f| f.name == name) {
            let file_missing = !Path::new(&self.meta.files[index].file_path).exists();
                
            if file_missing {
        
                // Now safely borrow mutably after copy is done
                let entry = &mut self.meta.files[index];
                entry.file_path = dest_path.clone();
                entry.file_name = file_name.to_string();
                entry.encrypted_item_key = encrypted_item_key;
                entry.size = file_size_mb.to_string();
                entry.extension = extension.clone();
                entry.is_recycled = false;
                entry.created_at = time::now();
                entry.updated_at = 0.to_string();

                if let Some(src) = path.to_str() {
                    if !self.copy_file(src, dest_path.as_str()) {
                        println!("Failed to copy file!");
                        return false;
                    }else {
                        self.save_meta();
                    } 
                } else {
                    println!("Invalid path (not UTF-8)!");
                    return false;
                }
            
                println!("♻️ Restored missing file for '{}'", name);
                return true;
            }
        }

        self.meta.files.push(FileEntry {
            name: name.to_string(),
            file_name: file_name.to_string(),
            encrypted_item_key,
            file_path: dest_path.clone(),
            size: file_size_mb.to_string(),
            extension: extension.clone(),
            is_recycled: false,
            is_encrypted: false,
            created_at: time::now(),
            updated_at: 0.to_string(),
        });

        if let Some(src) = path.to_str() {
            if !self.copy_file(src, dest_path.as_str()) {
                println!("Failed to copy file!");
                return false;
            }else {
                self.save_meta();
            }
        } else {
            println!("Invalid path (not UTF-8)!");
            return false;
        }

        println!(
            "✅ File added: {} (.{}) size <{} MB> (..copied..)",
            name, extension, file_size_mb
        );
        true
    }

    pub fn paste_file(&mut self, name: &str, dst_path: &str) -> bool {
            if let Some(file) = self.meta.files.iter().find(|f| f.name == name).cloned() {

            if !Path::new(&file.file_path).exists() {
                println!("❌ File don't exists or it is corrupted");
                return false;
            }

            let target_path = format!("{}/{}",dst_path,file.file_name);
        
            if !self.copy_file(&file.file_path, &target_path) {
                println!("❌ Failed to copy file");
                return false;
            }

            println!("✅ File pasted Successfully! (..copied..)");
            true 
        } else {
            println!("❌ No files found in Databse ( If it is encrypted, decrypt it first )");
            return false;
        } 
    }

    pub fn cut_paste_file(&mut self, name: &str, dst_path: &str) -> bool {
            if let Some(file) = self.meta.files.iter().find(|f| f.name == name).cloned() {

            if !Path::new(&file.file_path).exists() {
                println!("❌ File don't exists or it is corrupted");
                return false;
            }

            let target_path = format!("{}/{}",dst_path,file.file_name);
        
            if !self.copy_file(&file.file_path, &target_path) {
                println!("❌ Failed to copy file");
                return false;
            }

            self.meta.files.retain(|f| f.name != file.name);

            if let Err(e) = remove_file(&file.file_path) {
                println!("❌ Failed to remove original file {}",e);
                return false;
            }

            println!("✅ File pasted Successfully! (..cutted..)");
            self.save_meta();

            true 
        } else {
            println!("❌ No files found in Databse ( If it is encrypted, decrypt it first )");
            return false;
        } 
    }

    pub fn encrypt_file(&mut self, file_name: &str) -> bool {
        if let Some(entry) = self.meta.files.iter().find(|f| f.name == file_name).cloned() {
            if !Path::new(&entry.file_path).exists() {
                println!("❌ File does not exist: {}", entry.file_path);
                return false;
            }

            // Encrypted file path
            let encrypted_path = format!("{}/{}.enc", self.encrypted_dir, file_name);

            // Unwrap item key
            let key = match unwrap_item_key(&entry.encrypted_item_key, &self.master_key) {
                Some(k) => k,
                None => {
                    println!("❌ Wrong password or corrupted file: {}", file_name);
                    return false;
                }
            };
            // Encrypt the file
            let is_encrypted = self.encrypt_file_data(&entry.file_path, &encrypted_path, &key);
            if !is_encrypted {
                if Path::new(&encrypted_path).exists() {
                    let _ = remove_file(&encrypted_path); // rollback
                }
                return false;
            }

            // Add entry to encrypted_meta
            self.encrypted_meta.files.push(FileEntry {
                name: entry.name.clone(),
                file_name: entry.file_name.clone(),
                file_path: encrypted_path.clone(),
                size: entry.size.clone(),
                extension: entry.extension.clone(),
                encrypted_item_key: entry.encrypted_item_key.clone(), // keep key
                is_encrypted: true,
                is_recycled: false,
                created_at: entry.created_at.clone(),
                updated_at: time::now(),
            });

            // Remove from meta
            self.meta.files.retain(|f| f.name != entry.name);

            // Delete original file
            if let Err(e) = remove_file(&entry.file_path) {
                eprintln!("⚠️ Failed to delete original file: {}: {}", entry.file_path, e);
            }

            self.save_encrypted_meta();
            self.save_meta();

            println!("🔒 File encrypted to: {}", encrypted_path);
            true
        } else {
            println!("❌ No file found with name: {}", file_name);
            false
        }
    }

    pub fn decrypt_file(&mut self, file_name: &str) -> bool {
        if let Some(entry) = self.encrypted_meta.files.iter().find(|f| f.name == file_name).cloned() {
            // Check file exists
            if !Path::new(&entry.file_path).exists() {
                println!("❌ Encrypted file does not exist: {}", entry.file_path);
                return false;
            }

            let subfolder = self.get_sub_folder(entry.extension.as_str());

            // Decrypted file path
            let decrypted_path = format!("{}/{}/{}", self.decrypted_files_dir, subfolder, entry.file_name);

            // Decrypt the file
            let key = match unwrap_item_key(&entry.encrypted_item_key, &self.master_key) {
                Some(k) => k,
                None => {
                    println!("❌ Wrong password or corrupted file: {}", file_name);
                    return false;
                }   
            };
            let is_decrypted = self.decrypt_file_data(&entry.file_path, &decrypted_path, &key);

            if !is_decrypted { return false; }

            // Add entry back to meta
            self.meta.files.push(FileEntry {
                name: entry.name.clone(),
                file_name: entry.file_name.clone(),
                encrypted_item_key: entry.encrypted_item_key.clone(), // keep key
                file_path: decrypted_path.clone(),
                size: entry.size.clone(),
                extension: entry.extension.clone(),
                is_encrypted: false,
                is_recycled: false,
                created_at: entry.created_at.clone(),
                updated_at: time::now(),
            });

            // Remove from encrypted_meta
            self.encrypted_meta.files.retain(|f| f.name != entry.name);

            // Delete encrypted file
            if let Err(e) = remove_file(&entry.file_path) {
                eprintln!("⚠️ Failed to delete encrypted file: {}: {}", entry.file_path, e);
            }

            self.save_encrypted_meta();
            self.save_meta();

            println!("🔓 File decrypted to: {}", decrypted_path);
            true
        } else {
            println!("❌ No encrypted file found with name: {}", file_name);
            false
        }
    }

    // Helper function
    pub fn get_unique_file_name(&self, file_name: &str) -> String {
        let path = std::path::Path::new(file_name);
        let stem = path.file_stem().unwrap().to_string_lossy();
        let ext = path.extension().map(|e| e.to_string_lossy()).unwrap_or_default();

        let mut new_name = file_name.to_string();
        let mut counter = 1;

        loop {
            // Check if the name exists in meta or encrypted_meta
            let exists_in_meta = self.meta.files.iter().any(|f| f.file_name == new_name);
            let exists_in_encrypted = self.encrypted_meta.files.iter().any(|f| f.file_name == new_name);

            if !exists_in_meta && !exists_in_encrypted {
                break;
            }

            // Generate new name with counter
            new_name = if ext.is_empty() {
                format!("{}{}", stem, counter)
            } else {
                format!("{}{}.{}", stem, counter, ext)
            };

            counter += 1;
        }

        new_name
    }
    
    pub fn copy_file(&self, src_path: &str, dst_path: &str) -> bool {
        let metadata = metadata(src_path).expect("Failed to read file metadata");
        let file_size_bytes = metadata.len();

        let mut src_file = File::open(src_path).expect("Failed to open source file!");
        let dst_file = File::create(dst_path).expect("Failed to create destination file!");
        let mut dst_writer = BufWriter::new(dst_file);

        let mut buffer = [0u8; 8192];
        let mut copied_bytes: u64 = 0;

        let file_name = Path::new(src_path)
            .file_name()
            .unwrap()
            .to_string_lossy();

        loop {
            let n = src_file.read(&mut buffer).expect("File read error");
            if n == 0 { break; }

            dst_writer.write_all(&buffer[..n]).expect("File write error");
            copied_bytes += n as u64;

            let progress = copied_bytes as f64 / file_size_bytes as f64 * 100.0;
            print!("\r Copying '{}' : {:.2}%", file_name, progress);
            stdout().flush().unwrap();
        }

        // ✅ Ensure everything is written to disk
        dst_writer.flush().expect("Failed to flush data");
        println!();
        true
    }

    pub fn get_sub_folder(&self,extension: &str) -> &str {
        let folder = match extension {
                    "jpg" | "jpeg" | "png" | "gif" => "photos",
                    "mp4" | "mkv" | "avi"          => "videos",
                    "pdf" | "docx" | "txt"         => "documents",
                    _ => "other",
                };

        folder
    }
}