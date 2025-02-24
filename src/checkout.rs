use inquire::Select;

use crate::git_operations;

pub fn run_checkout() -> Result<(), String> {
    let branches = git_operations::get_branches()?;
    let available_branches: Vec<&git_operations::BranchInfo> = branches
        .iter()
        .filter(|branch| !branch.is_current)
        .collect();

    let selected_branch = Select::new("Select branch to checkout", available_branches)
        .prompt()
        .map_err(|e| format!("Prompt error: {}", e))?;

    git_operations::checkout_branch(&selected_branch.name)?;
    
    Ok(())
}