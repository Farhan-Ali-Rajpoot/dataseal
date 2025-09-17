use super::Database;
use crate::db::enc_keys::{unwrap_item_key, wrap_item_key};

impl Database {
    pub fn change_master_password(&mut self, old_password: &str, new_password: &str) -> bool {
        // Step 1: store old master key
        let old_master_key = self.master_key.clone();

        // Step 2: generate new master key
        let new_master_key = match self.config.change_master_password(old_password, new_password) {
            Some(k) => k,
            None => {
                eprintln!("❌ Wrong master password.");
                return false;
            }
        };

        // Helper closure to rewrap item keys
        let rewrap = |enc_b64: &str, old_key: &[u8], new_key: &[u8]| -> Option<String> {
            if enc_b64.is_empty() { return Some(String::new()); }
            let item_key = unwrap_item_key(enc_b64, old_key)?;
            wrap_item_key(&item_key, new_key)
        };

        // Step 3: create temporary copies of all metas
        let mut meta_tmp = self.meta.clone();
        let mut encrypted_meta_tmp = self.encrypted_meta.clone();
        let mut trash_meta_tmp = self.trash_meta.clone();

        // Step 4: rewrap passwords in temporary metas
        for entry in &mut meta_tmp.passwords {
            match rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                Some(new_key) => entry.encrypted_item_key = new_key,
                None => {
                    eprintln!("❌ Failed to rewrap password '{}'", entry.name);
                    return false;
                }
            }
        }

        for entry in &mut encrypted_meta_tmp.passwords {
            match rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                Some(new_key) => entry.encrypted_item_key = new_key,
                None => {
                    eprintln!("❌ Failed to rewrap encrypted password '{}'", entry.name);
                    return false;
                }
            }
        }

        for entry in &mut trash_meta_tmp.passwords {
            match rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                Some(new_key) => entry.encrypted_item_key = new_key,
                None => {
                    eprintln!("❌ Failed to rewrap trashed password '{}'", entry.name);
                    return false;
                }
            }
        }

        // Step 5: rewrap files in temporary metas
        for entry in &mut meta_tmp.files {
            match rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                Some(new_key) => entry.encrypted_item_key = new_key,
                None => {
                    eprintln!("❌ Failed to rewrap file '{}'", entry.name);
                    return false;
                }
            }
        }

        for entry in &mut encrypted_meta_tmp.files {
            match rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                Some(new_key) => entry.encrypted_item_key = new_key,
                None => {
                    eprintln!("❌ Failed to rewrap encrypted file '{}'", entry.name);
                    return false;
                }
            }
        }

        for entry in &mut trash_meta_tmp.files {
            match rewrap(&entry.encrypted_item_key, &old_master_key, &new_master_key) {
                Some(new_key) => entry.encrypted_item_key = new_key,
                None => {
                    eprintln!("❌ Failed to rewrap trashed file '{}'", entry.name);
                    return false;
                }
            }
        }

        // Step 6: All rewrapped successfully, swap in temporary metas
        self.meta = meta_tmp;
        self.encrypted_meta = encrypted_meta_tmp;
        self.trash_meta = trash_meta_tmp;

        // Step 7: Update master key and save all metas
        self.master_key = new_master_key;
        let _ = self.save_meta();
        let _ = self.save_trash_meta();
        let _ = self.save_encrypted_meta();

        println!("✅ Master password changed and all item keys safely rewrapped.");
        true
    }
}
