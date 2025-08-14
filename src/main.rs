use clap::{Parser, Subcommand};

mod branch;
mod checkout;
mod commit;
mod git_operations;
mod init;

#[derive(Parser)]
#[command(name = "sg", version = env!("CARGO_PKG_VERSION"), about = "SuperGit: Simplify your git workflow")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Configure the tool")]
    Init,
    #[command(about = "Record changes to the repository")]
    Commit,
    #[command(about = "List, create, or delete branches")]
    Branch {
        #[arg(short = 'd', long = "delete", help = "Delete a branch")]
        delete: bool,
        #[arg(short = 'D', help = "Force delete a branch")]
        force_delete: bool,
    },
    #[command(about = "Switch branches or restore working tree files")]
    Checkout {
        #[arg(short = 'b', long = "branch", help = "Create a new branch")]
        create_new: bool,
    },
    #[command(hide = true)]
    Ignore,
}

fn main() {
    let cli = Cli::parse();
    let config = init::load_config();
    let result = match &cli.command {
        Some(Commands::Commit) => commit::run_commit(
            //TODO: pass only the necessary toml table
            config.commit.conventional_commits,
            config.commit.ticket_suffix,
            config.commit.push_commits,
        ),
        Some(Commands::Branch {
            delete,
            force_delete,
        }) => branch::run_branch(delete.clone(), force_delete.clone()),
        Some(Commands::Checkout { create_new }) => {
            checkout::run_checkout(config.conventional_branches, create_new.clone())
        }
        Some(Commands::Ignore) => {
            println!("Ignore logic to implement later");
            Ok(())
        }
        Some(Commands::Init) => init::run_config(),
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
