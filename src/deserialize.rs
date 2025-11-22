use std::path::PathBuf;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScoopConfig {
    pub root_path: PathBuf,
}
impl ScoopConfig {
    pub fn new() -> anyhow::Result<Self> {
        let userprofile: PathBuf = std::env::var("userprofile")
            .context("Failed to get `userprofile` env variable")?
            .into();
        let config_path = userprofile.join(".config/scoop/config.json");
        serde_json::from_slice(&std::fs::read(&config_path).context("Failed to read scoop config")?)
            .context("Failed to deserialize scoop config")
    }
}

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub license: License,

    pub notes: Option<Notes>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum License {
    String(String),
    Object {
        identifier: String,
        url: Option<String>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Notes {
    String(String),
    Array(Vec<String>),
}
