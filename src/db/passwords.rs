use super::{Database,PasswordEntry};

use crate::db::time;




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

        let item_key = self.generate_item_key();
        let encrypted_item_key = match self.wrap_item_key(&item_key) {
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

    pub fn change_password (&mut self,name: &str, new_password: &str) -> bool {
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
            println!("Password '{}' not found. If it’s encrypted, decrypt it first before modifying.", name);
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
            let key = match self.unwrap_item_key(&entry.encrypted_item_key) {
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

            println!("✅ Password '{}' encrypted successfully.", name);
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
            let key = match self.unwrap_item_key(&entry.encrypted_item_key) {
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

}