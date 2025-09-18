use clap::Parser;

pub mod db;
pub mod cli;
pub mod gui;

use crate::cli::help_document::{help_document};

#[derive(Parser)]
#[command(
    name = "DataSeal",
    about = "A simple database app with CLI and GUI modes",
    version = env!("CARGO_PKG_VERSION")  // ✅ adds built-in -V/--version
)]
struct Args {
    /// Launch GUI version (default is CLI)
    #[arg(long)]
    gui: bool,

    /// Master password for CLI mode.
    #[arg(short, long)]
    password: Option<String>,
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
        },
        _ => {}
    }



    let args = Args::parse();

    if args.gui {
        println!("Coming soon...");
        return;
    }

    let master_password = match args.password {
        Some(p) => p,
        None => {
            eprintln!("❌ Please provide a master password with --password or -p");
            return;
        }
    };

    println!("🚀 Launching DataSeal CLI...");
    cli::repl::start(&master_password);
    // call your CLI logic here
}
