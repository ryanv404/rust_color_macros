//! `color_macros`
//! [![Rust](https://github.com/ryanv404/rust_color_macros/actions/workflows/rust.yml/badge.svg)](https://github.com/ryanv404/rust_color_macros/actions/workflows/rust.yml)
//! A library of simple macros for writing colored text to a buffer or to the terminal.
//!
//! ## Features
//! Set foreground and background colors using macros that are reminiscent of the Rust
//! standard library macros. There are three different color modes.
//!
//! Basic Color Mode:
//! - Writes to buffer: `write_color!()`, `writeln_color!()`
//! - Prints to stdout: `print_color!()`, `println_color!()`
//! - Prints to stderr: `eprint_color!()`, `eprintln_color!()`
//!
//! 256-Color Mode:
//! - Writes to buffer: `write_color256!()`, `writeln_color256!()`
//! - Prints to stdout: `print_color256!()`, `println_color256!()`
//! - Prints to stderr: `eprint_color256!()`, `eprintln_color256!()`
//!
//! 24-Bit RGB Color Mode:
//! - Writes to buffer: `write_rgb!()`, `writeln_rgb!()`
//! - Prints to stdout: `print_rgb!()`, `println_rgb!()`
//! - Prints to stderr: `eprint_rgb!()`, `eprintln_rgb!()`

#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![deny(missing_docs)]

/// Color options
#[derive(PartialEq, Eq)]
pub enum Color {
    /// Black
    Black,
    /// Red
    Red,
    /// Green
    Green,
    /// Yellow
    Yellow,
    /// Blue
    Blue,
    /// Magenta
    Magenta,
    /// Cyan
    Cyan,
    /// White
    White,
    /// Bright black
    BrBlack,
    /// Bright red
    BrRed,
    /// Bright green
    BrGreen,
    /// Bright yellow
    BrYellow,
    /// Bright blue
    BrBlue,
    /// Bright magenta
    BrMagenta,
    /// Bright cyan
    BrCyan,
    /// Bright white
    BrWhite,
    /// Current color value
    Current,
    /// Reset color attributes
    Reset,
    /// 256-color mode colors
    Color256(u8),
    /// 24-bit RGB colors
    Rgb(u8, u8, u8),
}

/// Write color text to a buffer.
///
/// # Arguments
/// * `$buffer` - The destination buffer.
/// * `$fg` - The foreground `Color` value.
/// * `$bg` - The background `Color` value.
/// * `$text` - The text to write.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::{write_color, Color};
///
/// let mut buffer: Vec<u8> = vec![];
///
/// // Write red text on a white background to a buffer.
/// write_color!(&mut buffer, Color::Red, Color::White, "test");
///
/// assert_eq!(buffer.as_slice(), b"\x1b[31;47mtest\x1b[0m");
/// ```
#[macro_export]
macro_rules! write_color {
    ($buffer:expr, $fg:expr, $bg:expr, $text:expr) => {
        write!(
            $buffer,
            "\u{001b}[{}{}{}m{}\u{001b}[0m",
            $fg.get_fg_code(),
            $fg.get_separator(&$bg),
            $bg.get_bg_code(),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    };
}

/// Same as `write_color!()` but with a newline appended at the end.
#[macro_export]
macro_rules! writeln_color {
    ($buffer:expr, $fg:expr, $bg:expr, $text:expr) => {
        write!(
            $buffer,
            "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
            $fg.get_fg_code(),
            $fg.get_separator(&$bg),
            $bg.get_bg_code(),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    };
}

/// Print color text to stdout.
///
/// # Arguments
/// * `$fg` - The foreground `Color` value.
/// * `$bg` - The background `Color` value.
/// * `$text` - The text to print to stdout.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::{print_color, Color};
///
/// // Prints red text on a white background to stdout.
/// print_color!(Color::Red, Color::White, "test");
/// ```
#[macro_export]
macro_rules! print_color {
    ($fg:expr, $bg:expr, $text:expr) => {{
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{}{}{}m{}\u{001b}[0m",
            $fg.get_fg_code(),
            $fg.get_separator(&$bg),
            $bg.get_bg_code(),
            $text
        )
        .unwrap();
        stdout.flush().unwrap();
    }};
}

/// Same as `print_color!()` but with a newline appended at the end.
#[macro_export]
macro_rules! println_color {
    ($fg:expr, $bg:expr, $text:expr) => {{
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
            $fg.get_fg_code(),
            $fg.get_separator(&$bg),
            $bg.get_bg_code(),
            $text
        )
        .unwrap();
        stdout.flush().unwrap();
    }};
}

/// Print color text to stderr.
///
/// # Arguments
/// * `$fg` - The foreground `Color` value.
/// * `$bg` - The background `Color` value.
/// * `$text` - The text to print to stderr.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::{eprint_color, Color};
///
/// // Prints red text on a white background to stderr.
/// eprint_color!(Color::Red, Color::White, "test");
/// ```
#[macro_export]
macro_rules! eprint_color {
    ($fg:expr, $bg:expr, $text:expr) => {{
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{}{}{}m{}\u{001b}[0m",
            $fg.get_fg_code(),
            $fg.get_separator(&$bg),
            $bg.get_bg_code(),
            $text
        )
        .unwrap();
        stderr.flush().unwrap();
    }};
}

/// Same as `eprint_color!()` but with a newline appended at the end.
#[macro_export]
macro_rules! eprintln_color {
    ($fg:expr, $bg:expr, $text:expr) => {{
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
            $fg.get_fg_code(),
            $fg.get_separator(&$bg),
            $bg.get_bg_code(),
            $text
        )
        .unwrap();
        stderr.flush().unwrap();
    }};
}

