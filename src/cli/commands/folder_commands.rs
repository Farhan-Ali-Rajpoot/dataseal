use super::{
    structs::{Database, FolderEntry},
    colored::*,
    terminal_size::{terminal_size, Width},
};








pub fn create_folder(db: &mut Database, parts: &[&str]) -> bool {
    let folder_names = &parts[1..];
    let mut success_count = 0;

    for name in folder_names {
        if db.create_folder(name) {
            success_count += 1;
        }
    }

    println!("✅ Created {} folder(s) successfully", success_count);
    success_count > 0
}

pub fn delete_folder(db: &mut Database, parts: &[&str]) -> bool {
    let folder_name = parts[1];

    db.delete_folder(folder_name, false)
}

pub fn delete_folder_force(db: &mut Database, parts: &[&str]) -> bool {
    let folder_name = parts[1];
    db.delete_folder(folder_name, true)
}

// List
pub fn list_decrypted_folders(db: &Database) -> bool {
    let folders = db.list_decrypted_folders();

    print_folders_pretty(&folders);
    true
}

pub fn list_encrypted_folders(db: &Database) -> bool {
    let folders = db.list_encrypted_folders();

    print_folders_pretty(&folders);
    true
}

pub fn list_deleted_folders(db: &Database) -> bool {
    let folders = db.list_deleted_folders();

    print_folders_pretty(&folders);
    true
}

pub fn list_folders(db: &Database) -> bool {
    let folders = db.list_all_folders();

    print_folders_pretty(&folders);
    true
}
// Search 
pub fn search_decrypted_folders(db: &Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let folders = db.search_decrypted_folders(&query);

    print_folders_pretty(&folders);
    true
}

pub fn search_encrypted_folders(db: &Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let folders = db.search_encrypted_folders(&query);

    print_folders_pretty(&folders);
    true
}

pub fn search_deleted_folders(db: &Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let folders = db.search_deleted_folders(&query);

    print_folders_pretty(&folders);
    true
}

pub fn search_folders(db: &Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let folders = db.search_decrypted_folders(&query);

    print_folders_pretty(&folders);
    true
}




