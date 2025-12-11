use crate::{git_operations, init::Commit};
use inquire::{Confirm, Select, Text};
use regex::Regex;

pub fn run_commit(commit_config: Commit) -> Result<(), String> {
    let repo = git_operations::get_repository().map_err(|e| e.to_string())?;

    let (_changes, staged) = git_operations::get_changes(&repo);

    if staged.is_empty() {
        println!("No staged files found.");
        return Ok(());
    }

    let index = repo
        .index()
        .map_err(|e| format!("Error accessing index: {}", e))?;

    let mut commit_header = if commit_config.conventional_commits {
        let type_and_scope = get_type_and_scope(commit_config.types)
            .map_err(|e| format!("An error occurred: {}", e))
            .map_err(|e| format!("Failed to get confirmation: {}", e))?;
        type_and_scope
    } else {
        String::new()
    };

    let ticket = if commit_config.ticket_suffix {
        let re = Regex::new(r"[A-Z]+-[0-9]+").unwrap();
        let branch = git_operations::get_current_branch().unwrap();
        re.find(&branch)
            .map(|regex_match| format!(" ({})", regex_match.as_str()))
            .unwrap_or_default()
    } else {
        "".to_string()
    };

    let user_input = Text::new("Enter commit message:")
        .prompt()
        .map_err(|e| format!("An error occurred: {}", e))?;

    let body = if commit_config.conventional_commits {
        let mut body_text = Text::new("Body:")
            .prompt()
            .map_err(|e| format!("An error occurred: {}", e))?;
        if !body_text.is_empty() {
            body_text = format!("\n\n{}", body_text);
        };
        body_text
    } else {
        String::new()
    };

    let footer = if commit_config.conventional_commits {
        let is_breaking_change = Confirm::new("BREAKING CHANGE?")
            .with_default(false)
            .prompt()
            .map_err(|e| format!("Failed to get confirmation: {}", e))?;

        let breaking_change = if is_breaking_change {
            let breaking_change_desc = Text::new("Breaking change description:")
                .prompt()
                .map_err(|e| format!("An error occurred: {}", e))?;
            commit_header.push('!');
            format!("\n\nBREAKING CHANGE: {}", breaking_change_desc)
        } else {
            String::new()
        };
        commit_header.push_str(": ");
        breaking_change
    } else {
        String::new()
    };

    let message = format!(
        "{}{}{}{}{}",
        commit_header, user_input, ticket, body, footer
    );

    print_in_box(&message);

    let should_commit = Confirm::new("Commit?")
        .with_default(true)
        .prompt()
        .map_err(|e| format!("Failed to get confirmation: {}", e))?;

    if should_commit {
        git_operations::commit(repo, index, message)
            .map_err(|e| format!("❌ Commit failed: {}", e))?;
        println!("✅ Commit successful!");
    } else {
        println!("❌ Commit canceled or failed to get user confirmation.");
    }

    Ok(())
}

fn print_in_box(message: &str) {
    let lines: Vec<&str> = message.lines().collect();
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    println!("┌{}┐", "─".repeat(max_len + 2));
    for line in lines {
        println!("│ {:width$} │", line, width = max_len);
    }
    println!("└{}┘", "─".repeat(max_len + 2));
}

fn get_type_and_scope(commit_types: Vec<String>) -> Result<String, String> {
    let selected_type = Select::new("Select commit type", commit_types)
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
