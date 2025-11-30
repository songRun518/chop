use std::fmt::Display;

use serde::Deserialize;

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

impl Display for License {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::String(s) => s,
            Self::Object { identifier: s } => s,
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Notes {
    String(String),
    Array(Vec<String>),
}

impl Display for Notes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::String(s) => s,
            Self::Array(v) => &v.join(""),
        };
        write!(f, "{s}")
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::manifest::AppManifest;

    #[test]
    fn deserialize_all() {
        let v = PathBuf::from("/home/songrun/Documents/scoopRoot/buckets")
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
