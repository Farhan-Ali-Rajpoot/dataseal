use super::{Config};
use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub file_name: String,
    pub encrypted_item_key: String,
    pub file_path: String,
    pub size: String,
    pub extension: String,
    pub is_encrypted: bool,
    pub is_recycled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordEntry {
    pub name: String,
    pub password: String, // plain text for now
    pub encrypted_item_key: String,
    pub is_encrypted: bool,
    pub is_recycled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DatabaseMeta {
    pub passwords: Vec<PasswordEntry>,
    pub files: Vec<FileEntry>,
}

pub struct Database {
    pub root_directory: String,
    pub master_key: [u8; 32],
    pub decrypted_dir: String,
    pub encrypted_dir: String,
    pub recycle_dir: String,
    pub decrypted_files_dir: String,
    pub meta_file: String,
    pub trash_meta_file: String,
    pub encrypted_meta_file: String,
    pub config_file: String,
    pub config: Config,
    pub meta: DatabaseMeta,
    pub trash_meta: DatabaseMeta,
    pub encrypted_meta: DatabaseMeta,
}







pub trait DatabaseType {
    fn new() -> Self;
    fn with_dir(root_directory: &str) -> Self;
    fn add_password(&mut self, name: &str, password: &str);
    fn change_password(&mut self, name: &str, new_password: &str);
    fn add_file(&mut self, name: &str, file_path: &str);
    fn save_meta(&self);
    fn save_trash_meta(&self);
    fn search_files(&self, query: &str) -> Vec<&FileEntry>;
    fn list_files(&self, query: &str) -> Vec<&FileEntry>;
    fn list_passwords(&self, query: &str) -> Vec<&PasswordEntry>;
    fn search_passwords(&self, query: &str) -> Vec<&PasswordEntry>;
    fn delete_all_passwords(&mut self);
    fn delete_all_files(&mut self);
    fn delete_file(&mut self, name: &str) -> bool;
    fn delete_password(&mut self, name: &str) -> bool;
    fn empty_recycle_bin(&mut self) -> (usize, usize);
    fn restore_password(&mut self, name: &str) -> bool;
    fn restore_file(&mut self, name: &str) -> bool;
}
