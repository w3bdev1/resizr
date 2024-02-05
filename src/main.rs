use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    image: PathBuf
}

fn main() {
    let args = Args::parse();
    let img_path = args.image;
    println!("{:?}", img_path);
}
