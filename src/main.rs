use clap::{Parser, Subcommand};

mod git_operations;
mod standard;

#[derive(Parser)]
#[command(name = "cmt", version = "1.0", about = "Commit management tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Commit,
    Ignore,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Commit) => {
            if let Err(err) = standard::run_standard() {
                eprintln!("❌ Error: {}", err);
                std::process::exit(1);
            }
        }
        Some(Commands::Ignore) => {
            println!("Ignore logic to implement later");
        }
        None => {
            println!("Default logic to implement later");
        }
    }
}