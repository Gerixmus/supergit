use std::{process::Command};

use inquire::{MultiSelect};

pub fn run_branch(delete: bool) -> Result<(), String> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("branch")
        .output()
        .map_err(|e| e.to_string())?;

    let output_str  = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let branches: Vec<String> = output_str.lines().map(|branch| branch.to_owned()).collect();
    
    if delete {      
        let branches: Vec<String> = branches
            .into_iter()
            .filter(|line| !line.trim().starts_with("*"))
            .map(|branch| branch.trim().to_owned())
            .collect();

        let selected_branches = MultiSelect::new("Select branches to delete", branches)
            .prompt()
            .map_err(|e| format!("Prompt error: {}", e))?;

        let flag = "-d";

        selected_branches.iter().try_for_each(|branch| {
            let status = Command::new("git")
                .arg("branch")
                .arg(flag)
                .arg(branch)
                .status()
                .map_err(|e| e.to_string())?;

            if !status.success() {
                Err(format!("Failed to delete branch: {}", branch))
            } else {
                Ok(())
            }
        })?;
        Ok(())
    } else {
        branches.iter().for_each(|branch| println!("{branch}"));
        Ok(())
    }
}