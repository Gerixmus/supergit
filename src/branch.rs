use std::{process::Command};

pub fn run_branch(delete: bool) -> Result<(), String> {
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("branch")
        .output()
        .map_err(|e| e.to_string())?;

    let output_str  = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let branches: Vec<String> = output_str .lines().map(|branch| branch.to_owned()).collect();
    
    if delete {      
        Ok(())
    } else {
        branches.iter().for_each(|branch| println!("{branch}"));
        Ok(())
    }
}