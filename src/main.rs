use clap::Parser;

pub mod db;
pub mod cli;
pub mod gui;

#[derive(Parser)]
struct Args {
    /// Launch CLI instead of default GUI
    #[arg(short,long)]
    cli: bool,
    #[arg(short, long)]
    password: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.cli {
        let master_password = match args.password {
            Some(p) => p,
            None => {
                eprintln!("❌ Please provide a master password with --master or -m");
                return;
            }
        };

        println!("🚀 Launching DataSeal CLI...");
        cli::repl::start(&master_password);
        return;
    }

    println!("🎨 GUI version is not implemented yet. Exiting...");
}
