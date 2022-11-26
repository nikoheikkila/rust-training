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
        println!("Please, input at least one filename");
        return;
    }

    let mut len = 0;

    for filename in args.filenames {
        let data = read(&filename).unwrap();
        len += data.len();

        println!("{}: {} bytes", filename.display(), data.len());
    }

    println!("total: {}", len);
}
