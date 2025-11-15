use clap::arg;

fn main() {
    let args = clap::command!()
        .args([arg!(<query> "Slice of application name")])
        .get_matches();
    let query = args.get_one::<String>("query").unwrap();
}
