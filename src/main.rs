use clap::Parser;
use colored::*;
use rpassword::read_password;
use std::io::{self, Write};
use std::path::Path;

pub mod db;
pub mod cli;
pub mod gui;

use crate::{
    db::structs::DatabaseArguments,
    cli::help_document::help_document
};

#[derive(Parser)]
#[command(
    name = "DataSeal",
    about = "A simple database app with CLI and GUI modes",
    version = env!("CARGO_PKG_VERSION")  
)]
struct Args {
    /// Launch GUI version (default is CLI)
    #[arg(long)]
    gui: bool,
}

fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt.bright_blue().bold());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("❌ Failed to read input");
    input.trim().to_string()
}

fn prompt_password(prompt: &str) -> String {
    print!("{}", prompt.bright_blue().bold());
    io::stdout().flush().unwrap();
    
    read_password().expect("❌ Failed to read password")
}

fn confirm_operation(question: &str) -> bool {
    print!("{} {} ", question.bright_yellow().bold(), "(y/N)".bright_black());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("❌ Failed to read input");
    
    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

fn main() {
    println!("{}", "🔐 DataSeal - Secure Database Manager".bright_green().bold());
    println!("{}", "=".repeat(50).bright_black());
    
    let mut args_raw = std::env::args();
    let _bin = args_raw.next();

    match args_raw.next().as_deref() {
        Some("-v") | Some("version") | Some("--version") => {
            println!("DataSeal {}", env!("CARGO_PKG_VERSION"));
            return;
        },
        Some("help") | Some("--help") | Some("-h") => {
            println!("{}", help_document());
            return;
        },
        _ => {}
    }

    let args = Args::parse();

    if args.gui {
        println!("{}", "🎨 GUI version coming soon...".bright_cyan());
        return;
    }

    // Check if config exists
    let root_directory = Path::new(".cache/local");
    let config_path = root_directory.join(".config.json");
    
    let mut db_args: DatabaseArguments = if config_path.exists() {
        println!("{} {}", "📁 Config path:".bright_cyan(), config_path.display().to_string().bright_white());
        
        // Login flow
        let master_password = prompt_password("🔑 Enter master password: ");
        
        DatabaseArguments {
            db_name: "default".to_string(), // Will be read from config
            owner: "user".to_string(),      // Will be read from config  
            description: "Existing database".to_string(), // Will be read from config
            master_password: master_password.to_string(),
            is_nested: false,
            root_directory: root_directory.to_path_buf(),
        }
    } else {
        // Signup flow
        println!("{}", "🆕 Welcome to DataSeal! Let's set up your database.".bright_green());
        println!("{}", "―".repeat(50).bright_black());
        
        let db_name = prompt_user("📂 Enter database name: ");
        let owner = prompt_user("👤 Enter your name: ");
        let description = prompt_user("📝 Enter database description: ");
        
        println!();
        println!("{}", "🔐 Master Password Requirements:".bright_yellow());
        println!("{}", "   • At least 8 characters".bright_white());
        println!("{}", "   • Mix of letters, numbers, and symbols".bright_white());
        println!();
        
        let master_password = prompt_password("🔑 Create master password: ");
        let confirm_password = prompt_password("🔑 Confirm master password: ");
        
        if master_password != confirm_password {
            eprintln!("{}", "❌ Passwords do not match!".bright_red());
            return;
        }
        
        if master_password.len() < 8 {
            eprintln!("{}", "❌ Password must be at least 8 characters long!".bright_red());
            return;
        }
        
        println!();
        println!("{}", "📋 Configuration Summary:".bright_cyan());
        println!("{}", "―".repeat(30).bright_black());
        println!("{} {}", "Database:".bright_white(), db_name.bright_green());
        println!("{} {}", "Owner:".bright_white(), owner.bright_green());
        println!("{} {}", "Description:".bright_white(), description.bright_green());
        println!("{} {}", "Storage:".bright_white(), root_directory.display().to_string().bright_green());
        println!();
        
        if !confirm_operation("Proceed with this configuration?") {
            println!("{}", "👋 Setup cancelled.".bright_yellow());
            return;
        }
        
        DatabaseArguments {
            db_name: db_name.to_string(), // Convert to &'static str
            owner: owner.to_string(),     // Convert to &'static str
            description: description.to_string(), // Convert to &'static str
            master_password: master_password.to_string(),
            is_nested: false,
            root_directory: root_directory.to_path_buf(),
        }
    };

    println!();
    println!("{}", "🚀 Launching DataSeal CLI...".bright_green().bold());
    
    // Pass ownership to start function
    cli::repl::start(&mut db_args);
}