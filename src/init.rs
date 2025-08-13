use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use inquire::MultiSelect;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub push_commits: bool,
    #[serde(default)]
    pub conventional_commits: bool,
    #[serde(default)]
    pub conventional_branches: bool,
    #[serde(default)]
    pub ticket_prefix: bool,
}

impl Config {
    pub fn default() -> Self {
        Config {
            push_commits: false,
            conventional_commits: false,
            conventional_branches: false,
            ticket_prefix: false,
        }
    }
}

fn get_config_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "supergit").expect("Failed to get project directory");

    let directory = proj_dirs.config_dir();
    directory.join("config.toml")
}

pub fn load_config() -> Config {
    let config_path = get_config_path();

    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| toml::to_string(&Config::default()).unwrap());

    toml::from_str(&config_content).expect("Failed to parse config")
}

pub fn run_config() -> Result<(), String> {
    let config_path = get_config_path();
    let config = create_config();
    save_config(&config, &config_path).map_err(|e| format!("Failed to save config: {}", e))?;
    println!("✅ Config created successfuly!");
    Ok(())
}

fn save_config(config: &Config, config_path: &PathBuf) -> io::Result<()> {
    if let Some(directory) = config_path.parent() {
        fs::create_dir_all(directory)?
    };
    let content = toml::to_string(config).unwrap();
    let mut file = fs::File::create(config_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn create_config() -> Config {
    let settings = vec![
        "push_commits",
        "conventional_commits",
        "conventional_branches",
        "ticket_prefix",
    ];

    let selected_options = MultiSelect::new("Select configuration:", settings)
        .prompt()
        .unwrap_or(vec![]);

    Config {
        push_commits: selected_options.contains(&"push_commits"),
        conventional_commits: selected_options.contains(&"conventional_commits"),
        conventional_branches: selected_options.contains(&"conventional_branches"),
        ticket_prefix: selected_options.contains(&"ticket_prefix"),
    }
}
