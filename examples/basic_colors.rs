use color_macros::{print_styled, println_styled};

fn main() {
    let colors: [&str; 16] = [
        "Black",
        "Red",
        "Green",
        "Yellow",
        "Blue",
        "Magenta",
        "Cyan",
        "White",
        "Bright Black",
        "Bright Red",
        "Bright Green",
        "Bright Yellow",
        "Bright Blue",
        "Bright Magenta",
        "Bright Cyan",
        "Bright White",
    ];

    for row in 0..=15 {
        for col in 0..=15 {
            match (col, row) {
                (15, r) => println_styled!(format!("{} on {}", colors[15], colors[r]), "[ + ]"),
                (c, r) => print_styled!(format!("{} on {}", colors[c], colors[r]), "[ + ]"),
            }
        }
    }
}
