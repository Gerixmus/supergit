use std::{fs, path::PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub conventional_commits: bool,
    pub ticket_prefix: bool
}

impl Config {
    pub fn default() -> Self {
        Config {
            conventional_commits: false,
            ticket_prefix: false
        }
    }
}

fn get_config_path() -> PathBuf {
    let proj_dirs  = ProjectDirs::from("", "", "cmt")
        .expect("Failed to get project directory");

    let directory = proj_dirs.config_dir();
    directory.join("config.toml")
}

pub fn load_config() -> Config {
    let config_path = get_config_path();

    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| toml::to_string(&Config::default()).unwrap());

    toml::from_str(&config_content).expect("Failed to parse config")
}