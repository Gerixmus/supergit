use crate::git_operations;

pub fn run_branch() -> Result<(), String> {
    let branches = git_operations::get_branches()?;

    branches.iter().for_each(|branch| {println!("{}", branch)});

    Ok(())
}