use std::io::Write;

use color_macros::{print_styled, println_styled, Color::*};

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
    println_styled!(Current, Current, "");
    print_styled!(BrightBlack, Current, "[ X ]");
    print_styled!(BrightRed, Current, "[ X ]");
    print_styled!(BrightGreen, Current, "[ X ]");
    print_styled!(BrightYellow, Current, "[ X ]");
    print_styled!(BrightBlue, Current, "[ X ]");
    print_styled!(BrightMagenta, Current, "[ X ]");
    print_styled!(BrightCyan, Current, "[ X ]");
    print_styled!(BrightWhite, Current, "[ X ]");
    println_styled!(Current, Current, "");

    // Background colors
    print_styled!(Current, Black, "[ X ]");
    print_styled!(Current, Red, "[ X ]");
    print_styled!(Current, Green, "[ X ]");
    print_styled!(Current, Yellow, "[ X ]");
    print_styled!(Current, Blue, "[ X ]");
    print_styled!(Current, Magenta, "[ X ]");
    print_styled!(Current, Cyan, "[ X ]");
    print_styled!(Current, White, "[ X ]");
    println_styled!(Current, Current, "");
    print_styled!(Current, BrightBlack, "[ X ]");
    print_styled!(Current, BrightRed, "[ X ]");
    print_styled!(Current, BrightGreen, "[ X ]");
    print_styled!(Current, BrightYellow, "[ X ]");
    print_styled!(Current, BrightBlue, "[ X ]");
    print_styled!(Current, BrightMagenta, "[ X ]");
    print_styled!(Current, BrightCyan, "[ X ]");
    print_styled!(Current, BrightWhite, "[ X ]");
    println_styled!(Current, Current, "");

    // Mixed foreground and background colors
    print_styled!(Black, Yellow, "[ X ]");
    print_styled!(Red, Cyan, "[ X ]");
    print_styled!(Green, Magenta, "[ X ]");
    print_styled!(Blue, White, "[ X ]");
    print_styled!(Yellow, Black, "[ X ]");
    print_styled!(Cyan, Red, "[ X ]");
    print_styled!(Magenta, Green, "[ X ]");
    print_styled!(White, Blue, "[ X ]");
    println_styled!(Current, Current, "");
    print_styled!(BrightBlack, BrightYellow, "[ X ]");
    print_styled!(BrightRed, BrightCyan, "[ X ]");
    print_styled!(BrightGreen, BrightMagenta, "[ X ]");
    print_styled!(BrightBlue, BrightWhite, "[ X ]");
    print_styled!(BrightYellow, BrightBlack, "[ X ]");
    print_styled!(BrightCyan, BrightRed, "[ X ]");
    print_styled!(BrightMagenta, BrightGreen, "[ X ]");
    print_styled!(BrightWhite, BrightBlue, "[ X ]");
    println_styled!(Current, Current, "");

    // 256-color mode colors
    print_styled!(Color256(123), Color256(52), "[ color256 ]");
    print_styled!(Color256(52), Color256(123), "[ color256 ]");
    println_styled!(Current, Current, "");

    // 24-bit RGB colors
    print_styled!(Rgb(61, 12, 231), Rgb(231, 12, 61), "[   RGB    ]");
    print_styled!(Rgb(231, 12, 61), Rgb(61, 12, 231), "[   RGB    ]");
    println_styled!(Current, Current, "");
}
