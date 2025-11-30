use crate::{error::MyError, manifest::AppManifest};

pub fn search(args: &crate::ArgParser) -> Result<Vec<AppInfo>, MyError> {
    let query = &args.query;

    let scoop_config = crate::config::load();
    let scoop_root_path = args.scoop_root_path.clone().unwrap();
    let scoop_buckets_path = scoop_root_path.join("buckets");

    let mut apps = Vec::with_capacity(50);

    for ele in scoop_buckets_path.read_dir()? {
        let bucket = ele?;
        let bucket_name = bucket.file_name().display().to_string();

        for ele in bucket.path().join("bucket").read_dir()? {
            let app_manifest = ele?;
            if let Some(appname) = app_manifest.path().file_stem() {
                let appname = appname.display().to_string();
                let manifest: AppManifest =
                    serde_json::from_slice(&std::fs::read(app_manifest.path())?)?;

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

#[derive(Debug, Clone)]
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
