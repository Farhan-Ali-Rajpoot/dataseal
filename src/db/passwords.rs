use super::{Database,PasswordEntry};

use crate::db::time;
use crate::db::enc_keys::{unwrap_item_key, wrap_item_key, generate_item_key};




impl Database {
    // Add password
    pub fn add_password(&mut self, name: &str, password: &str) -> bool {
        for password in &self.meta.passwords {
            if name == password.name {
                println!("❌ Password with this name already exists: {}",name);
                return false;
            }
        }
        for password in &self.encrypted_meta.passwords {
            if name == password.name {
                println!("❌ Password with this name already exists in Encrypted: {}",name);
                return false;
            }
        }

        let item_key = generate_item_key();
        let encrypted_item_key = match wrap_item_key(&item_key, &self.master_key) {
            Some(eik) => eik,
            None => {
                println!("❌ Failed to generate encrypted item key for password: {}", name);
                return false;
            }
        };

        self.meta.passwords.push(PasswordEntry {
            name: name.to_string(),
            password: password.to_string(),
            encrypted_item_key,
            is_encrypted: false,
            is_recycled: false,
            created_at: time::now(),
            updated_at: 0.to_string(),
        });
        self.save_meta();
        println!("✅ Password added: {}", name);
        true
    }

    pub fn change_password(&mut self,name: &str, new_password: &str) -> bool {
        if let Some(entry) = self.meta.passwords.iter_mut().find(|e| e.name == name) {
            if entry.password == new_password {
                println!("❌ New password is the same as the old password for: {}", name);
                return false; // No change needed
            }
           // Update the password
            entry.password = new_password.to_string();
            entry.updated_at = time::now(); // bump updated_at
            self.save_meta();      // or self.save_meta() depending on your design
            println!("🔄 Password updated for: {}", name);
            true
        } else {
            println!("❌ Password '{}' not found. If it’s encrypted, decrypt it first before modifying.", name);
            false // password not found
        }
    }

    pub fn encrypt_password(&mut self, name: &str) -> bool {
        // Find the index of the password in meta
        if let Some(pos) = self.meta.passwords.iter().position(|p| p.name == name) {
            let mut entry = self.meta.passwords.remove(pos);

            if entry.is_encrypted {
                println!("❌ Password '{}' is already encrypted.", name);
                // Put it back
                self.meta.passwords.insert(pos, entry);
                return false;
            }

            // Encrypt the password
            let key = match unwrap_item_key(&entry.encrypted_item_key, &self.master_key) {
                Some(k) => k,
                None => {
                    println!("❌ Wrong password or corrupted String: {}", name);
                    // Put it back
                    self.meta.passwords.insert(pos, entry);
                    return false;
                }
            };
            let encrypted_password = match self.encrypt_string(&entry.password, &key) {
                Some(ep) => ep,
                None => {
                    println!("❌ Password encryption failed for '{}'", name);
                    // Put it back
                    self.meta.passwords.insert(pos, entry);
                    return false;
                }
            };

            // Update entry
            entry.is_encrypted = true;
            entry.password = encrypted_password;
            entry.updated_at = time::now();

            // Move to encrypted meta
            self.encrypted_meta.passwords.push(entry);

            self.save_meta();
            self.save_encrypted_meta();

            println!("🔓 Password '{}' encrypted successfully.", name);
            true
        } else {
            println!("❌ No password found with name: {}", name);
            false
        }
    }

    pub fn decrypt_password(&mut self, name: &str) -> bool {
        // Find the index of the password in encrypted_meta
        if let Some(pos) = self.encrypted_meta.passwords.iter().position(|p| p.name == name) {
            let mut entry = self.encrypted_meta.passwords.remove(pos);

            if !entry.is_encrypted {
                println!("❌ Password '{}' is not encrypted.", name);
                // Put it back
                self.encrypted_meta.passwords.insert(pos, entry);
                return false;
            }

            // Decrypt the password
            let key = match unwrap_item_key(&entry.encrypted_item_key, &self.master_key) {
                Some(k) => k,
                None => {
                    println!("❌ Wrong password or corrupted String: {}", name);
                    // Put it back
                    self.encrypted_meta.passwords.insert(pos, entry);
                    return false;
                }
            };
            let decrypted_password = match self.decrypt_string(&entry.password, &key) {
                Some(dp) => dp,
                None => {
                    println!("❌ Wrong password or corrupted saved password '{}'", name);
                    // Put it back
                    self.encrypted_meta.passwords.insert(pos, entry);
                    return false;
                }
            };

            // Update entry
            entry.is_encrypted = false;
            entry.password = decrypted_password;
            entry.updated_at = time::now();

            // Move to main meta
            self.meta.passwords.push(entry);

            self.save_meta();
            self.save_encrypted_meta();

            println!("🔓 Password '{}' decrypted successfully.", name);
            true
        } else {
            println!("❌ No encrypted password found with name: {}", name);
            false
        }
    }

