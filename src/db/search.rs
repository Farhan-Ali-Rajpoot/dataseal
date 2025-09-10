use super::{Database, FileEntry, PasswordEntry};



impl Database {
    // Search passwords by query in active passwords only
    pub fn search_decrypted_files(&self, query: &str) -> Vec<&FileEntry> {
        self.meta.files
            .iter()
            .filter(|file| file.name.to_lowercase().contains(&query.to_lowercase()))
            .collect::<Vec<&FileEntry>>() // ✅ specify Vec type
    }
    pub fn search_decrypted_passwords(&self, query: &str) -> Vec<&PasswordEntry> {
        self.meta.passwords
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn list_decrypted_files(&self, status: Option<&str>) -> Vec<&FileEntry> {
        match status {
            Some("active") => self.meta.files.iter().collect(),
            Some("recycled") => self.trash_meta.files.iter().collect(),
            _ => {
                // Combine both active and recycled files
                self.meta.files.iter().chain(self.trash_meta.files.iter()).collect()
            }
        }
    }

    pub fn list_decrypted_passwords(&self, status: Option<&str>) -> Vec<&PasswordEntry> {
        match status {
            Some("recycled") => self.trash_meta.passwords.iter().collect(),
            _ => self.meta.passwords.iter().collect(), // default active
        }
    }

    // Encrypted items
    pub fn list_encrypted_files(&self) -> Vec<&FileEntry> {
        self.encrypted_meta.files.iter().collect()
    }

    pub fn list_encrypted_passwords(&self) -> Vec<&PasswordEntry> {
        self.encrypted_meta.passwords.iter().collect()
    }

    pub fn search_encrypted_passwords(&self, query: &str) -> Vec<&PasswordEntry> {
        self.encrypted_meta.passwords
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    pub fn search_encrypted_files(&self, query: &str) -> Vec<&FileEntry> {
        self.encrypted_meta.files
            .iter()
            .filter(|file| file.name.to_lowercase().contains(&query.to_lowercase()))
            .collect::<Vec<&FileEntry>>() // ✅ specify Vec type
    }
 
    // Deleted / Recycled items
    pub fn list_deleted_files(&self) -> Vec<&FileEntry> {
        self.trash_meta.files.iter().collect()
    }

    pub fn list_deleted_passwords(&self) -> Vec<&PasswordEntry> {
        self.trash_meta.passwords.iter().collect()
    }

    pub fn search_deleted_files(&self, query: &str) -> Vec<&FileEntry> {
        self.trash_meta.files
            .iter()
            .filter(|file| file.name.to_lowercase().contains(&query.to_lowercase()))
            .collect::<Vec<&FileEntry>>() // ✅ specify Vec type
    }

    pub fn search_deleted_passwords(&self, query: &str) -> Vec<&PasswordEntry> {
        self.trash_meta.passwords
            .iter()
            .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
            .collect()
    }

    // All items (Files + Passwords)
    pub fn list_all_files(&self) -> Vec<&FileEntry> {
        self.meta.files
            .iter()
            .chain(self.encrypted_meta.files.iter())
            .collect()
    }

    pub fn list_all_passwords(&self) -> Vec<&PasswordEntry> {
        self.meta.passwords
            .iter()
            .chain(self.encrypted_meta.passwords.iter())
            .collect()
    }
}