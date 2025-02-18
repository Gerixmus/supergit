use std::{fs, path::PathBuf};
use directories::ProjectDirs;
use inquire::{Confirm, MultiSelect, Select, Text};
use serde::Deserialize;
use crate::git_operations;

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

pub fn run_commit() -> Result<(), String> {
    let repo = git_operations::get_repository()
        .ok_or("Failed to open repository")?;

    let files_to_add = git_operations::get_untracked(&repo);
    if files_to_add.is_empty() {
        println!("No untracked or modified files found.");
        return Ok(());
    }
    let selected_files = MultiSelect::new("Select files to add:", files_to_add)
        .prompt()
        .map_err(|e| format!("An error occurred during selection: {}", e))?;

    if selected_files.is_empty() {
        println!("No files selected.");
        return Ok(());
    }

    let mut index = repo.index().map_err(|e| format!("Error accessing index: {}", e))?;
        
    let config = load_config();
    let commit_type = if config.conventional_commits {
        get_type().map_err(|e| format!("An error occurred: {}", e))?
    } else {
        String::new()
    };

    let user_input = Text::new("Enter commit message:").prompt()
        .map_err(|e| format!("An error occurred: {}", e))?;
    let message = format!("{}{}", commit_type, user_input);

    let should_commit = Confirm::new(&format!("Commit with message: \"{}\"?", message))
        .with_default(true)
        .prompt()
        .map_err(|e| format!("Failed to get confirmation: {}", e))?;

    git_operations::add_files(selected_files, &mut index)
        .map_err(|e| format!("Failed to add files: {}", e))?;
    
    if should_commit {
        git_operations::commit_and_push(repo, index, message)
            .map_err(|e| format!("❌ Commit and push failed: {}", e))?;
        
        println!("✅ Commit and push successful!");
    } else {
        println!("❌ Commit canceled or failed to get user confirmation.");
    }

    Ok(())
}