/// Write color text to a buffer using 256-color mode.
///
/// # Arguments
/// * `$buffer` - The destination buffer.
/// * `$fg_num` - The foreground 256-color mode color number.
/// * `$bg_num` - The background 256-color mode color number.
/// * `$text` - The text to write.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::write_color256;
///
/// let mut buffer: Vec<u8> = vec![];
///
/// // Write red text on a white background to a buffer.
/// write_color256!(&mut buffer, 21, 255, "test");
///
/// assert_eq!(buffer.as_slice(), b"\x1b[38;5;21;48;5;255mtest\x1b[0m");
/// ```
#[macro_export]
macro_rules! write_color256 {
    ($buffer:expr, $fg_num:expr, $bg_num:expr, $text:expr) => {
        write!(
            $buffer,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            $crate::Color::Color256($fg_num).get_fg_code(),
            $crate::Color::Color256($bg_num).get_bg_code(),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    };
}

/// Same as `write_color256!()` but with a newline appended at the end.
#[macro_export]
macro_rules! writeln_color256 {
    ($buffer:expr, $fg_num:expr, $bg_num:expr, $text:expr) => {
        write!(
            $buffer,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            $crate::Color::Color256($fg_num).get_fg_code(),
            $crate::Color::Color256($bg_num).get_bg_code(),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    };
}

/// Print color text to stdout.
///
/// # Arguments
/// * `$fg_num` - The foreground 256-color mode color number.
/// * `$bg_num` - The background 256-color mode color number.
/// * `$text` - The text to print to stdout.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::print_color256;
///
/// // Prints red text on a white background to stdout.
/// print_color256!(21, 255, "test");
/// ```
#[macro_export]
macro_rules! print_color256 {
    ($fg_num:expr, $bg_num:expr, $text:expr) => {{
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            $crate::Color::Color256($fg_num).get_fg_code(),
            $crate::Color::Color256($bg_num).get_bg_code(),
            $text
        )
        .unwrap();
        stdout.flush().unwrap();
    }};
}

/// Same as `print_color256!()` but with a newline appended at the end.
#[macro_export]
macro_rules! println_color256 {
    ($fg_num:expr, $bg_num:expr, $text:expr) => {{
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            $crate::Color::Color256($fg_num).get_fg_code(),
            $crate::Color::Color256($bg_num).get_bg_code(),
            $text
        )
        .unwrap();
        stdout.flush().unwrap();
    }};
}

