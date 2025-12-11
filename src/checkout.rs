use crate::{git_operations, init::Branch};
use inquire::{Confirm, Select};
use regex::Regex;

pub fn run_checkout(branch_config: Branch, create_new: bool) -> Result<(), String> {
    if create_new {
        let branch_type = if branch_config.conventional_branches {
            let selected_type = Select::new("Select branch type", branch_config.types)
                .prompt()
                .map_err(|e| format!("Prompt error: {}", e))?;
            format!("{}/", selected_type)
        } else {
            "".to_string()
        };

        let branch_input = inquire::Text::new("Enter branch name")
            .prompt()
            .map_err(|e| format!("Prompt error: {}", e))?;

        let re = Regex::new(r" +").unwrap();
        let branch_name = re.replace_all(branch_input.trim(), "-");

        let full_branch = format!("{}{}", branch_type, branch_name);

        let should_checkout =
            Confirm::new(&format!("Create and checkout to: \"{}\"?", full_branch))
                .with_default(true)
                .prompt()
                .map_err(|e| format!("Failed to get confirmation: {}", e))?;

        if should_checkout {
            git_operations::create_and_checkout_branch(&full_branch).map_err(|e| e.to_string())?;
            println!("✅ Created and switched to new branch '{}'", full_branch);
        } else {
            println!("❌ Commit canceled or failed to get user confirmation.");
        }
        Ok(())
    } else {
        let branches = git_operations::get_branches().map_err(|e| e.to_string())?;
        let available_branches: Vec<&git_operations::BranchInfo> = branches
            .iter()
            .filter(|branch| !branch.is_current)
            .collect();

        let selected_branch = Select::new("Select branch to checkout", available_branches)
            .prompt()
            .map_err(|e| format!("Prompt error: {}", e))?;

        git_operations::checkout_branch(&selected_branch.name).map_err(|e| e.to_string())?;

        Ok(())
    }
}
