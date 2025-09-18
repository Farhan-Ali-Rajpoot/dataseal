use std::io::{stdout, Write};
use std::{env, fs};
use std::process::Command;
use colored::*;

pub fn cd(path: &str) -> bool {
    if let Err(e) = env::set_current_dir(path) {
        eprintln!("Error: {}", e);
        return false;
    }
    match env::current_dir() {
        Ok(_dir) => {
            true
        }
        Err(e) => {
            eprintln!("cd: No such file or directory: {}", e);
            false
        }
    }
}

pub fn ls() -> String {
    match fs::read_dir(".") {
        Ok(entries) => {
            let mut result = String::new();
            for entry in entries.flatten() {
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();

                let colored_name = if path.is_dir() {
                    name.blue() // directories
                } else if path.is_file() {
                    name.bright_green() // files
                } else {
                    name.white() // other types (symlinks, etc.)
                };

                result.push_str(&format!("{}  ", colored_name));
            }
            result.trim_end().to_string()
        }
        Err(e) => format!("Error: could not read directory: {}", e),
    }
}


pub fn pwd() -> String {
    match env::current_dir() {
        Ok(dir) => dir.display().to_string(),
        Err(e) => format!("Error getting current directory: {}", e),
    }
}


pub fn clear() {
    if cfg!(target_os = "windows") {
        // On Windows, use the `cls` command
        let _ = Command::new("cmd")
            .args(&["/C", "cls"])
            .status();
    } else {
        // On Unix/Linux/macOS, use ANSI escape sequences
        print!("\x1B[2J\x1B[3J\x1B[H");
        // \x1B[2J -> clear screen
        // \x1B[3J -> clear scrollback buffer
        // \x1B[H  -> move cursor to top-left
        stdout().flush().unwrap();
    }
}
