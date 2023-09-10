use std::io::Write;

use color_macros::{print_styled, println_styled};

fn main() {
    (0..=15).for_each(|num| {
        if num == 0 {
            print_styled!(Color256(num), Color256(255), format!("[{:3}]", num));
        } else if num == 7 {
            println_styled!(Color256(num), Color256(0), format!("[{:3}]", num));
        } else if num == 15 {
            println_styled!(Color256(num), Color256(0), format!("[{:3}]", num));
        } else {
            print_styled!(Color256(num), Color256(0), format!("[{:3}]", num));
        }
    });

    (16..=255).for_each(|num| {
        if (num - 15) % 12 == 0 {
            println_styled!(Color256(num), Color256(0), format!("[{:3}]", num));
        } else {
            print_styled!(Color256(num), Color256(0), format!("[{:3}]", num));
        }
    });
}
