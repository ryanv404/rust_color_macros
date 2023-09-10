use std::io::Write;

use color_macros::{write_styled, writeln_styled, Color::*};

fn main() {
    let mut w = std::io::stdout().lock();

    // Foreground colors
    write_styled!(&mut w, Black, Current, "[ X ]");
    write_styled!(&mut w, Red, Current, "[ X ]");
    write_styled!(&mut w, Green, Current, "[ X ]");
    write_styled!(&mut w, Yellow, Current, "[ X ]");
    write_styled!(&mut w, Blue, Current, "[ X ]");
    write_styled!(&mut w, Magenta, Current, "[ X ]");
    write_styled!(&mut w, Cyan, Current, "[ X ]");
    write_styled!(&mut w, White, Current, "[ X ]");
    writeln_styled!(&mut w, Current, Current, "");
    write_styled!(&mut w, BrightBlack, Current, "[ X ]");
    write_styled!(&mut w, BrightRed, Current, "[ X ]");
    write_styled!(&mut w, BrightGreen, Current, "[ X ]");
    write_styled!(&mut w, BrightYellow, Current, "[ X ]");
    write_styled!(&mut w, BrightBlue, Current, "[ X ]");
    write_styled!(&mut w, BrightMagenta, Current, "[ X ]");
    write_styled!(&mut w, BrightCyan, Current, "[ X ]");
    write_styled!(&mut w, BrightWhite, Current, "[ X ]");
    writeln_styled!(&mut w, Current, Current, "");

    // Background colors
    write_styled!(&mut w, Current, Black, "[ X ]");
    write_styled!(&mut w, Current, Red, "[ X ]");
    write_styled!(&mut w, Current, Green, "[ X ]");
    write_styled!(&mut w, Current, Yellow, "[ X ]");
    write_styled!(&mut w, Current, Blue, "[ X ]");
    write_styled!(&mut w, Current, Magenta, "[ X ]");
    write_styled!(&mut w, Current, Cyan, "[ X ]");
    write_styled!(&mut w, Current, White, "[ X ]");
    writeln_styled!(&mut w, Current, Current, "");
    write_styled!(&mut w, Current, BrightBlack, "[ X ]");
    write_styled!(&mut w, Current, BrightRed, "[ X ]");
    write_styled!(&mut w, Current, BrightGreen, "[ X ]");
    write_styled!(&mut w, Current, BrightYellow, "[ X ]");
    write_styled!(&mut w, Current, BrightBlue, "[ X ]");
    write_styled!(&mut w, Current, BrightMagenta, "[ X ]");
    write_styled!(&mut w, Current, BrightCyan, "[ X ]");
    write_styled!(&mut w, Current, BrightWhite, "[ X ]");
    writeln_styled!(&mut w, Current, Current, "");

    // Mixed foreground and background colors
    write_styled!(&mut w, Black, Yellow, "[ X ]");
    write_styled!(&mut w, Red, Cyan, "[ X ]");
    write_styled!(&mut w, Green, Magenta, "[ X ]");
    write_styled!(&mut w, Blue, White, "[ X ]");
    write_styled!(&mut w, Yellow, Black, "[ X ]");
    write_styled!(&mut w, Cyan, Red, "[ X ]");
    write_styled!(&mut w, Magenta, Green, "[ X ]");
    write_styled!(&mut w, White, Blue, "[ X ]");
    writeln_styled!(&mut w, Current, Current, "");
    write_styled!(&mut w, BrightBlack, BrightYellow, "[ X ]");
    write_styled!(&mut w, BrightRed, BrightCyan, "[ X ]");
    write_styled!(&mut w, BrightGreen, BrightMagenta, "[ X ]");
    write_styled!(&mut w, BrightBlue, BrightWhite, "[ X ]");
    write_styled!(&mut w, BrightYellow, BrightBlack, "[ X ]");
    write_styled!(&mut w, BrightCyan, BrightRed, "[ X ]");
    write_styled!(&mut w, BrightMagenta, BrightGreen, "[ X ]");
    write_styled!(&mut w, BrightWhite, BrightBlue, "[ X ]");
    writeln_styled!(&mut w, Current, Current, "");

    // 256-color mode colors
    write_styled!(&mut w, Color256(123), Color256(52), "[ color256 ]");
    write_styled!(&mut w, Color256(52), Color256(123), "[ color256 ]");
    writeln_styled!(&mut w, Current, Current, "");

    // 24-bit RGB colors
    write_styled!(&mut w, Rgb(61, 12, 231), Rgb(231, 12, 61), "[   RGB    ]");
    write_styled!(&mut w, Rgb(231, 12, 61), Rgb(61, 12, 231), "[   RGB    ]");
    writeln_styled!(&mut w, Current, Current, "");
}
