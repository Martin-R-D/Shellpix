const CUBE_VALUES: [u8; 6] = [0, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

fn nearest_cube_index(v: u8) -> usize {
    let mut best = 0;
    let mut best_dist = i16::MAX;
    for (i, &cv) in CUBE_VALUES.iter().enumerate() {
        let dist = (v as i16 - cv as i16).abs();
        if dist < best_dist {
            best_dist = dist;
            best = i;
        }
    }
    best
}

pub fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let ri = nearest_cube_index(r);
    let gi = nearest_cube_index(g);
    let bi = nearest_cube_index(b);
    let cube_index = (16 + 36 * ri + 6 * gi + bi) as u8;
    let cube_r = CUBE_VALUES[ri] as i32;
    let cube_g = CUBE_VALUES[gi] as i32;
    let cube_b = CUBE_VALUES[bi] as i32;
    let cube_dist = (r as i32 - cube_r).pow(2)
        + (g as i32 - cube_g).pow(2)
        + (b as i32 - cube_b).pow(2);

    let gray_approx = (r as i32 * 30 + g as i32 * 59 + b as i32 * 11) / 100;
    let gray_idx = ((gray_approx - 8).max(0) * 24 / 238).min(23) as u8;
    let gray_value = (8 + 10 * gray_idx as i32) as i32;
    let gray_dist = (r as i32 - gray_value).pow(2)
        + (g as i32 - gray_value).pow(2)
        + (b as i32 - gray_value).pow(2);

    if gray_dist < cube_dist {
        232 + gray_idx
    } else {
        cube_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_red() {
        let idx = rgb_to_ansi256(255, 0, 0);
        // 196 = 16 + 36*5 + 6*0 + 0 (the brightest red in the cube)
        assert_eq!(idx, 196);
    }

    #[test]
    fn test_pure_white() {
        let idx = rgb_to_ansi256(255, 255, 255);
        // 231 = 16 + 36*5 + 6*5 + 5 (brightest white in the cube)
        assert_eq!(idx, 231);
    }

    #[test]
    fn test_mid_gray() {
        let idx = rgb_to_ansi256(128, 128, 128);
        // Should land in the grayscale ramp (232-255)
        assert!(idx >= 232);
    }

    #[test]
    fn test_pure_black() {
        let idx = rgb_to_ansi256(0, 0, 0);
        // 16 = 16 + 36*0 + 6*0 + 0 (black in the cube)
        assert_eq!(idx, 16);
    }
}
