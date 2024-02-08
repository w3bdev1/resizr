use std::path::PathBuf;
use clap::{ArgAction, Parser};
use image::GenericImageView;
use resizr::{resize, save};

#[derive(Parser, Debug)]
#[command(disable_help_flag = true)]
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
        requires("output"),
    )]
    width: Option<u32>,

    #[arg(
        short, long,
        value_name = "HEIGHT",
        requires("width"),
    )]
    height: Option<u32>,

    #[arg(
        short, long,
        value_name = "FILE",
    )]
    output: Option<PathBuf>,

    #[arg(long, action = ArgAction::HelpLong)]
    help: Option<u8>,
}

fn main() {
    let args = Args::parse();
    if let Ok(img) = image::open(args.image) {
        if let Some(width) = args.width {
            let new_img = resize(&img, width, args.height);
            let output_path = args.output.unwrap();
            save(&new_img, &output_path)
        } else {
            let (width, height) = img.dimensions();
            println!("Width: {width}, Height: {height}");
        }
    } else {
        eprintln!("Image not found");
    }
}

