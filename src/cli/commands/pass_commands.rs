use crate::db::{Database,PasswordEntry};
use colored::*;

pub fn restore_all_passwords(db: &mut Database) -> bool {
    db.restore_all_passwords()
}

pub fn add_password(db: &mut Database, parts: &[&str]) -> bool {

    let name = parts[1];
    let password = parts[2];

    if db.add_password(name, password) {
        true
    } else {
        false
    }
}

pub fn change_password(db: &mut Database, parts: &[&str]) -> bool {
    let name = parts[1];
    let new_password = parts[2];
    if db.change_password(name, new_password) {
        true
    }else {
        false
    }
}

pub fn encrypt_password(db: &mut Database, parts: &[&str]) -> bool {
    let name = parts[1];
    if db.encrypt_password(name) {
        true
    }else {
        false
    }
}

pub fn decrypt_password(db: &mut Database, parts: &[&str]) -> bool {
    let name = parts[1];
    if db.decrypt_password(name) {
        true
    }else {
        false
    }
}

pub fn delete_password(db: &mut Database, parts: &[&str]) -> bool {
    let name = parts[1];

    if db.delete_password(name) {
        true
    }else {
        false
    }
}

pub fn delete_all_passwords(db: &mut Database) -> bool {
    if db.delete_all_passwords() {
        true
    }else {
        false
    }
}

pub fn restore_password(db: &mut Database, parts: &[&str]) -> bool {
    let name = parts[1];
    db.restore_password(name) 
}

pub fn list_decrypted_passwords(db: &mut Database) -> bool {
    let passwords = db.list_decrypted_passwords(Some("active"));
    print_passwords_pretty(&passwords);
    true
}

pub fn list_encrypted_passwords(db: &mut Database) -> bool {
    let passwords = db.list_encrypted_passwords();
    print_passwords_pretty(&passwords);
    true
}

pub fn list_deleted_passwords(db: &mut Database) -> bool {
    let passwords = db.list_deleted_passwords();
    print_passwords_pretty(&passwords);
    true
} 

pub fn search_encrypted_passwords(db: &mut Database, parts: &[&str])-> bool {
    let query = parts[1];
    let passwords = db.search_encrypted_passwords(query);
    print_passwords_pretty(&passwords);
    true
}

pub fn search_decrypted_passwords(db: &mut Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let passwords = db.search_decrypted_passwords(query);
    print_passwords_pretty(&passwords);
    true
}

pub fn search_deleted_passwords(db: &mut Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let passwords = db.search_deleted_passwords(query);
    print_passwords_pretty(&passwords);
    true
}

pub fn list_all_passwords(db: &mut Database) -> bool {
    let mut passwords = db.list_decrypted_passwords(Some("active"));
    passwords.extend(db.list_encrypted_passwords());
    print_passwords_pretty(&passwords);
    true
}

pub fn search_all_passwords(db: &mut Database, parts: &[&str]) -> bool {
    let query = parts[1];
    let mut passwords = db.search_decrypted_passwords(query);
    passwords.extend(db.search_encrypted_passwords(query));

    print_passwords_pretty(&passwords);
    true
}

// Helper functions
/// Print a list of passwords in a pretty, aligned table.
pub fn print_passwords_pretty(passwords: &[&PasswordEntry]) {
    if passwords.is_empty() {
        println!("{}", "No passwords found.".yellow());
        return;
    }

    println!(); // blank line on top

    // Calculate column widths based on header and content
    let mut name_width = 25;
    let mut password_width = 20;
    let mut encrypted_width = 12;
    let mut recycled_width = 10;
    let mut created_width = 22;
    let mut updated_width = 22;

    // Find maximum content lengths
    for p in passwords {
        name_width = name_width.max(p.name.len());
        // For encrypted passwords, we'll show "********", so width is 8
        password_width = password_width.max(if p.is_encrypted { 8 } else { p.password.len() });
        encrypted_width = encrypted_width.max(3); // "Yes"/"No"
        recycled_width = recycled_width.max(3); // "Yes"/"No"
        created_width = created_width.max(p.created_at.len());
        updated_width = updated_width.max(p.updated_at.len());
    }

    // Add some padding and ensure minimum widths
    name_width = (name_width + 2).max(10);
    password_width = (password_width + 2).max(15);
    encrypted_width = (encrypted_width + 2).max(12);
    recycled_width = (recycled_width + 2).max(10);
    created_width = (created_width + 2).max(15);
    updated_width = (updated_width + 2).max(15);

    // Header
    println!(
        "{:<name_width$} {:<password_width$} {:<encrypted_width$} {:<recycled_width$} {:<created_width$} {:<updated_width$}",
        "Name".bright_yellow(),
        "Password".bright_yellow(),
        "Encrypted".bright_yellow(),
        "Deleted".bright_yellow(),
        "Created".bright_yellow(),
        "Updated".bright_yellow(),
        name_width = name_width,
        password_width = password_width,
        encrypted_width = encrypted_width,
        recycled_width = recycled_width,
        created_width = created_width,
        updated_width = updated_width
    );

    // Separator line (calculate total width)
    let total_width = name_width + password_width + encrypted_width + recycled_width + created_width + updated_width + 5;
    println!("{}", "-".repeat(total_width).blue());

    // Rows
    for p in passwords {
        let encrypted_str = if p.is_encrypted { 
            "Yes".bright_green().bold() 
        } else { 
            "No".red() 
        };
        
        let recycled_str = if p.is_recycled { 
            "Yes".red().bold()
        } else { 
            "No".bright_green() 
        };

        // Show asterisks for encrypted passwords, actual password for plain text
        let password_display = if p.is_encrypted {
            "********".to_string()
        } else if p.password.len() > password_width - 2 {
            format!("{}...", &p.password[..password_width.saturating_sub(5)])
        } else {
            p.password.clone()
        };

        // Apply color to password based on encryption status
        let password_colored = if p.is_encrypted {
            password_display.yellow() // Encrypted passwords in yellow (asterisks)
        } else {
            password_display.bright_red() // Plain text passwords in red (warning)
        };

        println!(
            "{:<name_width$} {:<password_width$} {:<encrypted_width$} {:<recycled_width$} {:<created_width$} {:<updated_width$}",
            p.name.bright_cyan(),
            password_colored,
            encrypted_str,
            recycled_str,
            p.created_at.dimmed(),
            p.updated_at.dimmed(),
            name_width = name_width,
            password_width = password_width,
            encrypted_width = encrypted_width,
            recycled_width = recycled_width,
            created_width = created_width,
            updated_width = updated_width
        );
    }

    println!(); // blank line at bottom
}