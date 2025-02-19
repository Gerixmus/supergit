use inquire::Select;

use crate::git_operations;

pub fn run_checkout() -> Result<(), String> {
    let branches = git_operations::get_branches()?;

    let selected_branch = Select::new("Select branch to checkout", branches).prompt();
    
    Ok(())
}