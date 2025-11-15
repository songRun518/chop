use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver};

use clap::arg;
use colored::Colorize;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;

fn main() {
    let args = clap::command!()
        .args([arg!(<query> "Slice of application name")])
        .get_matches();
    let query = args.get_one::<String>("query").unwrap();

    let query_copy = query.to_string();
    let (sender, receiver) = mpsc::channel::<Message>();
    let handle = std::thread::spawn(move || output(query_copy, receiver));

    let buckets = ScoopConfig::buckets_path();
    for bucket in buckets.read_dir().unwrap() {
        let bucket = bucket.unwrap();
        let bucket_name = bucket.file_name();
        let bucket_name = bucket_name.to_str().unwrap();

        let apps = std::fs::read_dir(bucket.path().join("bucket"))
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect::<Vec<_>>();
        apps.into_par_iter()
            .for_each_with(sender.clone(), |sender, app| {
                let appname = app.file_name();
                let appname = appname.to_str().unwrap();
                let appname = &appname[..appname.len() - ".json".len()];

                if appname.contains(query) {
                    let appinfo = AppInfo::new(app.path());
                    sender
                        .send(Message::Info {
                            appname: appname.to_string(),
                            bucket_name: bucket_name.to_string(),
                            details: appinfo,
                        })
                        .unwrap();
                }
            });
    }

    sender.send(Message::Close).unwrap();
    handle.join().unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Message {
    Info {
        appname: String,
        bucket_name: String,
        details: AppInfo,
    },
    Close,
}

fn output(query: String, receiver: Receiver<Message>) {
    let stdout = std::io::stdout();
    let mut buf_stdout = BufWriter::new(stdout.lock());

    while let Ok(message) = receiver.recv() {
        if let Message::Info {
            appname,
            bucket_name,
            details: detail,
        } = message
        {
            // U+2500	─
            // U+2502	│
            // U+250C	┌
            // U+2510	┐
            // U+2514	└
            // U+2518	┘

            let backcolor = |s: &str| {
                if let Some(i) = s.to_lowercase().find(&query) {
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
            writeln!(&mut buf_stdout, "┌{}┐", "─".repeat(width - 2)).unwrap();
            writeln!(
                &mut buf_stdout,
                "  {} in {}",
                backcolor(&appname),
                bucket_name.blue()
            )
            .unwrap();
            writeln!(
                &mut buf_stdout,
                "  description: {}",
                backcolor(&detail.description)
            )
            .unwrap();
            writeln!(&mut buf_stdout, "  version: {}", detail.version.cyan()).unwrap();
            writeln!(&mut buf_stdout, "  homepage: {}", detail.homepage.purple()).unwrap();
            writeln!(&mut buf_stdout, "  license: {}", detail.license.green()).unwrap();
            if let Some(notes) = detail.notes {
                writeln!(&mut buf_stdout, "  notes: {}", notes).unwrap();
            }
            writeln!(&mut buf_stdout, "└{}┘", "─".repeat(width - 2)).unwrap();
        } else {
            break;
        }
    }

    buf_stdout.flush().unwrap();
}

#[derive(Debug, Deserialize)]
struct ScoopConfig {
    root_path: PathBuf,
}

impl ScoopConfig {
    fn buckets_path() -> PathBuf {
        let userprofile: PathBuf = std::env::var("userprofile").unwrap().into();
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
struct AppInfo {
    version: String,
    description: String,
    homepage: String,
    license: String,
    notes: Option<String>,
}

impl AppInfo {
    fn new(path: impl AsRef<Path>) -> AppInfo {
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
