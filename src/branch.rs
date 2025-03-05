use crate::git_operations;

pub fn run_branch() -> Result<(), String> {
    let branches = git_operations::get_branches().map_err(|e| e.to_string())?;

    for branch in &branches {
        println!("{}", branch);
    }

    Ok(())
}