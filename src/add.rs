use crate::git_operations::{self, Change};
use inquire::MultiSelect;

pub fn stage_files() -> Result<(), String> {
    let repo = git_operations::get_repository().map_err(|e| e.to_string())?;

    let (changes, _staged) = git_operations::get_changes(&repo);

    if changes.is_empty() {
        println!("No untracked or modified files found.");
        return Ok(());
    }

    let mut selected_files = Vec::<Change>::new();

    let selected_unstaged = MultiSelect::new("Select changes to commit:", changes)
        .prompt()
        .map_err(|e| format!("An error occurred during selection: {}", e))?;

    if selected_unstaged.is_empty() && selected_files.is_empty() {
        println!("No files selected.");
        return Ok(());
    }

    selected_files.extend(selected_unstaged);

    let mut index = repo
        .index()
        .map_err(|e| format!("Error accessing index: {}", e))?;

    git_operations::add_files(selected_files, &mut index)
        .map_err(|e| format!("Failed to add files: {}", e))?;

    println!("✅ Added files successfuly!");
    Ok(())
}