use clap::{Parser, Subcommand};

mod git_operations;
mod commit;
mod branch;
mod checkout;
mod config;

#[derive(Parser)]
#[command(name = "sg", version = env!("CARGO_PKG_VERSION"), about = "SuperGit: Simplify your git workflow")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Commit,
    Branch,
    Ignore,
    Config,
    Checkout {
        new_branch: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let config = config::load_config();
    let result = match &cli.command {
        Some(Commands::Commit) => commit::run_commit(config.conventional_commits, config.ticket_prefix),
        Some(Commands::Branch) => branch::run_branch(),
        Some(Commands::Checkout { new_branch }) => checkout::run_checkout(new_branch.clone()),
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