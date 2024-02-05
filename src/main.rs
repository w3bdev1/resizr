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
        requires("output"),
    )]
    width: Option<u32>,

    #[arg(
        short, long,
        value_name = "FILE",
    )]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();
    if let Ok(img) = image::open(args.image) {
        if let Some(width) = args.width {
            let new_height = width * img.height() / img.width();
            let new_img = img.resize(width, new_height, image::imageops::FilterType::CatmullRom);
            let output_path = args.output.unwrap();
            match new_img.save(&output_path) {
                Ok(_) => println!("File saved to: {:?}", output_path),
                Err(msg) => eprintln!("Error while saving at {:?}: {}", output_path, msg),
            }
        } else {
            let (width, height) = img.dimensions();
            println!("Width: {width}, Height: {height}");
        }
    } else {
        eprintln!("Image not found");
    }
}
