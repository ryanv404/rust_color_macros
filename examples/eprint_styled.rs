use std::io::Write;

use color_macros::{eprint_styled, eprintln_styled, Color::*};

fn main() {
    // Foreground colors
    eprint_styled!(Black, Current, "[ X ]");
    eprint_styled!(Red, Current, "[ X ]");
    eprint_styled!(Green, Current, "[ X ]");
    eprint_styled!(Yellow, Current, "[ X ]");
    eprint_styled!(Blue, Current, "[ X ]");
    eprint_styled!(Magenta, Current, "[ X ]");
    eprint_styled!(Cyan, Current, "[ X ]");
    eprint_styled!(White, Current, "[ X ]");
    eprintln_styled!(Current, Current, "");
    eprint_styled!(BrightBlack, Current, "[ X ]");
    eprint_styled!(BrightRed, Current, "[ X ]");
    eprint_styled!(BrightGreen, Current, "[ X ]");
    eprint_styled!(BrightYellow, Current, "[ X ]");
    eprint_styled!(BrightBlue, Current, "[ X ]");
    eprint_styled!(BrightMagenta, Current, "[ X ]");
    eprint_styled!(BrightCyan, Current, "[ X ]");
    eprint_styled!(BrightWhite, Current, "[ X ]");
    eprintln_styled!(Current, Current, "");

    // Background colors
    eprint_styled!(Current, Black, "[ X ]");
    eprint_styled!(Current, Red, "[ X ]");
    eprint_styled!(Current, Green, "[ X ]");
    eprint_styled!(Current, Yellow, "[ X ]");
    eprint_styled!(Current, Blue, "[ X ]");
    eprint_styled!(Current, Magenta, "[ X ]");
    eprint_styled!(Current, Cyan, "[ X ]");
    eprint_styled!(Current, White, "[ X ]");
    eprintln_styled!(Current, Current, "");
    eprint_styled!(Current, BrightBlack, "[ X ]");
    eprint_styled!(Current, BrightRed, "[ X ]");
    eprint_styled!(Current, BrightGreen, "[ X ]");
    eprint_styled!(Current, BrightYellow, "[ X ]");
    eprint_styled!(Current, BrightBlue, "[ X ]");
    eprint_styled!(Current, BrightMagenta, "[ X ]");
    eprint_styled!(Current, BrightCyan, "[ X ]");
    eprint_styled!(Current, BrightWhite, "[ X ]");
    eprintln_styled!(Current, Current, "");

    // Mixed foreground and background colors
    eprint_styled!(Black, Yellow, "[ X ]");
    eprint_styled!(Red, Cyan, "[ X ]");
    eprint_styled!(Green, Magenta, "[ X ]");
    eprint_styled!(Blue, White, "[ X ]");
    eprint_styled!(Yellow, Black, "[ X ]");
    eprint_styled!(Cyan, Red, "[ X ]");
    eprint_styled!(Magenta, Green, "[ X ]");
    eprint_styled!(White, Blue, "[ X ]");
    eprintln_styled!(Current, Current, "");
    eprint_styled!(BrightBlack, BrightYellow, "[ X ]");
    eprint_styled!(BrightRed, BrightCyan, "[ X ]");
    eprint_styled!(BrightGreen, BrightMagenta, "[ X ]");
    eprint_styled!(BrightBlue, BrightWhite, "[ X ]");
    eprint_styled!(BrightYellow, BrightBlack, "[ X ]");
    eprint_styled!(BrightCyan, BrightRed, "[ X ]");
    eprint_styled!(BrightMagenta, BrightGreen, "[ X ]");
    eprint_styled!(BrightWhite, BrightBlue, "[ X ]");
    eprintln_styled!(Current, Current, "");

    // 256-color mode colors
    eprint_styled!(Color256(123), Color256(52), "[ color256 ]");
    eprint_styled!(Color256(52), Color256(123), "[ color256 ]");
    eprintln_styled!(Current, Current, "");

    // 24-bit RGB colors
    eprint_styled!(Rgb(61, 12, 231), Rgb(231, 12, 61), "[   RGB    ]");
    eprint_styled!(Rgb(231, 12, 61), Rgb(61, 12, 231), "[   RGB    ]");
    eprintln_styled!(Current, Current, "");
}
