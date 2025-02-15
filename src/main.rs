use inquire::MultiSelect;
use git2::{Repository, StatusOptions};
use std::path::Path;

fn main() {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("This is not a Git repository.");
            return;
        }
    };

    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);

    let statuses = match repo.statuses(Some(&mut status_opts)) {
        Ok(statuses) => statuses,
        Err(err) => {
            println!("Error fetching statuses: {}", err);
            return;
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
}
