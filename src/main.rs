use clap::Parser;

pub mod db;
pub mod cli;
pub mod gui;

#[derive(Parser)]
#[command(name = "DataSeal")]
#[command(about = "A simple database app with CLI and GUI modes", long_about = None)]
struct Args {
    /// Launch GUI version (default is CLI)
    #[arg(long)]
    gui: bool,

    /// Master password for CLI mode
    #[arg(short, long)]
    password: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.gui {
        println!("🎨 Launching DataSeal GUI...");
        gui::main::start(); // your GUI entrypoint (not yet implemented)
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
}
