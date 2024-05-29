use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = ".")]
    path: String,
    #[clap(short, long, default_value = "rs")]
    ext: String,
}

fn cli() -> (String, String) {
    let args = Cli::parse();
    (args.path, args.ext)
}

fn fmt_file(path: &str) -> String {
    format!(
        "// {}\n\n{}",
        path,
        std::fs::read_to_string(path).unwrap_or_default()
    )
}

fn ext_is(ext: &str, entry: &walkdir::DirEntry) -> bool {
    entry
        .path()
        .extension()
        .map_or(false, |e| e == ext)
}

fn main() {
    let (path, ext) = cli();

    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if ext_is(&ext, &entry) {
            println!("{}", fmt_file(&entry.path().to_string_lossy()));
        }
    }
}