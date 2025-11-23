use std::path::PathBuf;

use anyhow::{Context, Ok};
use clap::Parser;

use crate::deserialize::{AppManifest, ScoopConfig};

#[derive(Debug, serde::Deserialize, clap::Parser)]
#[command(arg_required_else_help = true)]
struct ArgParser {
    query: String,

    #[arg(short = 'p', long)]
    scoop_root_path: Option<PathBuf>,
}

pub fn search() -> anyhow::Result<Vec<AppInfo>> {
    let args = ArgParser::parse();
    let query = &args.query;

    let scoop_root_path = args
        .scoop_root_path
        .unwrap_or(ScoopConfig::new()?.root_path);
    let scoop_buckets_path = scoop_root_path.join("buckets");

    let mut apps = Vec::with_capacity(50);

    for ele in scoop_buckets_path
        .read_dir()
        .context("Failed to read scoop buckets dir")?
    {
        let bucket = ele?;
        let bucket_name = bucket.file_name().display().to_string();

        for ele in bucket
            .path()
            .join("bucket")
            .read_dir()
            .context(format!("Failed to read bucket `{bucket_name}`"))?
        {
            let app_manifest = ele?;
            if let Some(appname) = app_manifest.path().file_stem() {
                let appname = appname.display().to_string();
                let manifest: AppManifest = serde_json::from_slice(
                    &std::fs::read(app_manifest.path())
                        .context(format!("Failed to read manifest `{appname}`"))?,
                )
                .context(format!("Failed to deserialize `{appname}`"))?;

                if appname.to_lowercase().contains(&query.to_lowercase())
                    || manifest
                        .description
                        .to_lowercase()
                        .contains(&query.to_lowercase())
                {
                    apps.push((appname, bucket_name.clone(), manifest).into());
                }
            }
        }
    }

    Ok(apps)
}

#[derive(Debug)]
pub struct AppInfo {
    pub name: String,
    pub bucket: String,

    pub version: String,
    pub description: String,
    pub homepage: String,
    pub license: String,
    pub notes: Option<String>,
}

impl From<(String, String, AppManifest)> for AppInfo {
    fn from(value: (String, String, AppManifest)) -> Self {
        let (name, bucket, manifest) = value;

        Self {
            name,
            bucket,

            version: manifest.version,
            description: manifest.description,
            homepage: manifest.homepage,
            license: manifest.license.to_string(),
            notes: manifest.notes.map(|notes| notes.to_string()),
        }
    }
}
