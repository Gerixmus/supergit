use directories::ProjectDirs;
use inquire::{Confirm, MultiSelect, Select, Text};
use std::{fs, path::PathBuf};
use serde::Deserialize;

mod git_operations;

#[derive(Deserialize)]
struct Config {
    conventional_commits: bool,
}

fn get_config_path() -> PathBuf {
    let proj_dirs  = ProjectDirs::from("", "", "cmt")
        .expect("Failed to get project directory");

    let directory = proj_dirs.config_dir();
    directory.join("config.toml")
}

fn load_config() -> Config {
    let config_path = get_config_path();

    let config_content = fs::read_to_string(config_path)
        .unwrap_or_else(|_| "conventional_commits = false".to_string());

    toml::from_str(&config_content).expect("Failed to parse config")
}

fn get_type() -> Result<String, String> {
    let options = vec![
        "fix",
        "feat",
        "chore",
        "docs",
        "style",
        "refactor",
        "perf",
        "test",
        "improvement"
    ];

    let selected_option = Select::new("Select commit type", options).prompt();

    match selected_option {
        Ok(choice) => Ok(format!("{}: ", choice)),
        Err(err) => Err(format!("An error occurred: {}", err))
    }
}

fn main() {
    let repo = match git_operations::get_repository() {
        Some(value) => value,
        None => return,
    };

    let files_to_add = git_operations::get_untracked(&repo);

    if files_to_add.is_empty() {
        println!("No untracked or modified files found.");
        return;
    }

    let selected_files = match MultiSelect::new("Select files to add:", files_to_add).prompt() {
        Ok(choices) => choices,
        Err(err) => {
            println!("An error occurred during selection: {}", err);
            return;
        }
    };

    if selected_files.is_empty() {
        println!("No files selected.");
        return;
    }

    let mut index = match repo.index() {
        Ok(index) => index,
        Err(e) => {
            println!("Error accessing index: {}", e);
            return;
        }
    };

    let config = load_config();
    let mut commit_type = String::new();

    if config.conventional_commits {
        commit_type = match get_type() {
            Ok(choice) => choice,
            Err(err) => {
                println!("An error occurred: {}", err);
                return;
            }
        };
    }

    let user_input = Text::new("Enter commit message:").prompt();

    let message = match user_input {
        Ok(input) => format!("{}{}", commit_type, input),
        Err(err) => {
            println!("An error occurred: {}", err);
            return;
        }
    };

    let should_commit = Confirm::new(&format!("Commit with message: \"{}\"?", message))
        .with_default(true)
        .prompt();

    if let Err(err) = git_operations::add_files(selected_files, &mut index) {
        eprintln!("Failed to add files: {}", err);
        return;
    }

    if let Ok(true) = should_commit {
        match git_operations::commit_and_push(repo, index, message) {
            Ok(()) => {
                println!("✅ Commit and push successful!");
            }
            Err(err) => {
                println!("{}", err);
            }
        }
        return;
    } else {
    println!("❌ Commit canceled or failed to get user confirmation.");
}
}
