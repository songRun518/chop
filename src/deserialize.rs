use serde::Deserialize;

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
