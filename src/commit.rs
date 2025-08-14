use crate::git_operations;
use inquire::{Confirm, MultiSelect, Select, Text};
use regex::Regex;

fn print_in_box(message: &str) {
    let lines: Vec<&str> = message.lines().collect();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    println!("┌{}┐", "─".repeat(max_len + 2));
    for line in lines {
        println!("│ {:width$} │", line, width = max_len);
    }
    println!("└{}┘", "─".repeat(max_len + 2));
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
    is_conventional_commit: bool,
    ticket_suffix: bool,
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

    let mut commit_header = if is_conventional_commit {
        let type_and_scope = get_type_and_scope()
            .map_err(|e| format!("An error occurred: {}", e))
            .map_err(|e| format!("Failed to get confirmation: {}", e))?;
        type_and_scope
    } else {
        String::new()
    };

    let ticket = if ticket_suffix {
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

    let footer = if is_conventional_commit {
        let is_breaking_change = Confirm::new("BREAKING CHANGE?")
            .with_default(false)
            .prompt()
            .map_err(|e| format!("Failed to get confirmation: {}", e))?;

        let breaking_change = if is_breaking_change {
            let breaking_change_desc = Text::new("Breaking change description:")
                .prompt()
                .map_err(|e| format!("An error occurred: {}", e))?;
            commit_header.push_str("!");
            format!("\nBREAKING CHANGE: {}", breaking_change_desc)
        } else {
            String::new()
        };
        commit_header.push_str(": ");
        breaking_change
    } else {
        String::new()
    };

    let message = format!("{}{}{}{}", commit_header, user_input, ticket, footer);

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
