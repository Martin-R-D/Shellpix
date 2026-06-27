use image::RgbaImage;

use crate::color::{CUBE_VALUES, nearest_cube_index};

fn quantize_channel_ansi256(v: u8) -> u8 {
    CUBE_VALUES[nearest_cube_index(v)]
}

fn quantize_channel_truecolor(v: u8) -> u8 {
    (v / 8) * 8
}

pub fn floyd_steinberg(image: &mut RgbaImage, palette_size: u32) {
    let width = image.width() as i32;
    let height = image.height() as i32;

    let mut buf: Vec<Vec<[f32; 3]>> = (0..height)
        .map(|y| {
            (0..width)
                .map(|x| {
                    let p = image.get_pixel(x as u32, y as u32);
                    [p[0] as f32, p[1] as f32, p[2] as f32]
                })
                .collect()
        })
        .collect();

    let quantize: fn(u8) -> u8 = if palette_size <= 256 {
        quantize_channel_ansi256
    } else {
        quantize_channel_truecolor
    };

    for y in 0..height {
        for x in 0..width {
            let old = buf[y as usize][x as usize];

            let new = [
                quantize(old[0].clamp(0.0, 255.0) as u8),
                quantize(old[1].clamp(0.0, 255.0) as u8),
                quantize(old[2].clamp(0.0, 255.0) as u8),
            ];

            buf[y as usize][x as usize] = [new[0] as f32, new[1] as f32, new[2] as f32];

            let err = [
                old[0] - new[0] as f32,
                old[1] - new[1] as f32,
                old[2] - new[2] as f32,
            ];

            let neighbors: [(i32, i32, f32); 4] = [
                (x + 1, y, 7.0 / 16.0),
                (x - 1, y + 1, 3.0 / 16.0),
                (x, y + 1, 5.0 / 16.0),
                (x + 1, y + 1, 1.0 / 16.0),
            ];

            for (nx, ny, weight) in neighbors {
                if nx >= 0 && nx < width && ny >= 0 && ny < height {
                    let pixel = &mut buf[ny as usize][nx as usize];
                    pixel[0] += err[0] * weight;
                    pixel[1] += err[1] * weight;
                    pixel[2] += err[2] * weight;
                }
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let c = buf[y as usize][x as usize];
            let a = image.get_pixel(x as u32, y as u32)[3];
            image.put_pixel(
                x as u32,
                y as u32,
                image::Rgba([
                    c[0].clamp(0.0, 255.0) as u8,
                    c[1].clamp(0.0, 255.0) as u8,
                    c[2].clamp(0.0, 255.0) as u8,
                    a,
                ]),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_color_unchanged() {
        let mut img = RgbaImage::new(4, 4);
        for p in img.pixels_mut() {
            *p = image::Rgba([0x87, 0xaf, 0xd7, 255]);
        }
        floyd_steinberg(&mut img, 256);
        for p in img.pixels() {
            assert_eq!(p[0], 0x87);
            assert_eq!(p[1], 0xaf);
            assert_eq!(p[2], 0xd7);
        }
    }

    #[test]
    fn test_dither_does_not_panic_on_1x1() {
        let mut img = RgbaImage::new(1, 1);
        img.put_pixel(0, 0, image::Rgba([100, 150, 200, 255]));
        floyd_steinberg(&mut img, 256);
    }

    #[test]
    fn test_gradient_produces_variation() {
        let mut img = RgbaImage::new(10, 1);
        for x in 0..10 {
            let v = (x * 25) as u8;
            img.put_pixel(x, 0, image::Rgba([v, v, v, 255]));
        }
        floyd_steinberg(&mut img, 256);
        let mut values = std::collections::HashSet::new();
        for x in 0..10 {
            values.insert(img.get_pixel(x, 0)[0]);
        }
        assert!(values.len() >= 2);
    }
}
