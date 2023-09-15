use std::io::Write;

use color_macros::{print_rgb, println_rgb};

fn main() {
    let total_cols = 80;

    for col in 0..total_cols {
        let col = col as u32;

        let r = 255 - (col * 255 / total_cols);
        let r: u8 = r.try_into().unwrap();

        let g = col * 510 / total_cols;
        let g = if g > 255 { 510 - g } else { g };
        let g: u8 = g.try_into().unwrap();

        let b = col * 255 / total_cols;
        let b: u8 = b.try_into().unwrap();

        if col == (total_cols - 1) {
            println_rgb!((r, g, b), (255 - r, 255 - g, 255 - b), " ");
        } else {
            print_rgb!((r, g, b), (255 - r, 255 - g, 255 - b), " ");
        }
    }
}
