use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use inquire::Confirm;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub commit: Commit,
    pub conventional_branches: bool,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct Commit {
    pub conventional_commits: bool,
    pub ticket_suffix: bool,
}

impl Config {
    fn default() -> Self {
        Config {
            commit: Commit {
                conventional_commits: false,
                ticket_suffix: false,
            },
            conventional_branches: false,
        }
    }
}

pub fn load_config() -> Config {
    let config_path = get_config_path();

    if let Ok(config_content) = fs::read_to_string(&config_path) {
        toml::from_str(&config_content).expect("Failed to parse config")
    } else {
        Config::default()
    }
}

pub fn run_config() -> Result<(), String> {
    let config_path = get_config_path();
    let config = create_config()?;
    save_config(&config, &config_path).map_err(|e| format!("Failed to save config: {}", e))?;
    println!("✅ Config created successfuly!");
    Ok(())
}

fn get_config_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "supergit").expect("Failed to get project directory");

    let directory = proj_dirs.config_dir();
    directory.join("config.toml")
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

struct Setting<'a> {
    label: &'a str,
    get: fn(&Config) -> bool,
    set: fn(&mut Config, bool),
}

fn create_config() -> Result<Config, String> {
    let mut config = Config::default();

    let mut settings = vec![
        Setting {
            label: "Use conventional commits?",
            get: |conf| conf.commit.conventional_commits,
            set: |conf, val| conf.commit.conventional_commits = val,
        },
        Setting {
            label: "Use ticket suffix?",
            get: |conf| conf.commit.ticket_suffix,
            set: |conf, val| conf.commit.ticket_suffix = val,
        },
        Setting {
            label: "Use conventional branches?",
            get: |conf| conf.conventional_branches,
            set: |conf, val| conf.conventional_branches = val,
        },
    ];

    for setting in settings.iter_mut() {
        let answer = Confirm::new(setting.label)
            .with_default((setting.get)(&config))
            .prompt()
            .map_err(|e| format!("An error occurred during selection: {}", e))?;

        (setting.set)(&mut config, answer);
    }

    Ok(config)
}
