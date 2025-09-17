use super::utils::{run_in_dir};
use std::path::{Path, PathBuf};
use colored::*;
use crate::db::{Database, FileEntry};


pub fn restore_all_files(db: &mut Database) -> bool {
    db.restore_all_files()
}

pub fn empty_recycle_bin_verbose(db: &mut Database) -> bool {
    let (deleted_passwords, deleted_files) = db.empty_recycle_bin();

    if deleted_passwords == 0 && deleted_files == 0 {
        println!("🗑️ Recycle bin is already empty.");
    } else {
        println!("🗑️ Recycle Bin Emptied!");
        println!("Deleted passwords: {}", deleted_passwords);
        println!("Deleted files: {}", deleted_files);
    }

    true
}

pub fn add_file(
    db: &mut Database,
    parts: &[&str],
    current_dir: &Path,
    initial_dir: &Path
) -> bool {
    let name = parts[1];
    let filename = parts[2];

    let full_path = if Path::new(filename).is_absolute() {
        PathBuf::from(filename)
    } else {
        current_dir.join(filename)
    };

    if !full_path.exists() {
        eprintln!("{}: file not found", full_path.display().to_string().red());
        return false;
    }

    run_in_dir(initial_dir, || {
        db.add_file(name, full_path.to_string_lossy().as_ref())
    })
}

pub fn encrypt_file(db: &mut Database, parts: &[&str], initial_dir: &Path) -> bool {
    let filename = parts[1];
    run_in_dir(initial_dir, || {
        db.encrypt_file(filename)
    })
}

pub fn decrypt_file(db: &mut Database, parts: &[&str], initial_dir: &Path) -> bool {
    let filename = parts[1];
    run_in_dir(initial_dir, || {
        db.decrypt_file(filename)
    })
}

pub fn delete_file(db: &mut Database, parts: &[&str]) -> bool {
    let name = parts[1];
    db.delete_file(name)
}

pub fn delete_all_files(db: &mut Database) -> bool {
    db.delete_all_files()
}

pub fn restore_file(db: &mut Database, parts: &[&str]) -> bool {
    let name = parts[1];
    db.restore_file(name)
}

pub fn list_decrypted_files(db: &mut Database) -> bool {
    let files = db.list_decrypted_files(Some("active"));
    print_files_pretty(&files);
    true
}

pub fn list_encrypted_files(db: &mut Database) -> bool {
    let files = db.list_encrypted_files();
    print_files_pretty(&files);
    true
}

pub fn list_deleted_files(db: &mut Database) -> bool {
    let files = db.list_deleted_files();
    print_files_pretty(&files);
    true
}

pub fn search_decrypted_files(db: &mut Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let files = db.search_decrypted_files(query);
    print_files_pretty(&files);
    true
}

pub fn search_encrypted_files(db: &mut Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let files = db.search_encrypted_files(query);
    print_files_pretty(&files);
    true
}

pub fn search_deleted_files(db: &mut Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let files = db.search_deleted_files(query);
    print_files_pretty(&files);
    true
}

pub fn list_all_files(db: &mut Database) -> bool {
    let mut files = db.list_decrypted_files(Some("active"));
    files.extend(db.list_encrypted_files());

    print_files_pretty(&files);
    true
}

pub fn search_all_files(db: &mut Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let mut files = db.search_decrypted_files(query);
    files.extend(db.search_encrypted_files(query));

    print_files_pretty(&files);
    true
}

// Helper function
pub fn print_files_pretty(files: &[&FileEntry]) {
    if files.is_empty() {
        println!("{}", "No files found.".yellow());
        return;
    }

    println!(); // blank line on top

    // Calculate column widths based on header and content
    let mut name_width = 25;
    let mut ext_width = 10;
    let mut size_width = 12;
    let mut encrypted_width = 12;
    let mut recycled_width = 10;
    let mut created_width = 22;
    let mut updated_width = 22;

    // Find maximum content lengths
    for f in files {
        name_width = name_width.max(f.name.len());
        ext_width = ext_width.max(f.extension.len());
        size_width = size_width.max(f.size.len() + 3); // +3 for decimal places
        encrypted_width = encrypted_width.max(3); // "Yes"/"No"
        recycled_width = recycled_width.max(3); // "Yes"/"No"
        created_width = created_width.max(f.created_at.len());
        updated_width = updated_width.max(f.updated_at.len());
    }

    // Add some padding and ensure minimum widths
    name_width = (name_width + 2).max(10);
    ext_width = (ext_width + 2).max(8);
    size_width = (size_width + 2).max(12);
    encrypted_width = (encrypted_width + 2).max(12);
    recycled_width = (recycled_width + 2).max(10);
    created_width = (created_width + 2).max(15);
    updated_width = (updated_width + 2).max(15);

    // Header - Reordered to match password layout: Name, Ext, Size, Encrypted, Deleted, Created, Updated
    println!(
        "{:<name_width$} {:<ext_width$} {:<size_width$} {:<encrypted_width$} {:<recycled_width$} {:<created_width$} {:<updated_width$}",
        "Name".bright_yellow(),
        "Ext".bright_yellow(),
        "Size(MB)".bright_yellow(),
        "Encrypted".bright_yellow(),
        "Deleted".bright_yellow(),
        "Created".bright_yellow(),
        "Updated".bright_yellow(),
        name_width = name_width,
        ext_width = ext_width,
        size_width = size_width,
        encrypted_width = encrypted_width,
        recycled_width = recycled_width,
        created_width = created_width,
        updated_width = updated_width
    );

    // Separator line (calculate total width)
    let total_width = name_width + ext_width + size_width + encrypted_width + recycled_width + created_width + updated_width + 6;
    println!("{}", "-".repeat(total_width).blue());

    // Rows
    for f in files {
        let encrypted_str = if f.is_encrypted { 
            "Yes".bright_green().bold() // Encrypted = green
        } else { 
            "No".red() // Not encrypted = red (warning)
        };
        
        let recycled_str = if f.is_recycled { 
            "Yes".red().bold() // Deleted = red
        } else { 
            "No".bright_green() // Not deleted = green
        };

        // Format size nicely - handle parsing errors gracefully
        let size: Result<f64, _> = f.size.parse();
        let size_display = match size {
            Ok(val) => format!("{:.6}", val),
            Err(_) => "N/A".to_string(),
        };

        println!(
            "{:<name_width$} {:<ext_width$} {:<size_width$} {:<encrypted_width$} {:<recycled_width$} {:<created_width$} {:<updated_width$}",
            f.name.bright_cyan(),
            f.extension.normal(),
            size_display,
            encrypted_str,
            recycled_str,
            f.created_at.dimmed(),
            f.updated_at.dimmed(),
            name_width = name_width,
            ext_width = ext_width,
            size_width = size_width,
            encrypted_width = encrypted_width,
            recycled_width = recycled_width,
            created_width = created_width,
            updated_width = updated_width
        );
    }

    println!(); // blank line at bottom
}