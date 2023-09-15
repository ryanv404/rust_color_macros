use std::io::Write;

use color_macros::{print_color256, println_color256};

fn main() {
    (0..=255).for_each(|num| match num {
        7 | 15 => {
            println_color256!(num, 8, format!("[{:03}]", num));
        }
        n if n > 16 && (n - 15) % 12 == 0 => {
            println_color256!(num, 8, format!("[{:03}]", num));
        }
        0 | 8 | _ => {
            print_color256!(num, 8, format!("[{:03}]", num));
        }
    });
}
