use walkdir::WalkDir;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Path to search for files
    #[clap(short, long, default_value = ".")]
    path: String,
    /// File extension to search for
    #[clap(short, long, default_value = "rs")]
    ext: String,
    /// Whether to include the LLM prompt
    #[clap(long, default_value = "false")]
    prompt: bool,
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
    let args = Cli::parse();

    if args.prompt {
        println!("{}\n```{}\n", include_str!("prompt.txt"), args.ext);
    }

    for entry in WalkDir::new(&args.path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if ext_is(&args.ext, &entry) {
            println!("{}", fmt_file(&entry.path().to_string_lossy()));
        }
    }

    if args.prompt {
        println!("\n```");
    }
}