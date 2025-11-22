use std::path::PathBuf;

use chop::deserialize::Manifest;

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

    for path in v {
        let bytes = std::fs::read(&path).unwrap();
        let manifest = serde_json::from_slice::<Manifest>(&bytes).unwrap_or_else(|err| {
            dbg!(&path);
            panic!("{err}");
        });
    }
}
