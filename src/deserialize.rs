use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScoopConfig {
    root_path: PathBuf,
}

impl ScoopConfig {
    pub fn buckets_path(root_path: Option<&String>) -> PathBuf {
        if let Some(root_path) = root_path {
            return PathBuf::from(root_path).join("buckets");
        }

        let userprofile: PathBuf = std::env::var("userprofile")
            .expect("Please give scoop root path by `-p`")
            .into();
        let config_file_path = userprofile.join(".config/scoop/config.json");
        let config_buf = std::fs::read(&config_file_path).unwrap();
        let config = serde_json::from_slice::<ScoopConfig>(&config_buf).unwrap();
        config.root_path.join("buckets")
    }
}

#[derive(Debug, Deserialize)]
struct Manifest {
    version: String,
    description: String,
    homepage: String,
    license: License,
    notes: Option<Notes>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum License {
    String(String),
    Object { identifier: String },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Notes {
    String(String),
    Array(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppInfo {
    pub version: String,
    pub description: String,
    pub homepage: String,
    pub license: String,
    pub notes: Option<String>,
}

impl AppInfo {
    pub fn new(path: impl AsRef<Path>) -> AppInfo {
        let buf = std::fs::read(path).unwrap();
        let manifest = serde_json::from_slice::<Manifest>(&buf).unwrap();
        let license = match manifest.license {
            License::String(s) => s,
            License::Object { identifier: s } => s,
        };
        let notes = manifest.notes.map(|notes| match notes {
            Notes::String(s) => s,
            Notes::Array(arr) => arr.join("\n         "), // Align to "  note: "
        });

        AppInfo {
            version: manifest.version,
            description: manifest.description,
            homepage: manifest.homepage,
            license,
            notes,
        }
    }
}
