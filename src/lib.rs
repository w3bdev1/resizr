use image::DynamicImage;
use std::path::PathBuf;

pub fn resize(img: &DynamicImage, width: u32, height: Option<u32>) -> DynamicImage {
    match height {
        Some(height) => {
            return img.resize_exact(width, height, image::imageops::FilterType::CatmullRom)
        }
        None => {
            let height = width * img.height() / img.width();
            return img.resize(width, height, image::imageops::FilterType::CatmullRom);
        }
    };
}

pub fn save(img: &DynamicImage, output_path: &PathBuf) {
    match img.save(output_path) {
        Ok(_) => println!("File saved to: {:?}", output_path),
        Err(msg) => eprintln!("Error while saving at {:?}: {}", output_path, msg),
    }
}
