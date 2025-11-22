mod deserialize;
mod output;

use std::sync::mpsc;

use clap::{Parser, arg};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    deserialize::{AppInfo, ScoopConfig},
    output::Message,
};

#[derive(clap::Parser)]
#[command(arg_required_else_help = true)]
struct ArgParser {
    query: String,

    #[arg(short = 'p', long)]
    root_path: Option<String>,
}

fn main() {
    let args = ArgParser::parse();
    let query = &args.query;
    let root_path = args.root_path.as_ref();

    let query_copy = query.to_string();
    let (sender, receiver) = mpsc::channel::<Message>();
    let handle = std::thread::spawn(move || output::worker(query_copy, receiver));

    let buckets = ScoopConfig::buckets_path(root_path);
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
