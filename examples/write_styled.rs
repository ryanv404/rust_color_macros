use std::io::Write;

use color_macros::{write_styled, writeln_styled};

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
    writeln_styled!(&mut w, Current, Current, " <- regular");

    write_styled!(&mut w, BrightBlack, Current, "[ X ]");
    write_styled!(&mut w, BrightRed, Current, "[ X ]");
    write_styled!(&mut w, BrightGreen, Current, "[ X ]");
    write_styled!(&mut w, BrightYellow, Current, "[ X ]");
    write_styled!(&mut w, BrightBlue, Current, "[ X ]");
    write_styled!(&mut w, BrightMagenta, Current, "[ X ]");
    write_styled!(&mut w, BrightCyan, Current, "[ X ]");
    write_styled!(&mut w, BrightWhite, Current, "[ X ]");
    writeln_styled!(&mut w, Current, Current, " <- bright");

    // Background colors
    write_styled!(&mut w, Current, Black, "[ X ]");
    write_styled!(&mut w, Current, Red, "[ X ]");
    write_styled!(&mut w, Current, Green, "[ X ]");
    write_styled!(&mut w, Current, Yellow, "[ X ]");
    write_styled!(&mut w, Current, Blue, "[ X ]");
    write_styled!(&mut w, Current, Magenta, "[ X ]");
    write_styled!(&mut w, Current, Cyan, "[ X ]");
    write_styled!(&mut w, Current, White, "[ X ]");
    writeln_styled!(&mut w, Current, Current, " <- regular");

    write_styled!(&mut w, Current, BrightBlack, "[ X ]");
    write_styled!(&mut w, Current, BrightRed, "[ X ]");
    write_styled!(&mut w, Current, BrightGreen, "[ X ]");
    write_styled!(&mut w, Current, BrightYellow, "[ X ]");
    write_styled!(&mut w, Current, BrightBlue, "[ X ]");
    write_styled!(&mut w, Current, BrightMagenta, "[ X ]");
    write_styled!(&mut w, Current, BrightCyan, "[ X ]");
    write_styled!(&mut w, Current, BrightWhite, "[ X ]");
    writeln_styled!(&mut w, Current, Current, " <- bright");

    // 256-color mode colored text
//    write_styled!(&mut w, Color256(123), Current, "[ color256 ]");
//    write_styled!(&mut w, Current, Color256(52), "[ on_color256 ]");
//    writeln_styled!(&mut w, Current, Current, "");

    // RGB colored text
//    write_styled!(&mut w, Rgb(192, 123, 23), Current, "[ rgb ]");
//    write_styled!(&mut w, Current, Rgb(192, 123, 23), "[ on_rgb ]");
//    writeln_styled!(&mut w, Current, Current, "");

    // Mix of foreground and background colors
    writeln_styled!(&mut w, Black, Cyan, "[ black on cyan ]");
    writeln_styled!(&mut w, Green, Magenta, "[ green on magenta ]");
    writeln_styled!(&mut w, Red, White, "[ red on white ]");
    writeln_styled!(&mut w, Blue, Yellow, "[ blue on yellow ]");
    writeln_styled!(&mut w, BrightBlack, Cyan, "[ bright black on cyan ]");
    writeln_styled!(&mut w, BrightGreen, Magenta, "[ bright green on magenta ]");
    writeln_styled!(&mut w, BrightRed, White, "[ bright red on white ]");
    writeln_styled!(&mut w, BrightBlue, Yellow, "[ bright blue on yellow ]");
    writeln_styled!(&mut w, Black, BrightCyan, "[ black on bright cyan ]");
    writeln_styled!(&mut w, Green, BrightMagenta, "[ green on bright magenta ]");
    writeln_styled!(&mut w, Red, BrightWhite, "[ red on bright white ]");
    writeln_styled!(&mut w, Blue, BrightYellow, "[ blue on bright yellow ]");
    writeln_styled!(&mut w, BrightBlack, BrightCyan, "[ bright black on bright cyan ]");
    writeln_styled!(&mut w, BrightGreen, BrightMagenta, "[ bright green on bright Magenta ]");
    writeln_styled!(&mut w, BrightRed, BrightWhite, "[ bright red on bright white ]");
    writeln_styled!(&mut w, BrightBlue, BrightYellow, "[ bright blue on bright yellow ]");
}
