use crate::git_operations;
use inquire::{Confirm, MultiSelect, Select, Text};
use regex::Regex;

fn print_in_box(message: &str) {
    let border_len = message.len() + 4;
    let horizontal_border = format!("+{}+", "-".repeat(border_len - 2));

    println!("{}", horizontal_border);
    println!("| {:width$} |", message, width = message.len());
    println!("{}", horizontal_border);
}

fn get_type_and_scope() -> Result<String, String> {
    let options = vec![
        "fix",
        "feat",
        "chore",
        "docs",
        "style",
        "refactor",
        "perf",
        "test",
        "improvement",
    ];

    let selected_type = Select::new("Select commit type", options)
        .prompt()
        .map_err(|e| format!("An error occurred: {}", e))?;

    let mut scope = Text::new("Scope:")
        .prompt()
        .map_err(|e| format!("An error occurred: {}", e))?;

    if !scope.is_empty() {
        scope = format!("({})", scope);
    }

    Ok(format!("{}{}", selected_type, scope))
}

pub fn run_commit(
    conventional_commit: bool,
    ticket_prefix: bool,
    push_commits: bool,
) -> Result<(), String> {
    let repo = git_operations::get_repository().map_err(|e| e.to_string())?;

    let (changes, staged) = git_operations::get_changes(&repo);

    if changes.is_empty() && staged.is_empty() {
        println!("No untracked or modified files found.");
        return Ok(());
    }

    let mut selected_files = staged;

    if !changes.is_empty() {
        let selected_unstaged = MultiSelect::new("Select changes to commit:", changes)
            .prompt()
            .map_err(|e| format!("An error occurred during selection: {}", e))?;

        if selected_unstaged.is_empty() && selected_files.is_empty() {
            println!("No files selected.");
            return Ok(());
        }

        selected_files.extend(selected_unstaged);
    }

    let mut index = repo
        .index()
        .map_err(|e| format!("Error accessing index: {}", e))?;

    let commit_type = if conventional_commit {
        let type_and_scope = get_type_and_scope()
            .map_err(|e| format!("An error occurred: {}", e))
            .map_err(|e| format!("Failed to get confirmation: {}", e))?;
        format!("{}: ", type_and_scope)

        // TODO: implement breaking change in body
        // let breaking_change = Confirm::new("BREAKING CHANGE?")
        //     .with_default(false)
        //     .prompt()
        //     .map_err(|e| format!("Failed to get confirmation: {}", e))?;

        // if breaking_change {
        //     scope = format!("{}!", scope);
        // }
    } else {
        String::new()
    };

    let ticket = if ticket_prefix {
        let re = Regex::new(r"[A-Z]+-[0-9]+").unwrap();
        let branch = git_operations::get_current_branch().unwrap();
        re.find(&branch)
            .map(|regex_match| format!(" ({})", regex_match.as_str()))
            .unwrap_or_else(|| "".to_string())
    } else {
        "".to_string()
    };

    let user_input = Text::new("Enter commit message:")
        .prompt()
        .map_err(|e| format!("An error occurred: {}", e))?;
    let message = format!("{}{}{}", commit_type, user_input, ticket);

    print_in_box(&message);

    let should_commit = Confirm::new("Commit?")
        .with_default(true)
        .prompt()
        .map_err(|e| format!("Failed to get confirmation: {}", e))?;

    if should_commit {
        git_operations::add_files(selected_files, &mut index)
            .map_err(|e| format!("Failed to add files: {}", e))?;
        git_operations::commit_and_push(repo, index, message, push_commits)
            .map_err(|e| format!("❌ Commit and push failed: {}", e))?;
        if push_commits {
            println!("✅ Commit and push successful!");
        } else {
            println!("✅ Commit successful!");
        }
    } else {
        println!("❌ Commit canceled or failed to get user confirmation.");
    }

    Ok(())
}
