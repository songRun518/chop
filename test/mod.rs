use std::path::PathBuf;

#[test]
fn deserialize() {
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

    for p in v {
        println!("{}", p.display());
    }
}
