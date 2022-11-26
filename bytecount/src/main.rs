use clap::Parser;
use std::fs::read;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    filenames: Vec<PathBuf>,
}

fn main() {
    let args = Args::parse();

    if args.filenames.is_empty() {
        eprintln!("Please, input at least one filename");
        return;
    }

    let result = args.filenames.iter().map(count_bytes).sum::<usize>();

    println!("Total: {}", result);
}

fn count_bytes(filename: &PathBuf) -> usize {
    let data = read(filename);

    match data {
        Ok(data) => data.len(),
        Err(e) => handle_error(e),
    }
}

fn handle_error(e: std::io::Error) -> usize {
    eprintln!("Error: {}", e);

    0
}
