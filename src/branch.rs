use std::process::Command;

pub fn run_branch() -> Result<(), String> {
    let status = Command::new("git")
        .arg("--no-pager")
        .arg("branch")
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(())
    } else {
        Err("git branch command failed".to_string())
    }
}