use std::io::Write;

use color_macros::{eprint_styled, eprintln_styled};

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
    eprintln_styled!(Current, Current, " <- regular");

    eprint_styled!(BrightBlack, Current, "[ X ]");
    eprint_styled!(BrightRed, Current, "[ X ]");
    eprint_styled!(BrightGreen, Current, "[ X ]");
    eprint_styled!(BrightYellow, Current, "[ X ]");
    eprint_styled!(BrightBlue, Current, "[ X ]");
    eprint_styled!(BrightMagenta, Current, "[ X ]");
    eprint_styled!(BrightCyan, Current, "[ X ]");
    eprint_styled!(BrightWhite, Current, "[ X ]");
    eprintln_styled!(Current, Current, " <- bright");

    // Background colors
    eprint_styled!(Current, Black, "[ X ]");
    eprint_styled!(Current, Red, "[ X ]");
    eprint_styled!(Current, Green, "[ X ]");
    eprint_styled!(Current, Yellow, "[ X ]");
    eprint_styled!(Current, Blue, "[ X ]");
    eprint_styled!(Current, Magenta, "[ X ]");
    eprint_styled!(Current, Cyan, "[ X ]");
    eprint_styled!(Current, White, "[ X ]");
    eprintln_styled!(Current, Current, " <- regular");

    eprint_styled!(Current, BrightBlack, "[ X ]");
    eprint_styled!(Current, BrightRed, "[ X ]");
    eprint_styled!(Current, BrightGreen, "[ X ]");
    eprint_styled!(Current, BrightYellow, "[ X ]");
    eprint_styled!(Current, BrightBlue, "[ X ]");
    eprint_styled!(Current, BrightMagenta, "[ X ]");
    eprint_styled!(Current, BrightCyan, "[ X ]");
    eprint_styled!(Current, BrightWhite, "[ X ]");
    eprintln_styled!(Current, Current, " <- bright");

    // 256-color mode colored text
//    eprint_styled!(Color256(123), Current, "[ color256 ]");
//    eprint_styled!(Current, Color256(52), "[ on_color256 ]");
//    eprintln_styled!(Current, Current, "");

    // RGB colored text
//    eprint_styled!(Rgb(192, 123, 23), Current, "[ rgb ]");
//    eprint_styled!(Current, Rgb(192, 123, 23), "[ on_rgb ]");
//    eprintln_styled!(Current, Current, "");

    // Mix of foreground and background colors
    eprintln_styled!(Black, Cyan, "[ black on cyan ]");
    eprintln_styled!(Green, Magenta, "[ green on magenta ]");
    eprintln_styled!(Red, White, "[ red on white ]");
    eprintln_styled!(Blue, Yellow, "[ blue on yellow ]");
    eprintln_styled!(BrightBlack, Cyan, "[ bright black on cyan ]");
    eprintln_styled!(BrightGreen, Magenta, "[ bright green on magenta ]");
    eprintln_styled!(BrightRed, White, "[ bright red on white ]");
    eprintln_styled!(BrightBlue, Yellow, "[ bright blue on yellow ]");
    eprintln_styled!(Black, BrightCyan, "[ black on bright cyan ]");
    eprintln_styled!(Green, BrightMagenta, "[ green on bright magenta ]");
    eprintln_styled!(Red, BrightWhite, "[ red on bright white ]");
    eprintln_styled!(Blue, BrightYellow, "[ blue on bright yellow ]");
    eprintln_styled!(BrightBlack, BrightCyan, "[ bright black on bright cyan ]");
    eprintln_styled!(BrightGreen, BrightMagenta, "[ bright green on bright Magenta ]");
    eprintln_styled!(BrightRed, BrightWhite, "[ bright red on bright white ]");
    eprintln_styled!(BrightBlue, BrightYellow, "[ bright blue on bright yellow ]");
}
