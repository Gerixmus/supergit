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
    Branch {
        #[arg(short = 'd', long = "delete", help = "Delete a branch")]
        delete: bool,
        #[arg(short = 'D', help = "Force delete a branch")]
        force_delete: bool,
    },
    Ignore,
    Config,
    Checkout {
        #[arg(short = 'b', long = "branch", help = "Create a new branch")] 
        create_new: bool,
    },
}

fn main() {
    let cli = Cli::parse();
    let config = config::load_config();
    let result = match &cli.command {
        Some(Commands::Commit) => commit::run_commit(config.conventional_commits, config.ticket_prefix),
        Some(Commands::Branch { delete , force_delete})=> branch::run_branch(delete.clone(), force_delete.clone()),
        Some(Commands::Checkout { create_new }) => checkout::run_checkout(create_new.clone()),
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