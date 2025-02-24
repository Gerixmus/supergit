use crate::git_operations;

pub fn run_branch() -> Result<(), String> {
    let branches = git_operations::get_branches()?;

    for branch in &branches {
        println!("{}", branch);
    }

    Ok(())
}