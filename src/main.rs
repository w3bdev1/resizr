use std::path::PathBuf;
use clap::Parser;
use image::GenericImageView;

#[derive(Parser, Debug)]
struct Args {
    #[arg(
        short, long,
        value_name = "FILE",
        required = true,
    )]
    image: PathBuf,

    #[arg(
        short, long,
        value_name = "WIDTH",
    )]
    width: Option<i32>,
}

fn main() {
    let args = Args::parse();
    if let Ok(img) = image::open(args.image) {
        if let Some(width) = args.width {
            println!("Width: {width}")
        } else {
            let (width, height) = img.dimensions();
            println!("Width: {width}, Height: {height}");
        }
    } else {
        println!("Image not found");
    }
}
