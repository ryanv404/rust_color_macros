use std::io::Write;

use color_macros::{
    print_color, println_color,
    Color::{self, *},
};

fn main() {
    let reg_colors: [Color; 8] = [Black, Red, Green, Yellow, Blue, Magenta, Cyan, White];

    let bright_colors: [Color; 8] = [
        BrBlack, BrRed, BrGreen, BrYellow, BrBlue, BrMagenta, BrCyan, BrWhite,
    ];

    (0..=7).for_each(|row| {
        (0..=7).for_each(|col| match (col, row) {
            (7, r) => println_color!(bright_colors[7], reg_colors[r], "[ + ]"),
            (c, r) => print_color!(bright_colors[c], reg_colors[r], "[ + ]"),
        });
    });
}
