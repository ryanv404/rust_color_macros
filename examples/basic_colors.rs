use std::io::Write;

use color_macros::{
    print_styled, println_styled,
    Color::{self, *}
};

fn main() {
    let reg_colors: [Color; 8] = [
        Black, Red, Green, Yellow, Blue, Magenta, Cyan, White
    ];

    let bright_colors: [Color; 8] = [
        BrightBlack, BrightRed, BrightGreen, BrightYellow,
        BrightBlue, BrightMagenta, BrightCyan, BrightWhite
    ];

    (0..=7).for_each(|row| {
        (0..=7).for_each(|col| {
            match (col, row) {
                (7, r) => println_styled!(bright_colors[7], reg_colors[r], "[ + ]"),
                (c, r) => print_styled!(bright_colors[c], reg_colors[r], "[ + ]"),
            }
        });
    });
}
