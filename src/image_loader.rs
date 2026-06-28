use std::io::Read;
use std::path::Path;
use std::time::Duration;

use image::RgbaImage;

fn load_from_file(path: &str) -> image::DynamicImage {
    let path = Path::new(path);

    if !path.exists() {
        eprintln!("Error: file not found: {}", path.display());
        std::process::exit(1);
    }

    image::open(path).unwrap_or_else(|e| {
        eprintln!("Error: invalid image file: {e}");
        std::process::exit(1);
    })
}

fn load_from_url(url: &str) -> image::DynamicImage {
    eprintln!("Downloading...");

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Error: failed to create HTTP client: {e}");
            std::process::exit(1);
        });

    let response = client.get(url).send().unwrap_or_else(|e| {
        eprintln!("Error: network request failed: {e}");
        std::process::exit(1);
    });

    if !response.status().is_success() {
        eprintln!("Error: HTTP {}", response.status());
        std::process::exit(1);
    }

    let bytes = response.bytes().unwrap_or_else(|e| {
        eprintln!("Error: failed to read response body: {e}");
        std::process::exit(1);
    });

    image::load_from_memory(&bytes).unwrap_or_else(|e| {
        eprintln!("Error: downloaded data is not a valid image: {e}");
        std::process::exit(1);
    })
}

fn load_from_stdin() -> image::DynamicImage {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).unwrap_or_else(|e| {
        eprintln!("Error: failed to read from stdin: {e}");
        std::process::exit(1);
    });

    image::load_from_memory(&buf).unwrap_or_else(|e| {
        eprintln!("Error: stdin data is not a valid image: {e}");
        std::process::exit(1);
    })
}

pub fn load_and_resize(input: &str, target_width: u32, target_height: u32) -> RgbaImage {
    let img = if input == "-" {
        load_from_stdin()
    } else if input.starts_with("http://") || input.starts_with("https://") {
        load_from_url(input)
    } else {
        load_from_file(input)
    };

    let img = img.resize(target_width, target_height, image::imageops::FilterType::Lanczos3);
    img.to_rgba8()
}
