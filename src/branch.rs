use crate::git_operations;

pub fn run_branch() -> Result<(), String> {
    let repo = git_operations::get_repository()
        .ok_or("Failed to open repository")?;

    let branches = repo.branches(Some(git2::BranchType::Local))
        .map_err(|e| format!("Failed to get branches: {}", e))?;

    let head = repo.head().ok();
    let current_branch = head
        .and_then(|h| h.shorthand().map(|s| s.to_string()));

    for branch_result in branches {
        let (branch, _) = branch_result
            .map_err(|e| format!("Error accessing branch: {}", e))?;

        let name = branch.name()
            .map_err(|e| format!("Failed to get branch name: {}", e))?
            .unwrap_or("Unnamed branch")
            .to_string();

        if Some(name.clone()) == current_branch {
            println!("* {}", name);
        } else {
            println!("  {}", name);
        }
    }

    Ok(())
}