use std::io::Write;

use color_macros::{print_styled, println_styled};

fn main() {
    // Foreground colors
    print_styled!(Black, Current, "[ X ]");
    print_styled!(Red, Current, "[ X ]");
    print_styled!(Green, Current, "[ X ]");
    print_styled!(Yellow, Current, "[ X ]");
    print_styled!(Blue, Current, "[ X ]");
    print_styled!(Magenta, Current, "[ X ]");
    print_styled!(Cyan, Current, "[ X ]");
    print_styled!(White, Current, "[ X ]");
    println_styled!(Current, Current, " <- regular");

    print_styled!(BrightBlack, Current, "[ X ]");
    print_styled!(BrightRed, Current, "[ X ]");
    print_styled!(BrightGreen, Current, "[ X ]");
    print_styled!(BrightYellow, Current, "[ X ]");
    print_styled!(BrightBlue, Current, "[ X ]");
    print_styled!(BrightMagenta, Current, "[ X ]");
    print_styled!(BrightCyan, Current, "[ X ]");
    print_styled!(BrightWhite, Current, "[ X ]");
    println_styled!(Current, Current, " <- bright");

    // Background colors
    print_styled!(Current, Black, "[ X ]");
    print_styled!(Current, Red, "[ X ]");
    print_styled!(Current, Green, "[ X ]");
    print_styled!(Current, Yellow, "[ X ]");
    print_styled!(Current, Blue, "[ X ]");
    print_styled!(Current, Magenta, "[ X ]");
    print_styled!(Current, Cyan, "[ X ]");
    print_styled!(Current, White, "[ X ]");
    println_styled!(Current, Current, " <- regular");

    print_styled!(Current, BrightBlack, "[ X ]");
    print_styled!(Current, BrightRed, "[ X ]");
    print_styled!(Current, BrightGreen, "[ X ]");
    print_styled!(Current, BrightYellow, "[ X ]");
    print_styled!(Current, BrightBlue, "[ X ]");
    print_styled!(Current, BrightMagenta, "[ X ]");
    print_styled!(Current, BrightCyan, "[ X ]");
    print_styled!(Current, BrightWhite, "[ X ]");
    println_styled!(Current, Current, " <- bright");

    // 256-color mode colored text
//    print_styled!(Color256(123), Current, "[ color256 ]");
//    print_styled!(Current, Color256(52), "[ on_color256 ]");
//    println_styled!(Current, Current, "");

    // RGB colored text
//    print_styled!(Rgb(192, 123, 23), Current, "[ rgb ]");
//    print_styled!(Current, Rgb(192, 123, 23), "[ on_rgb ]");
//    println_styled!(Current, Current, "");

    // Mix of foreground and background colors
    println_styled!(Black, Cyan, "[ black on cyan ]");
    println_styled!(Green, Magenta, "[ green on magenta ]");
    println_styled!(Red, White, "[ red on white ]");
    println_styled!(Blue, Yellow, "[ blue on yellow ]");
    println_styled!(BrightBlack, Cyan, "[ bright black on cyan ]");
    println_styled!(BrightGreen, Magenta, "[ bright green on magenta ]");
    println_styled!(BrightRed, White, "[ bright red on white ]");
    println_styled!(BrightBlue, Yellow, "[ bright blue on yellow ]");
    println_styled!(Black, BrightCyan, "[ black on bright cyan ]");
    println_styled!(Green, BrightMagenta, "[ green on bright magenta ]");
    println_styled!(Red, BrightWhite, "[ red on bright white ]");
    println_styled!(Blue, BrightYellow, "[ blue on bright yellow ]");
    println_styled!(BrightBlack, BrightCyan, "[ bright black on bright cyan ]");
    println_styled!(BrightGreen, BrightMagenta, "[ bright green on bright Magenta ]");
    println_styled!(BrightRed, BrightWhite, "[ bright red on bright white ]");
    println_styled!(BrightBlue, BrightYellow, "[ bright blue on bright yellow ]");
}
