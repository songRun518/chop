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
pub struct AppManifest {
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
    Object { identifier: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Notes {
    String(String),
    Array(Vec<String>),
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::deserialize::AppManifest;

    #[test]
    fn deserialize_all() {
        let v = PathBuf::from("D:/home/apps/scoop/buckets")
            .read_dir()
            .unwrap()
            .map(|re| re.unwrap())
            .flat_map(|bucket| {
                bucket
                    .path()
                    .join("bucket")
                    .read_dir()
                    .unwrap()
                    .map(|re| re.unwrap().path())
            })
            .collect::<Vec<_>>();

        for path in v {
            let bytes = std::fs::read(&path).unwrap();
            serde_json::from_slice::<AppManifest>(&bytes).unwrap_or_else(|err| {
                dbg!(&path);
                panic!("{err}");
            });
        }
    }
}
