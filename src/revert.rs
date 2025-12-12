use std::{fmt, process::Command};

use inquire::Select;

use crate::git_operations;

struct Commit{
    hash: String,
    message: String
}

impl fmt::Display for Commit{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub fn run_revert() -> Result<(), String> {
    let output = Command::new("git").arg("log").arg("--pretty=format:%H%x00%s").output().unwrap();
    let commits = String::from_utf8_lossy(&output.stdout).lines().map(|s| {
        let data: Vec<&str> = s.split('\0').collect();
        Commit {
            hash: data[0].to_string(),
            message: data[1].to_string()
        }
    }).collect();

    let selected_commit = Select::new("Select commit to revert:", commits).prompt().unwrap();
    let mut binding = Command::new("git");
    let result = binding.arg("revert").arg("--no-commit").arg(&selected_commit.hash).arg("-m");
    println!("{:?}", result);
    println!("revert: \"{}\"",selected_commit.message);
    println!("This reverts commit: {}",selected_commit.hash);

    
    let repo = git_operations::get_repository().map_err(|e| e.to_string())?;
    let index = repo
    .index()
    .map_err(|e| format!("Error accessing index: {}", e))?;

    git_operations::commit(repo, index, message)
    .map_err(|e| format!("❌ Commit failed: {}", e))?;

    Ok(())
}