use std::path::Path;

use image::RgbaImage;

pub fn load_and_resize(path: &str, target_width: u32, target_height: u32) -> RgbaImage {
    let path = Path::new(path);

    if !path.exists() {
        eprintln!("Error: file not found: {}", path.display());
        std::process::exit(1);
    }

    let img = image::open(path).unwrap_or_else(|e| {
        eprintln!("Error: failed to load image: {e}");
        std::process::exit(1);
    });

    let img = img.resize(target_width, target_height, image::imageops::FilterType::Lanczos3);

    img.to_rgba8()
}