/// Print color text to stderr.
///
/// # Arguments
/// * `$fg_num` - The foreground 256-color mode color number.
/// * `$bg_num` - The background 256-color mode color number.
/// * `$text` - The text to print to stderr.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::eprint_color256;
///
/// // Prints red text on a white background to stderr.
/// eprint_color256!(21, 255, "test");
/// ```
#[macro_export]
macro_rules! eprint_color256 {
    ($fg_num:expr, $bg_num:expr, $text:expr) => {{
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            $crate::Color::Color256($fg_num).get_fg_code(),
            $crate::Color::Color256($bg_num).get_bg_code(),
            $text
        )
        .unwrap();
        stderr.flush().unwrap();
    }};
}

/// Same as `eprint_color256!()` but with a newline appended at the end.
#[macro_export]
macro_rules! eprintln_color256 {
    ($fg_num:expr, $bg_num:expr, $text:expr) => {{
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            $crate::Color::Color256($fg_num).get_fg_code(),
            $crate::Color::Color256($bg_num).get_bg_code(),
            $text
        )
        .unwrap();
        stderr.flush().unwrap();
    }};
}

/// Write color text to a buffer using 24-bit RGB color values.
///
/// # Arguments
/// * `$buffer` - The destination buffer.
/// * (`$fg_r`, `$fg_g`, `$fg_b`) - The foreground RGB value.
/// * (`$bg_r`, `$bg_g`, `$bg_b`) - The background RGB value.
/// * `$text` - The text to write.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::write_rgb;
///
/// let mut buffer: Vec<u8> = vec![];
///
/// // Write red text on a white background to a buffer.
/// write_rgb!(&mut buffer, (211, 0, 0), (255, 255, 255), "test");
///
/// assert_eq!(buffer.as_slice(), b"\x1b[38;2;211;0;0;48;2;255;255;255mtest\x1b[0m");
/// ```
#[macro_export]
macro_rules! write_rgb {
    ($buffer:expr, ($fg_r:expr, $fg_g:expr, $fg_b:expr),
    ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {
        write!(
            $buffer,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            $crate::Color::Rgb($fg_r, $fg_g, $fg_b).get_fg_code(),
            $crate::Color::Rgb($bg_r, $bg_g, $bg_b).get_bg_code(),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    };
}

/// Same as `write_rgb!()` but with a newline appended at the end.
#[macro_export]
macro_rules! writeln_rgb {
    ($buffer:expr, ($fg_r:expr, $fg_g:expr, $fg_b:expr),
    ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {
        write!(
            $buffer,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            $crate::Color::Rgb($fg_r, $fg_g, $fg_b).get_fg_code(),
            $crate::Color::Rgb($bg_r, $bg_g, $bg_b).get_bg_code(),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    };
}

/// Print color text to stdout using 24-bit RGB color values.
///
/// # Arguments
/// * (`$fg_r`, `$fg_g`, `$fg_b`) - The foreground RGB value.
/// * (`$bg_r`, `$bg_g`, `$bg_b`) - The background RGB value.
/// * `$text` - The text to print to stdout.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::print_rgb;
///
/// // Print red text on a white background to stdout.
/// print_rgb!((211, 222, 233), (0, 0, 0), "test");
/// ```
#[macro_export]
macro_rules! print_rgb {
    (($fg_r:expr, $fg_g:expr, $fg_b:expr),
     ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {{
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            $crate::Color::Rgb($fg_r, $fg_g, $fg_b).get_fg_code(),
            $crate::Color::Rgb($bg_r, $bg_g, $bg_b).get_bg_code(),
            $text
        )
        .unwrap();
        stdout.flush().unwrap();
    }};
}

/// Same as `print_rgb!()` but with a newline appended at the end.
#[macro_export]
macro_rules! println_rgb {
    (($fg_r:expr, $fg_g:expr, $fg_b:expr),
     ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {{
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            $crate::Color::Rgb($fg_r, $fg_g, $fg_b).get_fg_code(),
            $crate::Color::Rgb($bg_r, $bg_g, $bg_b).get_bg_code(),
            $text
        )
        .unwrap();
        stdout.flush().unwrap();
    }};
}

/// Print color text to stderr using 24-bit RGB color values.
///
/// # Arguments
/// * (`$fg_r`, `$fg_g`, `$fg_b`) - The foreground RGB value.
/// * (`$bg_r`, `$bg_g`, `$bg_b`) - The background RGB value.
/// * `$text` - The text to print to stderr.
///
/// # Example
///
/// ```rust
/// use std::io::Write;
/// use color_macros::eprint_rgb;
///
/// // Print red text on a white background to stderr.
/// eprint_rgb!((211, 222, 233), (0, 0, 0), "test");
/// ```
#[macro_export]
macro_rules! eprint_rgb {
    (($fg_r:expr, $fg_g:expr, $fg_b:expr),
     ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {{
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            $crate::Color::Rgb($fg_r, $fg_g, $fg_b).get_fg_code(),
            $crate::Color::Rgb($bg_r, $bg_g, $bg_b).get_bg_code(),
            $text
        )
        .unwrap();
        stderr.flush().unwrap();
    }};
}

/// Same as `eprint_rgb!()` but with a newline appended at the end.
#[macro_export]
macro_rules! eprintln_rgb {
    (($fg_r:expr, $fg_g:expr, $fg_b:expr),
     ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {{
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            $crate::Color::Rgb($fg_r, $fg_g, $fg_b).get_fg_code(),
            $crate::Color::Rgb($bg_r, $bg_g, $bg_b).get_bg_code(),
            $text
        )
        .unwrap();
        stderr.flush().unwrap();
    }};
}

impl Color {
    /// Returns true if this Color instance is `Color::Current`.
    #[must_use]
    pub fn is_current(&self) -> bool {
        *self == Self::Current
    }

    /// Returns a semicolon separator if self and other are not `Color::Current`.
    /// Else it returns an empty string.
    #[must_use]
    pub fn get_separator(&self, other: &Self) -> String {
        if !self.is_current() && !other.is_current() {
            ";".to_string()
        } else {
            String::new()
        }
    }

    /// Returns the numeric portion of the foreground ANSI color code.
    #[must_use]
    pub fn get_fg_code(&self) -> String {
        match *self {
            Self::Black => "30".to_string(),
            Self::Red => "31".to_string(),
            Self::Green => "32".to_string(),
            Self::Yellow => "33".to_string(),
            Self::Blue => "34".to_string(),
            Self::Magenta => "35".to_string(),
            Self::Cyan => "36".to_string(),
            Self::White => "37".to_string(),
            Self::BrBlack => "90".to_string(),
            Self::BrRed => "91".to_string(),
            Self::BrGreen => "92".to_string(),
            Self::BrYellow => "93".to_string(),
            Self::BrBlue => "94".to_string(),
            Self::BrMagenta => "95".to_string(),
            Self::BrCyan => "96".to_string(),
            Self::BrWhite => "97".to_string(),
            Self::Color256(num) => format!("38;5;{num}"),
            Self::Rgb(r, g, b) => format!("38;2;{r};{g};{b}"),
            Self::Current => String::new(),
            Self::Reset => "0".to_string(),
        }
    }

    /// Returns the numeric portion of the background ANSI color code.
    #[must_use]
    pub fn get_bg_code(&self) -> String {
        match *self {
            Self::Black => "40".to_string(),
            Self::Red => "41".to_string(),
            Self::Green => "42".to_string(),
            Self::Yellow => "43".to_string(),
            Self::Blue => "44".to_string(),
            Self::Magenta => "45".to_string(),
            Self::Cyan => "46".to_string(),
            Self::White => "47".to_string(),
            Self::BrBlack => "100".to_string(),
            Self::BrRed => "101".to_string(),
            Self::BrGreen => "102".to_string(),
            Self::BrYellow => "103".to_string(),
            Self::BrBlue => "104".to_string(),
            Self::BrMagenta => "105".to_string(),
            Self::BrCyan => "106".to_string(),
            Self::BrWhite => "107".to_string(),
            Self::Color256(num) => format!("48;5;{num}"),
            Self::Rgb(r, g, b) => format!("48;2;{r};{g};{b}"),
            Self::Current => String::new(),
            Self::Reset => "0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::Color::*;
    use super::*;

    macro_rules! test_color {
        ($test_name:ident:
         $fg:tt, $bg:tt,
         $text:literal => $expected:literal) => {
            #[test]
            fn $test_name() {
                let mut output: Vec<u8> = vec![];
                write_color!(&mut output, $fg, $bg, $text);
                assert_eq!(output.as_slice(), $expected);
            }
        };
    }

    macro_rules! test_rgb {
        ($test_name:ident:
         ($fg_r:tt, $fg_g:tt, $fg_b:tt),
         ($bg_r:tt, $bg_g:tt, $bg_b:tt),
         $text:literal => $expected:literal) => {
            #[test]
            fn $test_name() {
                let mut output: Vec<u8> = vec![];
                write_rgb!(
                    &mut output,
                    ($fg_r, $fg_g, $fg_b),
                    ($bg_r, $bg_g, $bg_b),
                    $text
                );
                assert_eq!(output.as_slice(), $expected);
            }
        };
    }

    // Basic foreground color tests
    test_color!(blk:    Black,     Current, "hi" => b"\x1b[30mhi\x1b[0m");
    test_color!(red:    Red,       Current, "hi" => b"\x1b[31mhi\x1b[0m");
    test_color!(grn:    Green,     Current, "hi" => b"\x1b[32mhi\x1b[0m");
    test_color!(ylw:    Yellow,    Current, "hi" => b"\x1b[33mhi\x1b[0m");
    test_color!(blu:    Blue,      Current, "hi" => b"\x1b[34mhi\x1b[0m");
    test_color!(mag:    Magenta,   Current, "hi" => b"\x1b[35mhi\x1b[0m");
    test_color!(cyn:    Cyan,      Current, "hi" => b"\x1b[36mhi\x1b[0m");
    test_color!(wht:    White,     Current, "hi" => b"\x1b[37mhi\x1b[0m");
    test_color!(br_blk: BrBlack,   Current, "hi" => b"\x1b[90mhi\x1b[0m");
    test_color!(br_red: BrRed,     Current, "hi" => b"\x1b[91mhi\x1b[0m");
    test_color!(br_grn: BrGreen,   Current, "hi" => b"\x1b[92mhi\x1b[0m");
    test_color!(br_ylw: BrYellow,  Current, "hi" => b"\x1b[93mhi\x1b[0m");
    test_color!(br_blu: BrBlue,    Current, "hi" => b"\x1b[94mhi\x1b[0m");
    test_color!(br_mag: BrMagenta, Current, "hi" => b"\x1b[95mhi\x1b[0m");
    test_color!(br_cyn: BrCyan,    Current, "hi" => b"\x1b[96mhi\x1b[0m");
    test_color!(br_wht: BrWhite,   Current, "hi" => b"\x1b[97mhi\x1b[0m");

    // Basic background color tests
    test_color!(bg_blk:    Current, Black,     "hi" => b"\x1b[40mhi\x1b[0m");
    test_color!(bg_red:    Current, Red,       "hi" => b"\x1b[41mhi\x1b[0m");
    test_color!(bg_grn:    Current, Green,     "hi" => b"\x1b[42mhi\x1b[0m");
    test_color!(bg_ylw:    Current, Yellow,    "hi" => b"\x1b[43mhi\x1b[0m");
    test_color!(bg_blu:    Current, Blue,      "hi" => b"\x1b[44mhi\x1b[0m");
    test_color!(bg_mag:    Current, Magenta,   "hi" => b"\x1b[45mhi\x1b[0m");
    test_color!(bg_cyn:    Current, Cyan,      "hi" => b"\x1b[46mhi\x1b[0m");
    test_color!(bg_wht:    Current, White,     "hi" => b"\x1b[47mhi\x1b[0m");
    test_color!(bg_br_blk: Current, BrBlack,   "hi" => b"\x1b[100mhi\x1b[0m");
    test_color!(bg_br_red: Current, BrRed,     "hi" => b"\x1b[101mhi\x1b[0m");
    test_color!(bg_br_grn: Current, BrGreen,   "hi" => b"\x1b[102mhi\x1b[0m");
    test_color!(bg_br_ylw: Current, BrYellow,  "hi" => b"\x1b[103mhi\x1b[0m");
    test_color!(bg_br_blu: Current, BrBlue,    "hi" => b"\x1b[104mhi\x1b[0m");
    test_color!(bg_br_mag: Current, BrMagenta, "hi" => b"\x1b[105mhi\x1b[0m");
    test_color!(bg_br_cyn: Current, BrCyan,    "hi" => b"\x1b[106mhi\x1b[0m");
    test_color!(bg_br_wht: Current, BrWhite,   "hi" => b"\x1b[107mhi\x1b[0m");

    // Mixed foreground/background color tests
    test_color!(red_on_wht:        Red,     White,     "hi" => b"\x1b[31;47mhi\x1b[0m");
    test_color!(blk_on_cyan:       Black,   Cyan,      "hi" => b"\x1b[30;46mhi\x1b[0m");
    test_color!(grn_on_purp:       Green,   Magenta,   "hi" => b"\x1b[32;45mhi\x1b[0m");
    test_color!(blue_on_ylw:       Blue,    Yellow,    "hi" => b"\x1b[34;43mhi\x1b[0m");
    test_color!(br_red_on_wht:     BrRed,   White,     "hi" => b"\x1b[91;47mhi\x1b[0m");
    test_color!(br_blk_on_cyan:    BrBlack, Cyan,      "hi" => b"\x1b[90;46mhi\x1b[0m");
    test_color!(br_grn_on_purp:    BrGreen, Magenta,   "hi" => b"\x1b[92;45mhi\x1b[0m");
    test_color!(br_blue_on_ylw:    BrBlue,  Yellow,    "hi" => b"\x1b[94;43mhi\x1b[0m");
    test_color!(red_on_br_wht:     Red,     BrWhite,   "hi" => b"\x1b[31;107mhi\x1b[0m");
    test_color!(blk_on_br_cyan:    Black,   BrCyan,    "hi" => b"\x1b[30;106mhi\x1b[0m");
    test_color!(grn_on_br_purp:    Green,   BrMagenta, "hi" => b"\x1b[32;105mhi\x1b[0m");
    test_color!(blue_on_br_ylw:    Blue,    BrYellow,  "hi" => b"\x1b[34;103mhi\x1b[0m");
    test_color!(br_red_on_br_wht:  BrRed,   BrWhite,   "hi" => b"\x1b[91;107mhi\x1b[0m");
    test_color!(br_blk_on_br_cyan: BrBlack, BrCyan,    "hi" => b"\x1b[90;106mhi\x1b[0m");
    test_color!(br_grn_on_br_purp: BrGreen, BrMagenta, "hi" => b"\x1b[92;105mhi\x1b[0m");
    test_color!(br_blue_on_br_ylw: BrBlue,  BrYellow,  "hi" => b"\x1b[94;103mhi\x1b[0m");

    // Test all color256 foreground and background combinations
    #[test]
    fn test_all_color256_fg_and_bg_values() {
        let mut output = Vec::new();
        let mut expected = Vec::new();

        for (fg_color, bg_color) in (0..=255).zip(0..=255) {
            write_color256!(&mut output, fg_color, bg_color, "hi");
            expected.extend(format!("\x1b[38;5;{fg_color};48;5;{bg_color}mhi\x1b[0m").as_bytes());
        }

        assert_eq!(output.as_slice(), expected.as_slice());
    }

    // RGB tests
    test_rgb!(rgb_1: (192, 123, 23), (231, 12, 73), "hi" =>
        b"\x1b[38;2;192;123;23;48;2;231;12;73mhi\x1b[0m"
    );
    test_rgb!(rgb_2: (92, 13, 239), (132, 1, 177), "hi" =>
        b"\x1b[38;2;92;13;239;48;2;132;1;177mhi\x1b[0m"
    );
    test_rgb!(rgb_3: (255, 0, 255), (0, 255, 0), "hi" =>
        b"\x1b[38;2;255;0;255;48;2;0;255;0mhi\x1b[0m"
    );
}
