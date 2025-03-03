use clap::{Parser, Subcommand};

mod git_operations;
mod commit;
mod branch;
mod checkout;
mod config;

#[derive(Parser)]
#[command(name = "cmt", version = "1.0", about = "Commit management tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Commit,
    Checkout,
    Branch,
    Ignore,
    Config
}

fn main() {
    let cli = Cli::parse();
    let config = config::load_config();
    let result = match &cli.command {
        Some(Commands::Commit) => commit::run_commit(config.conventional_commits, config.ticket_prefix),
        Some(Commands::Branch) => branch::run_branch(),
        Some(Commands::Checkout) => checkout::run_checkout(),
        Some(Commands::Ignore) => {
            println!("Ignore logic to implement later");
            Ok(())
        }
        Some(Commands::Config) => config::run_config(),
        None => {
            println!("Default logic to implement later");
            Ok(())
        }
    };
    
    if let Err(err) = result {
        eprintln!("❌ Error: {}", err);
        std::process::exit(1);
    }
}