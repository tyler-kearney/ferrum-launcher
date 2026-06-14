use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use std::fs;


#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub pinned_apps: Vec<String>,
}

fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME")
        .expect("Could not determine HOME directory");

    PathBuf::from(home)
        .join("Library")
        .join("Application Support")
        .join("Ferrum Launcher")
        .join("config.json")
}

pub fn load_config() -> Config {
    let path = get_config_path();

    match fs::read_to_string(path) {
        Ok(contents) => {
            serde_json::from_str(&contents).unwrap_or_default()
        }
        Err(_) => Config::default(),
    }
}

pub fn save_config(config: &Config) {
    let path = get_config_path();

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .expect("Could not create config directory");
    }

    let json = serde_json::to_string_pretty(config)
        .expect("Could not serialize config");

    let _ = fs::write(path, json);
}