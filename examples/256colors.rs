use std::io::Write;

use color_macros::{print_styled, println_styled};

fn main() {
    (0..=255).for_each(|num| {
        match num {
            7 | 15 => {
                println_styled!(Color256(num), Color256(0), format!("[{:3}]", num));
            },
            n if n > 16 && (n - 15) % 12 == 0 => {
                println_styled!(Color256(num), Color256(0), format!("[{:3}]", num));
            },
            0 | 8 | _ => {
                print_styled!(Color256(num), Color256(0), format!("[{:3}]", num));
            }
        }
    });
}
