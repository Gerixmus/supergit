use core::fmt;
use std::{path::Path, process::Command};
use colored::Colorize;
use git2::{FetchOptions, FetchPrune, Repository, Status, StatusOptions};

#[derive(Clone)]
pub struct Change {
    pub path: String,
    status: git2::Status
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self.status {
            s if s.contains(Status::WT_NEW) => "new",
            s if s.contains(Status::WT_MODIFIED) => "modified",
            s if s.contains(Status::WT_DELETED) => "deleted",
            _ => "?",
        };
        write!(f, "{}: {}", status_str, self.path)
    }
}

pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub upstream: bool
}

impl fmt::Display for BranchInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let current_marker = if self.is_current { "* " } else { "  " };
        let upstream_marker = if self.upstream { String::new() } else { " (no upstream)".red().to_string() };
        let branch_name = if self.is_current {
            self.name.green()
        } else {
            self.name.normal()
        };
        write!(f, "{}{}{}", current_marker, branch_name, upstream_marker)
    }
}

pub fn get_branches() -> Result<Vec<BranchInfo>, String> {
    let repo = get_repository()
        .ok_or("Failed to open repository")?;
    if let Err(e) = fetch_with_prune(&repo) {
        eprintln!("Fetch failed: {}", e);
    }
    let branches = repo.branches(Some(git2::BranchType::Local))
        .map_err(|e| format!("Failed to get branches: {}", e))?;
    let head = repo.head().ok();
    let current_branch = head
        .and_then(|h| h.shorthand().map(|s| s.to_string()));

    let mut branch_list = Vec::new();

    for branch in branches {
        let (branch, _) = branch.map_err(|e| format!("Error reading branch: {}", e))?;
        let name = branch.name()
            .map_err(|e| format!("Error getting branch name: {}", e))?
            .unwrap_or("Unnamed branch")
            .to_string();
        let upstream = branch.upstream().is_ok();

        branch_list.push(BranchInfo{
            is_current: Some(name.clone()) == current_branch,
            name,
            upstream
        });
    }

    Ok(branch_list)
}

fn fetch_with_prune(repo: &Repository) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote("origin")?;

    let mut fetch_options = FetchOptions::new();
    fetch_options.prune(FetchPrune::On);

    remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_options), None)?;
    
    Ok(())
}

pub fn get_repository() -> Option<Repository> {
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("This is not a Git repository.");
            return None;
        }
    };
    Some(repo)
}

pub fn get_untracked(repo: &Repository) -> Vec<Change> {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);

    let statuses = match repo.statuses(Some(&mut status_opts)) {
        Ok(statuses) => statuses,
        Err(err) => {
            println!("Error fetching statuses: {}", err);
            return Vec::new();
        }
    };

    let mut changes= Vec::new();
    for entry in statuses.iter() {
        let status = entry.status();
        if status.intersects(Status::WT_NEW | Status::WT_MODIFIED | Status::WT_DELETED) {
            if let Some(path) = entry.path() {
                let path = path.to_string();
                changes.push(Change {
                    path,
                    status
                });
            }
        }
    }

    changes
}

pub fn add_files(selected_files: Vec<Change>, index: &mut git2::Index) -> Result<(), git2::Error>{
    for change in selected_files.iter() {
        let path = Path::new(&change.path);
        if change.status == Status::WT_DELETED {
            index.remove_path(path).map_err(|err| err)?;
        } else {
            index.add_path(path).map_err(|err| err)?;
        }
    }
    
    index.write()?;
    Ok(())
}

pub fn push_to_origin() -> Result<(), String> {
    let output = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("HEAD") 
        .output()
        .expect("Failed to execute git push command");

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("{}", error_message));
    }

    Ok(())
}

pub fn commit_and_push(repo: git2::Repository, mut index: git2::Index, message: String) -> Result<(), String> {
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
    if let Err(err) = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &parent_refs,
    ) {
        return Err(format!("❌ Commit failed: {}", err));
    }
    if let Err(err) = push_to_origin() {
        return Err(format!("❌ Push failed: {}", err));
    }
    
    Ok(())
}

pub fn checkout_branch(branch: &str) -> Result<(), String>  {
    let repo = get_repository().ok_or("Failed to open repository")?;

    let (object, reference) = repo
        .revparse_ext(branch)
        .map_err(|e| format!("Failed to find branch: {}", e))?;

    repo.checkout_tree(&object, None)
        .map_err(|e| format!("Failed to checkout tree: {}", e))?;

    if let Some(reference) = reference {
        repo.set_head(reference.name().ok_or("Invalid branch reference")?)
            .map_err(|e| format!("Failed to set HEAD: {}", e))?;
    } else {
        repo.set_head_detached(object.id())
            .map_err(|e| format!("Failed to set HEAD detached: {}", e))?;
    }
    
    Ok(())
}

pub fn get_current_branch() -> Result<String, String> {
    let repo = get_repository().ok_or("Failed to open repository")?;

    let head = repo.head().map_err(|e| format!("Failed to get HEAD: {}", e))?;

    head.shorthand()
    .map(|s| s.to_string())
    .ok_or_else(|| "Failed to get branch name".to_string())
}
