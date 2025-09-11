use super::Database;

use crate::db::enc_keys::{unwrap_item_key, wrap_item_key};



impl Database {
    pub fn change_master_password(&mut self, old_password: &str, new_password: &str) -> bool {
        // Step 1: store old master key
        let old_master_key = self.master_key.clone();

        // Step 2: change master password, get new master key
        let new_master_key = match self.config.change_master_password(old_password, new_password) {
            Some(k) => k,
            None => {
                eprintln!("❌ Failed to change master password.");
                return false;
            }
        };

        // Step 3: helper closure to rewrap item keys
        let rewrap = |enc_b64: &str, old_key: &[u8], new_key: &[u8]| -> Option<String> {
            if enc_b64.is_empty() { return Some(String::new()); }
            let item_key = unwrap_item_key(enc_b64, old_key)?; // <-- free function
            wrap_item_key(&item_key, new_key)
        };


        // Step 4: loop over passwords (meta, encrypted, trash)
        for entry in &mut self.meta.passwords {
            if let Some(new_key) = rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                entry.encrypted_item_key = new_key;
            }
        }
        for entry in &mut self.encrypted_meta.passwords {
            if let Some(new_key) = rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                entry.encrypted_item_key = new_key;
            }
        }
        for entry in &mut self.trash_meta.passwords {
            if let Some(new_key) = rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                entry.encrypted_item_key = new_key;
            }
        }

        // Step 5: loop over files (meta, encrypted, trash)
        for entry in &mut self.meta.files {
            if let Some(new_key) = rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                entry.encrypted_item_key = new_key;
            }
        }
        for entry in &mut self.encrypted_meta.files {
            if let Some(new_key) = rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                entry.encrypted_item_key = new_key;
            }
        }
        for entry in &mut self.trash_meta.files {
            if let Some(new_key) = rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                entry.encrypted_item_key = new_key;
            }
        }

        // Step 6: update master key and save all metas
        self.master_key = new_master_key;
        let _ = self.save_meta();
        let _ = self.save_trash_meta();
        let _ = self.save_encrypted_meta();

        println!("✅ Master password changed and all item keys rewrapped.");
        true
    }
}
