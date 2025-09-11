use std::io::{self, Write};
use std::env::{current_dir};
use std::path::PathBuf;
use crate::db::Database;
use colored::*;

use super::{
    help_document::{help_document, unknown_command_message},
    commands::{fs_commands,}
};
// use crate::help_document::HELP_DOCUMENT;

pub fn start(master_password: &str) {
    // Initialize database
    let mut db: Database = match Database::new(master_password) {
        Some(d) => d,
        None => {
            println!("Failed to initialize database. Exiting...");
            return;
        }
    };
    // Initialize current directory
    let mut current_directory: PathBuf = current_dir().unwrap();

    println!("DataSeal CLI ready. Type 'help' for commands.");

    loop {
        // Show prompt with current path
        print!("{}{}{}", "ds:".bright_green(), current_directory.display().to_string().blue(), ">".bright_green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Split input into command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts[0] {
            "help" => println!("{}", help_document()),
            "ls" => println!("{}", fs_commands::ls()),
            "pwd" => println!("{}", fs_commands::pwd()),
            "cd" => {
                if let Some(arg) = parts.get(1) {
                    if fs_commands::cd(arg) {
                        if let Ok(dir) = current_dir() {
                            current_directory = dir;
                        }
                    }
                } else {
                    println!("Usage: cd <path>");
                }
            },
            "clear" => fs_commands::clear(),
            "exit" | "quit" => break,
            _ => println!("{}", unknown_command_message(input)),
        }
    }

    println!("Exiting DataSeal CLI.");
}
