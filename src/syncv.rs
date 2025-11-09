use std::io::Write;
use std::path::{Path, PathBuf};

use colored::Colorize;
use serde::Deserialize;

pub fn syncv() {
    print!("Search an app: ");
    std::io::stdout().flush().unwrap();

    let mut query = String::new();
    std::io::stdin().read_line(&mut query).unwrap();
    let query = query.trim();

    let buckets = buckets_path();
    for bucket in buckets.read_dir().unwrap() {
        let bucket = bucket.unwrap();
        let bucket_name = bucket.file_name();
        let bucket_name = bucket_name.to_str().unwrap();

        let apps = std::fs::read_dir(bucket.path().join("bucket"))
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect::<Vec<_>>();
        apps.into_iter().for_each(|app| {
            let appname = app.file_name();
            let appname = appname.to_str().unwrap();
            let appname = &appname[..appname.len() - ".json".len()];

            if appname.contains(query) {
                let appinfo = appinfo(app.path());

                // U+2500	─
                // U+2502	│
                // U+250C	┌
                // U+2510	┐
                // U+2514	└
                // U+2518	┘

                let background = |s: &str| {
                    if let Some(i) = s.to_lowercase().find(query) {
                        let j = i + query.chars().count();
                        format!(
                            "{}{}{}",
                            &s[..i],
                            s[i..j].bold().on_yellow().black(),
                            &s[j..]
                        )
                    } else {
                        s.to_string()
                    }
                };

                let width = crossterm::terminal::size().unwrap().0 as usize;
                println!("┌{}┐", "─".repeat(width - 2));
                println!("  {} in {}", background(appname), bucket_name.blue());
                println!("  description: {}", background(&appinfo.description));
                println!("  version: {}", appinfo.version.cyan());
                println!("  homepage: {}", appinfo.homepage.purple());
                println!("  license: {}", appinfo.license.green());
                if let Some(notes) = appinfo.notes {
                    println!("  notes: {}", notes);
                }
                println!("└{}┘", "─".repeat(width - 2));
            }
        });
    }
}

#[derive(Debug, Deserialize)]
struct ScoopConfig {
    root_path: String,
}

fn buckets_path() -> PathBuf {
    let userprofile: PathBuf = std::env::var("userprofile").unwrap().into();
    let config_file_path = userprofile.join(".config/scoop/config.json");
    let config_buf = std::fs::read(&config_file_path).unwrap();
    let root_path: PathBuf = serde_json::from_slice::<ScoopConfig>(&config_buf)
        .unwrap()
        .root_path
        .into();
    root_path.join("buckets")
}

#[derive(Debug, Default)]
struct AppInfo {
    version: String,
    description: String,
    homepage: String,
    license: String,
    notes: Option<String>,
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

fn appinfo(path: impl AsRef<Path>) -> AppInfo {
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