// Helper function
pub fn print_folders_pretty(folders: &[&FolderEntry]) {
    if folders.is_empty() {
        println!("{}", "No folders found.".yellow());
        return;
    }

    println!(); // blank line on top

    // Get terminal width (fallback to 100 if unknown)
    let term_width = match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 100,
    };

    // Calculate minimum required widths based on content
    let mut name_width = 25;
    let mut size_width = 10;
    let encrypted_width = 10; // Fixed width for "Encrypted"/"Yes"/"No"
    let recycled_width = 10;  // Fixed width for "Deleted"/"Yes"/"No"
    let empty_width = 8;      // Fixed width for "Empty"/"Yes"/"No"
    let mut files_width = 8;  // For sub-files count
    let mut subfolders_width = 12; // For sub-folders count
    let mut created_width = 20;
    let mut updated_width = 20;

    // Adjust widths according to content
    for f in folders {
        name_width = name_width.max(f.name.len() + 2);
        size_width = size_width.max(f.size.len() + 2);
        files_width = files_width.max(f.sub_files.len().to_string().len() + 2);
        subfolders_width = subfolders_width.max(f.sub_folders.len().to_string().len() + 2);
        created_width = created_width.max(f.created_at.len() + 2);
        updated_width = updated_width.max(f.updated_at.len() + 2);
    }

    // Compute total width
    let total_width = name_width + size_width + encrypted_width + 
                     recycled_width + empty_width + files_width + subfolders_width + 
                     created_width + updated_width + 8;

    // If total width exceeds terminal, scale proportionally but preserve name column
    if total_width > term_width {
        // Calculate available width for variable columns (excluding fixed ones)
        let fixed_columns_width = encrypted_width + recycled_width + empty_width + 8;
        let available_width = term_width - fixed_columns_width;
        
        // Distribute available width proportionally to variable columns
        let variable_columns_total = name_width + size_width + files_width + 
                                   subfolders_width + created_width + updated_width;
        let scale = available_width as f64 / variable_columns_total as f64;
        
        name_width = (name_width as f64 * scale).max(15.0) as usize; // Minimum 15 chars for name
        size_width = (size_width as f64 * scale).max(8.0) as usize;
        files_width = (files_width as f64 * scale).max(6.0) as usize;
        subfolders_width = (subfolders_width as f64 * scale).max(8.0) as usize;
        created_width = (created_width as f64 * scale).max(12.0) as usize;
        updated_width = (updated_width as f64 * scale).max(12.0) as usize;
    }

    // Header
    println!(
        "{:<name_width$} {:<size_width$} {:<encrypted_width$} {:<recycled_width$} {:<empty_width$} {:<files_width$} {:<subfolders_width$} {:<created_width$} {:<updated_width$}",
        "Name".bright_yellow(),
        "Size".bright_yellow(),
        "Encrypted".bright_yellow(),
        "Deleted".bright_yellow(),
        "Empty".bright_yellow(),
        "Files".bright_yellow(),
        "Subfolders".bright_yellow(),
        "Created".bright_yellow(),
        "Updated".bright_yellow(),
        name_width = name_width,
        size_width = size_width,
        encrypted_width = encrypted_width,
        recycled_width = recycled_width,
        empty_width = empty_width,
        files_width = files_width,
        subfolders_width = subfolders_width,
        created_width = created_width,
        updated_width = updated_width
    );

    println!("{}", "-".repeat(term_width.min(total_width)).blue());

    // Rows - handle long names with truncation and ellipsis
    for f in folders {
        let encrypted_str = if f.is_encrypted {
            "Yes".bright_green().bold()
        } else {
            "No".red()
        };
        let recycled_str = if f.is_recycled {
            "Yes".red().bold()
        } else {
            "No".bright_green()
        };
        let empty_str = if f.is_empty {
            "Yes".bright_green()
        } else {
            "No".yellow().bold()
        };

        let files_count = f.sub_files.len();
        let subfolders_count = f.sub_folders.len();

        // Color code based on content
        let files_display = if files_count > 0 {
            format!("{}", files_count).bright_cyan()
        } else {
            format!("{}", files_count).dimmed()
        };

        let subfolders_display = if subfolders_count > 0 {
            format!("{}", subfolders_count).bright_magenta()
        } else {
            format!("{}", subfolders_count).dimmed()
        };

        // Truncate long names with ellipsis
        let name_display = if f.name.len() > name_width {
            format!("{}...", &f.name[..name_width.saturating_sub(3)])
        } else {
            f.name.clone()
        };

        let created_display = if f.created_at.len() > created_width {
            format!("{}...", &f.created_at[..created_width.saturating_sub(3)])
        } else {
            f.created_at.clone()
        };

        let updated_display = if f.updated_at.len() > updated_width {
            format!("{}...", &f.updated_at[..updated_width.saturating_sub(3)])
        } else {
            f.updated_at.clone()
        };

        // Size display with formatting
        let size_display = if f.size == "0" || f.size.is_empty() {
            "0".to_string()
        } else {
            f.size.clone()
        };

        println!(
            "{:<name_width$} {:<size_width$} {:<encrypted_width$} {:<recycled_width$} {:<empty_width$} {:<files_width$} {:<subfolders_width$} {:<created_width$} {:<updated_width$}",
            name_display.bright_cyan(),
            size_display,
            encrypted_str,
            recycled_str,
            empty_str,
            files_display,
            subfolders_display,
            created_display.dimmed(),
            updated_display.dimmed(),
            name_width = name_width,
            size_width = size_width,
            encrypted_width = encrypted_width,
            recycled_width = recycled_width,
            empty_width = empty_width,
            files_width = files_width,
            subfolders_width = subfolders_width,
            created_width = created_width,
            updated_width = updated_width
        );
    }

    println!(); // blank line at bottom
}