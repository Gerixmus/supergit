use std::process::Command;

use inquire::{Confirm, MultiSelect};

pub fn run_branch(delete: bool, force_delete: bool) -> Result<(), String> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("branch")
        .output()
        .map_err(|e| e.to_string())?;

    let output_str = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let branches: Vec<String> = output_str.lines().map(|branch| branch.to_owned()).collect();

    if delete || force_delete {
        let branches: Vec<String> = branches
            .into_iter()
            .filter(|line| !line.trim().starts_with("*"))
            .map(|branch| branch.trim().to_owned())
            .collect();

        let selected_branches = MultiSelect::new("Select branches to delete", branches)
            .prompt()
            .map_err(|e| format!("Prompt error: {}", e))?;

        let flag = if delete { "-d" } else { "-D" };

        let should_delete = Confirm::new("Delete selected branches?")
            .with_default(true)
            .prompt()
            .map_err(|e| format!("Failed to get confirmation: {}", e))?;

        if should_delete {
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
        } else {
            println!("❌ Commit canceled or failed to get user confirmation.");
        }

        Ok(())
    } else {
        branches.iter().for_each(|branch| println!("{branch}"));
        Ok(())
    }
}
