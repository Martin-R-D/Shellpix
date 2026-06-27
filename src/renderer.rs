use image::RgbaImage;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let alpha = a as f32 / 255.0;
        Self {
            r: (r as f32 * alpha) as u8,
            g: (g as f32 * alpha) as u8,
            b: (b as f32 * alpha) as u8,
        }
    }
}

const HALF_BLOCK: char = '▀';

pub fn render_halfblock(image: &RgbaImage) -> Vec<Vec<(Color, Color, char)>> {
    let width = image.width();
    let height = image.height();
    let mut grid = Vec::new();

    let mut y = 0;
    while y < height {
        let mut row = Vec::with_capacity(width as usize);
        for x in 0..width {
            let top = image.get_pixel(x, y);
            let fg = Color::from_rgba(top[0], top[1], top[2], top[3]);

            let bg = if y + 1 < height {
                let bot = image.get_pixel(x, y + 1);
                Color::from_rgba(bot[0], bot[1], bot[2], bot[3])
            } else {
                Color { r: 0, g: 0, b: 0 }
            };

            row.push((fg, bg, HALF_BLOCK));
        }
        grid.push(row);
        y += 2;
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn test_red_2x4_image() {
        let mut img = RgbaImage::new(2, 4);
        for pixel in img.pixels_mut() {
            *pixel = image::Rgba([255, 0, 0, 255]);
        }

        let grid = render_halfblock(&img);

        assert_eq!(grid.len(), 2);
        assert_eq!(grid[0].len(), 2);
        assert_eq!(grid[1].len(), 2);

        let red = Color { r: 255, g: 0, b: 0 };
        for row in &grid {
            for &(fg, bg, ch) in row {
                assert_eq!(fg, red);
                assert_eq!(bg, red);
                assert_eq!(ch, '▀');
            }
        }
    }

    #[test]
    fn test_odd_height() {
        let mut img = RgbaImage::new(1, 3);
        for pixel in img.pixels_mut() {
            *pixel = image::Rgba([0, 255, 0, 255]);
        }

        let grid = render_halfblock(&img);

        assert_eq!(grid.len(), 2);
        let green = Color { r: 0, g: 255, b: 0 };
        let black = Color { r: 0, g: 0, b: 0 };

        assert_eq!(grid[0][0].0, green);
        assert_eq!(grid[0][0].1, green);
        assert_eq!(grid[1][0].0, green);
        assert_eq!(grid[1][0].1, black);
    }

    #[test]
    fn test_alpha_blending() {
        let mut img = RgbaImage::new(1, 2);
        img.put_pixel(0, 0, image::Rgba([200, 100, 50, 128]));
        img.put_pixel(0, 1, image::Rgba([255, 255, 255, 0]));

        let grid = render_halfblock(&img);

        let fg = grid[0][0].0;
        assert!(fg.r > 90 && fg.r < 110);
        assert!(fg.g > 45 && fg.g < 55);
        assert!(fg.b > 20 && fg.b < 30);

        let bg = grid[0][0].1;
        assert_eq!(bg, Color { r: 0, g: 0, b: 0 });
    }
}
