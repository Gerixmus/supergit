use crate::git_operations;

pub fn run_branch() -> Result<(), String> {
    let repo = git_operations::get_repository()
        .ok_or("Failed to open repository")?;
    let branches = repo.branches(Some(git2::BranchType::Local))
        .map_err(|e| format!("Failed to get branches: {}", e))?;

    let head = repo.head().ok();
    let current_branch = head
        .and_then(|h| h.shorthand().map(|s| s.to_string()));

    for branch in branches {
        let (branch, _) = branch.map_err(|e| format!("Error reading branch: {}", e))?;
        let name = branch.name()
            .map_err(|e| format!("Error getting branch name: {}", e))?
            .unwrap_or("Unnamed branch");

        let upstream = match branch.upstream() {
            Ok(_) => "",
            Err(_) => " (no upstream)",
        };

        if Some(name.to_string()) == current_branch {
            println!("* {}{}", name, upstream);
        } else {
            println!("  {}{}", name, upstream);
        }
    }

    Ok(())
}