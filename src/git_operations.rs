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

pub fn get_branches() -> Result<Vec<BranchInfo>, git2::Error> {
    let repo = get_repository()?;
    if let Err(e) = fetch_with_prune(&repo) {
        eprintln!("Fetch failed: {}", e);
    }
    let branches = repo.branches(Some(git2::BranchType::Local))?;
    let head = repo.head().ok();
    let current_branch = head
        .and_then(|h| h.shorthand().map(|s| s.to_string()));

    let mut branch_list = Vec::new();

    for branch in branches {
        let (branch, _) = branch?;
        let name = branch.name()?
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

pub fn get_repository() -> Result<Repository, git2::Error> {
    Repository::discover(".")
}

pub fn get_changes(repo: &Repository) -> (Vec<Change>, Vec<Change>) {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);

    let statuses = match repo.statuses(Some(&mut status_opts)) {
        Ok(statuses) => statuses,
        Err(err) => {
            println!("Error fetching statuses: {}", err);
            return (Vec::new(), Vec::new());
        }
    };

    let mut untracked = Vec::new();
    let mut staged = Vec::new();

    for entry in statuses.iter() {
        if let Some(path) = entry.path() {
            let path = path.to_string();
            let status = entry.status();

            if status.intersects(Status::WT_NEW | Status::WT_MODIFIED | Status::WT_DELETED) {
                untracked.push(Change { path: path.clone(), status });
            }
            if status.intersects(Status::INDEX_NEW | Status::INDEX_MODIFIED | Status::INDEX_DELETED) {
                staged.push(Change { path, status });
            }
        }
    }

    (untracked, staged)
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
        .map_err(|e| format!("Failed to execute git push command: {}", e))?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(format!("❌ Push failed: {}", error_message));
    }

    Ok(())
}

pub fn commit_and_push(repo: git2::Repository, mut index: git2::Index, message: String) -> Result<(), git2::Error> {
    let signature = repo.signature()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    let parent_commits: Vec<git2::Commit> = repo.head()
        .ok()
        .and_then(|head| head.peel_to_commit().ok().map(|c| vec![c]))
        .unwrap_or_default(); 
    let parent_refs: Vec<&git2::Commit> = parent_commits.iter().collect();
    repo.commit(Some("HEAD"), &signature,&signature, &message,&tree,&parent_refs)?;
    push_to_origin().map_err(|e| git2::Error::from_str(&e))?;
    Ok(())
}

pub fn checkout_branch(branch: &str) -> Result<(), git2::Error>  {
    let repo = get_repository()?;

    let (object, reference) = repo
        .revparse_ext(branch)?;

    repo.checkout_tree(&object, None)?;

    if let Some(reference) = reference {
        let reference_name = reference.name()
            .ok_or_else(|| git2::Error::from_str("Invalid branch reference"))?;
        repo.set_head(reference_name)?;
    } else {
        repo.set_head_detached(object.id())?;
    }
    
    Ok(())
}

pub fn get_current_branch() -> Result<String, git2::Error> {
    let repo = get_repository()?;

    let head = repo.head()?;
    head.shorthand()
    .map(|s| s.to_string())
    .ok_or_else(|| git2::Error::from_str("Failed to get branch name"))
}
