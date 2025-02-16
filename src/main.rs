use inquire::{Confirm, MultiSelect, Select, Text};
use git2::{Repository, StatusOptions};
use std::path::Path;

fn get_untracked(repo: &Repository) -> Vec<String> {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);

    let statuses = match repo.statuses(Some(&mut status_opts)) {
        Ok(statuses) => statuses,
        Err(err) => {
            println!("Error fetching statuses: {}", err);
            return Vec::new();
        }
    };

    let mut files_to_add = Vec::new();
    for entry in statuses.iter() {
        let status = entry.status();
        if status.is_wt_new() || status.is_wt_modified() {
            if let Some(path) = entry.path() {
                files_to_add.push(path.to_string());
            }
        }
    }

    files_to_add
}

fn main() {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("This is not a Git repository.");
            return;
        }
    };

    let files_to_add = get_untracked(&repo);

    if files_to_add.is_empty() {
        println!("No untracked or modified files found.");
        return;
    }

    let selected_files = match MultiSelect::new("Select files to add:", files_to_add).prompt() {
        Ok(choices) => choices,
        Err(err) => {
            println!("An error occurred during selection: {}", err);
            return;
        }
    };

    if selected_files.is_empty() {
        println!("No files selected.");
        return;
    }

    let mut index = match repo.index() {
        Ok(index) => index,
        Err(e) => {
            println!("Error accessing index: {}", e);
            return;
        }
    };

    for file in selected_files.iter() {
        let path = Path::new(file);
        if let Err(err) = index.add_path(path) {
            println!("Error adding file '{}': {}", file, err);
        }
    }

    if let Err(e) = index.write() {
        println!("Error writing index: {}", e);
    }

    let options = vec![
        "fix",
        "feat",
        "chore",
        "docs",
        "style",
        "refactor",
        "perf",
        "test",
        "improvement"
    ];

    let selected_option = Select::new("Select your prefix", options).prompt();

    let prefix = match selected_option {
        Ok(choice) => choice,
        Err(err) => {
            println!("An error occurred: {}", err);
            return;
        }
    };

    let user_input = Text::new("Enter commit message:").prompt();

    let message = match user_input {
        Ok(input) => format!("{}: {}", prefix, input),
        Err(err) => {
            println!("An error occurred: {}", err);
            return;
        }
    };

    let signature = repo.signature().unwrap();
    let tree_oid = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_oid).unwrap();

    let head = repo.head();
    let parent_commits = match head {
        Ok(head) => {
            if let Ok(parent) = head.peel_to_commit() {
                vec![parent]
            } else {
                vec![]
            }
        }
        Err(_) => vec![],
    };

    let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();

    let should_commit = Confirm::new(&format!("Commit with message: \"{}\"?", message))
        .with_default(true)
        .prompt();

    match should_commit {
        Ok(true) => {
            println!("Committing...");
    
            repo.commit(
                Some("HEAD"),
                &signature,
                &signature,
                &message,
                &tree,
                &parent_refs,
            ).unwrap();
    
            println!("✅ Commit successful!");
        }
        Ok(false) => {
            println!("❌ Commit canceled.");
        }
        Err(_) => {
            println!("⚠️ Failed to get user confirmation.");
        }
    }
}
