use clap::Parser;
use rpassword::read_password; // ✅ add rpassword crate in Cargo.toml
use std::io::{self, Write};

pub mod db;
pub mod cli;
pub mod gui;

use crate::cli::help_document::{help_document};

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

fn main() {
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
        println!("Coming soon...");
        return;
    }

    // ✅ Prompt user for master password interactively
    println!("Enter master password:");
    io::stdout().flush().unwrap();
    let master_password = match read_password() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Failed to read password: {}", e);
            return;
        }
    };

    println!("🚀 Launching DataSeal CLI...");
    cli::repl::start(&master_password);
}
