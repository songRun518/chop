use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub root_path: PathBuf,
}

pub fn load() -> Option<Config> {
    let userprofile: PathBuf = std::env::var("userprofile").ok()?.into();
    let config_path = userprofile.join(".config/scoop/config.json");
    serde_json::from_slice(&std::fs::read(&config_path).ok()?).ok()
}