    pub fn encrypt_all_passwords(&mut self) -> bool {
        let mut success_count = 0;
        let mut failure_count = 0;
        let mut passwords_to_move = Vec::new();

        // First, collect all the data we need without holding mutable references
        let password_data: Vec<(usize, String, String, String)> = self.meta.passwords
            .iter()
            .enumerate()
            .filter(|(_, p)| !p.is_encrypted)
            .map(|(i, p)| (i, p.encrypted_item_key.clone(), p.password.clone(), p.name.clone()))
            .collect();

        let total_to_process = password_data.len();
        
        if total_to_process == 0 {
            println!("ℹ️  No unencrypted passwords found to encrypt.");
            return true;
        }

        println!("🔒 Encrypting {} passwords...", total_to_process);

        // Process each password with progress reporting
        for (current, (index, encrypted_item_key, password, name)) in password_data.iter().enumerate() {
            print!("[{}/{}] Encrypting '{}'... ", current + 1, total_to_process, name);

            let key = match unwrap_item_key(encrypted_item_key, &self.master_key) {
                Some(k) => k,
                None => {
                    println!("❌ (key error)");
                    failure_count += 1;
                    continue;
                }
            };

            match self.encrypt_string(password, &key) {
                Some(encrypted_pass) => {
                    passwords_to_move.push((*index, encrypted_pass, name.clone()));
                    success_count += 1;
                    println!("✅");
                }
                None => {
                    println!("❌ (encryption failed)");
                    failure_count += 1;
                }
            }
        }

        // Now update the actual entries in reverse order to avoid index shifting
        println!("Updating database...");
        for (index, encrypted_pass, _name) in passwords_to_move.into_iter().rev() {
            if let Some(pass_entry) = self.meta.passwords.get_mut(index) {
                pass_entry.password = encrypted_pass;
                pass_entry.updated_at = time::now();
                pass_entry.is_encrypted = true;

                // Move to encrypted_meta
                let encrypted_entry = self.meta.passwords.remove(index);
                self.encrypted_meta.passwords.push(encrypted_entry);
            }
        }

        // Only save if we had successful encryptions
        if success_count > 0 {
            self.save_meta();
            self.save_encrypted_meta();
        }

        // Log results
        println!("\n📊 Encryption Summary:");
        println!("   Total:    {}", total_to_process);
        println!("   Success:  {}", success_count);
        println!("   Failed:   {}", failure_count);

        if success_count > 0 && failure_count == 0 {
            println!("✅ Successfully encrypted all {} passwords.", success_count);
        } else if success_count > 0 {
            println!("⚠️  Encrypted {}/{} passwords. {} failed.", success_count, total_to_process, failure_count);
        } else {
            println!("❌ Failed to encrypt any passwords. {}/{} failed.", failure_count, total_to_process);
        }

        failure_count == 0
    }

    pub fn decrypt_all_passwords(&mut self) -> bool {
        let mut success_count = 0;
        let mut failure_count = 0;
        let mut i = 0;

        // Collect encrypted passwords first to get count and names
        let encrypted_passwords: Vec<(usize, String)> = self.encrypted_meta.passwords
            .iter()
            .enumerate()
            .filter(|(_, p)| p.is_encrypted)
            .map(|(i, p)| (i, p.name.clone()))
            .collect();

        let total_to_process = encrypted_passwords.len();

        if total_to_process == 0 {
            println!("ℹ️  No encrypted passwords found to decrypt.");
            return true;
        }

        println!("🔓 Decrypting {} passwords...", total_to_process);

        let mut current = 0;
        while i < self.encrypted_meta.passwords.len() {
            let pass_entry = &self.encrypted_meta.passwords[i];

            if !pass_entry.is_encrypted {
                i += 1;
                continue;
            }

            current += 1;
            print!("[{}/{}] Decrypting '{}'... ", current, total_to_process, pass_entry.name);

            let key = match unwrap_item_key(&pass_entry.encrypted_item_key, &self.master_key) {
                Some(k) => k,
                None => {
                    println!("❌ (key error)");
                    failure_count += 1;
                    i += 1;
                    continue;
                }
            };

            let decrypted_password = match self.decrypt_string(&pass_entry.password, &key) {
                Some(dp) => dp,
                None => {
                    println!("❌ (decryption failed)");
                    failure_count += 1;
                    i += 1;
                    continue;
                }
            };

            // Update and move to main meta
            let mut decrypted_entry = self.encrypted_meta.passwords.remove(i);
            decrypted_entry.password = decrypted_password;
            decrypted_entry.updated_at = time::now();
            decrypted_entry.is_encrypted = false;

            self.meta.passwords.push(decrypted_entry);
            success_count += 1;
            println!("✅");
            // Don't increment i since we removed the current element
        }

        if success_count > 0 || failure_count > 0 {
            self.save_meta();
            self.save_encrypted_meta();
        }

        // Log results
        println!("\n📊 Decryption Summary:");
        println!("   Total:    {}", total_to_process);
        println!("   Success:  {}", success_count);
        println!("   Failed:   {}", failure_count);

        if success_count > 0 && failure_count == 0 {
            println!("✅ Successfully decrypted all {} passwords.", success_count);
        } else if success_count > 0 {
            println!("⚠️  Decrypted {}/{} passwords. {} failed.", success_count, total_to_process, failure_count);
        } else {
            println!("❌ Failed to decrypt any passwords. {}/{} failed.", failure_count, total_to_process);
        }

        failure_count == 0
    }
}