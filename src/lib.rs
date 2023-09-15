//! `color_macros`
//! 
//! [![Rust](https://github.com/ryanv404/rust_color_macros/actions/workflows/rust.yml/badge.svg)](https://github.com/ryanv404/rust_color_macros/actions/workflows/rust.yml)
//! 
//! A simple Rust library containing easy-to-use macros for writing colored
//! text to a buffer or to the terminal.
//! 
//! Features
//! ========
//! 
//! Set foreground and background colors using simple to use macros that are
//! reminiscent of the most popular Rust standard library macros.
//! 
//! * `write_styled`/`write_color256`/`write_rgb` write colored text to a buffer.
//! * `print_styled`/`print_color256`/`print_rgb` print colored text to stdout.
//! * `eprint_styled`/`eprint_color256`/`eprint_rgb` print colored text to stderr.
//! 
//! Note that each macro has a newline version (e.g. `println_styled`).
//! 
//! Usage
//! =====
//! 
//! Basic Color Mode
//! ----------------
//! 
//! Foreground and background colors are set using an "X on Y" style string, where
//! X and Y are each one of the following color options:
//! 
//! black, red, green, yellow, blue, magenta, cyan, white, bright black, bright red,
//! bright green, bright yellow, bright blue, bright magenta, bright cyan,
//! bright white, or current.
//! 
//! Entering "X" alone (e.g. "red") for the style string sets the foreground color
//! without altering the background, while entering "on Y" (e.g. "on red") for the
//! style string sets the background color without altering the foreground.
//! 
//! 256-Color Mode
//! --------------
//! 
//! Foreground and background colors are set using color numbers ranging from 0 - 255.
//! 
//! 24-Bit RGB Color Mode
//! ---------------------
//! 
//! Foreground and background colors are set using tuples containing red, green, and
//! blue color values, each ranging from 0 - 255.
//!

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
    /// 256-color mode colors
    Color256(u8),
    /// 24-bit RGB colors
    Rgb(u8, u8, u8),
    /// Current color value
    Current,
    /// Reset color attributes
    Reset,
}

impl Color {
    /// Returns true if this `Color` instance is `Color::Current`.
    #[must_use]
    pub fn is_current(&self) -> bool {
        *self == Self::Current
    }

    /// Returns a semicolon separator if self and other are not `Current`.
    /// Else it returns an empty string.
    #[must_use]
    pub fn get_separator(&self, other_is_current: bool) -> String {
        if !self.is_current() && !other_is_current {
            ";".to_string()
        } else {
            String::new()
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value {
            "black" => Self::Black,
            "red" => Self::Red,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "blue" => Self::Blue,
            "magenta" => Self::Magenta,
            "cyan" => Self::Cyan,
            "white" => Self::White,
            "reset" => Self::Reset,
            _ => Self::Current,
        }
    }
}

/// Wrapper that represents a regular foreground color.
pub struct Fg;

impl Fg {
    /// Returns the numeric portion of the foreground ANSI color code.
    #[must_use]
    pub fn get_ansi_code(color: &Color) -> String {
        match *color {
            Color::Black => "30".to_string(),
            Color::Red => "31".to_string(),
            Color::Green => "32".to_string(),
            Color::Yellow => "33".to_string(),
            Color::Blue => "34".to_string(),
            Color::Magenta => "35".to_string(),
            Color::Cyan => "36".to_string(),
            Color::White => "37".to_string(),
            Color::Reset => "0".to_string(),
            Color::Color256(num) => format!("38;5;{num}"),
            Color::Rgb(r, g, b) => format!("38;2;{r};{g};{b}"),
            Color::Current => String::new(),
        }
    }
}

/// Wrapper that represents a bright foreground color.
pub struct FgBright;

impl FgBright {
    /// Returns the numeric portion of the foreground ANSI color code.
    #[must_use]
    pub fn get_ansi_code(color: &Color) -> String {
        match *color {
            Color::Black => "90".to_string(),
            Color::Red => "91".to_string(),
            Color::Green => "92".to_string(),
            Color::Yellow => "93".to_string(),
            Color::Blue => "94".to_string(),
            Color::Magenta => "95".to_string(),
            Color::Cyan => "96".to_string(),
            Color::White => "97".to_string(),
            Color::Reset => "0".to_string(),
            Color::Color256(num) => format!("38;5;{num}"),
            Color::Rgb(r, g, b) => format!("38;2;{r};{g};{b}"),
            Color::Current => String::new(),
        }
    }
}

/// Wrapper that represents a regular background color.
pub struct Bg;

impl Bg {
    /// Returns the numeric portion of the background ANSI color code.
    #[must_use]
    pub fn get_ansi_code(color: &Color) -> String {
        match *color {
            Color::Black => "40".to_string(),
            Color::Red => "41".to_string(),
            Color::Green => "42".to_string(),
            Color::Yellow => "43".to_string(),
            Color::Blue => "44".to_string(),
            Color::Magenta => "45".to_string(),
            Color::Cyan => "46".to_string(),
            Color::White => "47".to_string(),
            Color::Reset => "0".to_string(),
            Color::Color256(num) => format!("48;5;{num}"),
            Color::Rgb(r, g, b) => format!("48;2;{r};{g};{b}"),
            Color::Current => String::new(),
        }
    }
}

/// Wrapper that represents a bright background color.
pub struct BgBright;

impl BgBright {
    /// Returns the numeric portion of the foreground ANSI color code.
    #[must_use]
    pub fn get_ansi_code(color: &Color) -> String {
        match *color {
            Color::Black => "100".to_string(),
            Color::Red => "101".to_string(),
            Color::Green => "102".to_string(),
            Color::Yellow => "103".to_string(),
            Color::Blue => "104".to_string(),
            Color::Magenta => "105".to_string(),
            Color::Cyan => "106".to_string(),
            Color::White => "107".to_string(),
            Color::Reset => "0".to_string(),
            Color::Color256(num) => format!("48;5;{num}"),
            Color::Rgb(r, g, b) => format!("48;2;{r};{g};{b}"),
            Color::Current => String::new(),
        }
    }
}

/// Parses the style expression in `x_styled!()` macros and returns a string
/// containing the ansi color code(s).
#[macro_export]
macro_rules! parse_colors {
    ($style:expr) => {{
        use $crate::{Color, Bg, Fg, BgBright, FgBright};

        let style_args: Vec<String> = $style.split(' ')
            .map(|s| {
                let s = s.trim();
                let mut new_str = s.to_string();
                new_str.make_ascii_lowercase();
                new_str
            })
            .collect();

        let style_strs: Vec<&str> = style_args.iter()
            .map(|s| s.as_str())
            .collect();

        let mut ansi_code = String::new();

        match style_strs.len() {
            // Regular foreground color (e.g. "red")
            1 => {
                ansi_code.push_str(&Fg::get_ansi_code(&Color::from(style_strs[0])));
                ansi_code.push('m');
            },
            2 => match style_strs[..] {
                // Regular background color (e.g. "on red")
                ["on", bg_color] => {
                    ansi_code.push_str(&Bg::get_ansi_code(&Color::from(bg_color)));
                    ansi_code.push('m');
                },
                // Bright foreground color (e.g. "bright red")
                ["bright", fg_br_color] => {
                    ansi_code.push_str(&FgBright::get_ansi_code(&Color::from(fg_br_color)));
                    ansi_code.push('m');
                },
                _ => {},
            },
            3 => match style_strs[..] {
                // Bright background color (e.g. "on bright red")
                ["on", "bright", bg_br_color] => {
                    ansi_code.push_str(&BgBright::get_ansi_code(&Color::from(bg_br_color)));
                    ansi_code.push('m');
                },
                // Regular foreground color and regular background color (e.g. "green on red")
                [fg_color, "on", bg_color] => {
                    ansi_code.push_str(&Fg::get_ansi_code(&Color::from(fg_color)));
                    if fg_color != "current" && bg_color != "current" { ansi_code.push(';') }
                    ansi_code.push_str(&Bg::get_ansi_code(&Color::from(bg_color)));
                    ansi_code.push('m');
                },
                _ => {},
            },
            4 => match style_strs[..] {
                // Regular foreground color and bright background color (e.g. "green on bright red")
                [fg_color, "on", "bright", bg_br_color] => {
                    ansi_code.push_str(&Fg::get_ansi_code(&Color::from(fg_color)));
                    if fg_color != "current" && bg_br_color != "current" { ansi_code.push(';') }
                    ansi_code.push_str(&BgBright::get_ansi_code(&Color::from(bg_br_color)));
                    ansi_code.push('m');
                },
                // Bright foreground color and regular background color (e.g. "bright green on red")
                ["bright", fg_br_color, "on", bg_color] => {
                    ansi_code.push_str(&FgBright::get_ansi_code(&Color::from(fg_br_color)));
                    if fg_br_color != "current" && bg_color != "current" { ansi_code.push(';') }
                    ansi_code.push_str(&Bg::get_ansi_code(&Color::from(bg_color)));
                    ansi_code.push('m');
                },
                _ => {},
            },
            5 => match style_strs[..] {
                // Bright foreground color and bright background color (e.g. "bright green on bright red")
                ["bright", fg_br_color, "on", "bright", bg_br_color] => {
                    ansi_code.push_str(&FgBright::get_ansi_code(&Color::from(fg_br_color)));
                    if fg_br_color != "current" && bg_br_color != "current" { ansi_code.push(';') }
                    ansi_code.push_str(&BgBright::get_ansi_code(&Color::from(bg_br_color)));
                    ansi_code.push('m');
                },
                _ => {},
            },
            _ => {},
        }
        ansi_code
    }};
}

/// Write color text to a buffer.
///
/// # Arguments
/// * `$buffer` - The destination buffer.
/// * `$style` - "X on Y" style string (e.g. "bright green on white".
/// * `$text` - The text to write.
///
/// # Example
///
/// ```rust
/// use color_macros::write_styled;
///
/// let mut buffer: Vec<u8> = vec![];
///
/// // Write text whose color is the currently set foreground color on
/// // a bright magenta background to a buffer.
/// write_styled!(&mut buffer, "current on bright magenta", "test");
///
/// assert_eq!(buffer.as_slice(), b"\x1b[105mtest\x1b[0m");
/// ```
#[macro_export]
macro_rules! write_styled {
    ($buffer:expr, $style:expr, $text:expr) => {{
        use std::io::Write;
        write!(
            $buffer,
            "\u{001b}[{}{}\u{001b}[0m",
            $crate::parse_colors!($style),
            $text
        )
        .unwrap();
    }};
}

/// Same as `write_styled!()` but with a newline appended at the end.
#[macro_export]
macro_rules! writeln_styled {
    ($buffer:expr, $style:expr, $text:expr) => {{
        use std::io::Write;
        write!(
            $buffer,
            "\u{001b}[{}{}\u{001b}[0m\n",
            $crate::parse_colors!($style),
            $text
        )
        .unwrap();
    }};
}

/// Print color text to stdout.
///
/// # Arguments
/// * `$style` - "X on Y" style string (e.g. "bright green on white".
/// * `$text` - The text to print to stdout.
///
/// # Example
///
/// ```rust
/// use color_macros::print_styled;
///
/// // Prints bright green text on a black background to stdout.
/// print_styled!("bright green on black", "test");
/// ```
#[macro_export]
macro_rules! print_styled {
    ($style:expr, $text:expr) => {{
        use std::io::Write;
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{}{}\u{001b}[0m",
            $crate::parse_colors!($style),
            $text
        )
        .unwrap();
        stdout.flush().unwrap();
    }};
}

/// Same as `print_styled!()` but with a newline appended at the end.
#[macro_export]
macro_rules! println_styled {
    ($style:expr, $text:expr) => {{
        use std::io::Write;
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{}{}\u{001b}[0m\n",
            $crate::parse_colors!($style),
            $text
        )
        .unwrap();
        stdout.flush().unwrap();
    }};
}

/// Print color text to stderr.
///
/// # Arguments
/// * `$style` - "X on Y" style string (e.g. "bright green on white".
/// * `$text` - The text to print to stderr.
///
/// # Example
///
/// ```rust
/// use color_macros::eprint_styled;
///
/// // Prints bright red text on a white background to stderr.
/// eprint_styled!("bright red on white", "test");
/// ```
#[macro_export]
macro_rules! eprint_styled {
    ($style:expr, $text:expr) => {{
        use std::io::Write;
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{}{}\u{001b}[0m",
            $crate::parse_colors!($style),
            $text
        )
        .unwrap();
        stderr.flush().unwrap();
    }};
}

/// Same as `eprint_styled!()` but with a newline appended at the end.
#[macro_export]
macro_rules! eprintln_styled {
    ($style:expr, $text:expr) => {{
        use std::io::Write;
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{}{}\u{001b}[0m\n",
            $crate::parse_colors!($style),
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
/// use color_macros::write_color256;
///
/// let mut buffer: Vec<u8> = vec![];
///
/// // Write red text on a white background to a buffer.
/// write_color256!(&mut buffer, 196, 255, "test");
///
/// assert_eq!(buffer.as_slice(), b"\x1b[38;5;196;48;5;255mtest\x1b[0m");
/// ```
#[macro_export]
macro_rules! write_color256 {
    ($buffer:expr, $fg_num:expr, $bg_num:expr, $text:expr) => {{
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        write!(
            $buffer,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            Fg::get_ansi_code(&Color::Color256($fg_num)),
            Bg::get_ansi_code(&Color::Color256($bg_num)),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    }};
}

/// Same as `write_color256!()` but with a newline appended at the end.
#[macro_export]
macro_rules! writeln_color256 {
    ($buffer:expr, $fg_num:expr, $bg_num:expr, $text:expr) => {{
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        write!(
            $buffer,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            Fg::get_ansi_code(&Color::Color256($fg_num)),
            Bg::get_ansi_code(&Color::Color256($bg_num)),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    }};
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
/// use color_macros::print_color256;
///
/// // Prints red text on a white background to stdout.
/// print_color256!(196, 255, "test");
/// ```
#[macro_export]
macro_rules! print_color256 {
    ($fg_num:expr, $bg_num:expr, $text:expr) => {{
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            Fg::get_ansi_code(&Color::Color256($fg_num)),
            Bg::get_ansi_code(&Color::Color256($bg_num)),
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
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            Fg::get_ansi_code(&Color::Color256($fg_num)),
            Bg::get_ansi_code(&Color::Color256($bg_num)),
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
/// use color_macros::eprint_color256;
///
/// // Prints red text on a white background to stderr.
/// eprint_color256!(196, 255, "test");
/// ```
#[macro_export]
macro_rules! eprint_color256 {
    ($fg_num:expr, $bg_num:expr, $text:expr) => {{
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            Fg::get_ansi_code(&Color::Color256($fg_num)),
            Bg::get_ansi_code(&Color::Color256($bg_num)),
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
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            Fg::get_ansi_code(&Color::Color256($fg_num)),
            Bg::get_ansi_code(&Color::Color256($bg_num)),
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
    ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {{
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        write!(
            $buffer,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            Fg::get_ansi_code(&Color::Rgb($fg_r, $fg_g, $fg_b)),
            Bg::get_ansi_code(&Color::Rgb($bg_r, $bg_g, $bg_b)),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    }};
}

/// Same as `write_rgb!()` but with a newline appended at the end.
#[macro_export]
macro_rules! writeln_rgb {
    ($buffer:expr, ($fg_r:expr, $fg_g:expr, $fg_b:expr),
    ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {{
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        write!(
            $buffer,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            Fg::get_ansi_code(&Color::Rgb($fg_r, $fg_g, $fg_b)),
            Bg::get_ansi_code(&Color::Rgb($bg_r, $bg_g, $bg_b)),
            $text
        )
        .unwrap();
        $buffer.flush().unwrap();
    }};
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
/// use color_macros::print_rgb;
///
/// // Print red text on a white background to stdout.
/// print_rgb!((211, 0, 0), (255, 255, 255), "test");
/// ```
#[macro_export]
macro_rules! print_rgb {
    (($fg_r:expr, $fg_g:expr, $fg_b:expr),
     ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {{
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            Fg::get_ansi_code(&Color::Rgb($fg_r, $fg_g, $fg_b)),
            Bg::get_ansi_code(&Color::Rgb($bg_r, $bg_g, $bg_b)),
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
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        let mut stdout = std::io::stdout().lock();
        write!(
            &mut stdout,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            Fg::get_ansi_code(&Color::Rgb($fg_r, $fg_g, $fg_b)),
            Bg::get_ansi_code(&Color::Rgb($bg_r, $bg_g, $bg_b)),
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
/// use color_macros::eprint_rgb;
///
/// // Print red text on a white background to stderr.
/// eprint_rgb!((211, 0, 0), (255, 255, 255), "test");
/// ```
#[macro_export]
macro_rules! eprint_rgb {
    (($fg_r:expr, $fg_g:expr, $fg_b:expr),
     ($bg_r:expr, $bg_g:expr, $bg_b:expr), $text:expr) => {{
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{};{}m{}\u{001b}[0m",
            Fg::get_ansi_code(&Color::Rgb($fg_r, $fg_g, $fg_b)),
            Bg::get_ansi_code(&Color::Rgb($bg_r, $bg_g, $bg_b)),
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
        use std::io::Write;
        use $crate::{Bg, Color, Fg};
        let mut stderr = std::io::stderr().lock();
        write!(
            &mut stderr,
            "\u{001b}[{};{}m{}\u{001b}[0m\n",
            Fg::get_ansi_code(&Color::Rgb($fg_r, $fg_g, $fg_b)),
            Bg::get_ansi_code(&Color::Rgb($bg_r, $bg_g, $bg_b)),
            $text
        )
        .unwrap();
        stderr.flush().unwrap();
    }};
}

#[cfg(test)]
mod tests {
    use super::{write_color256, write_rgb, write_styled};

    // Test all color256 foreground and background color combinations
    #[test]
    fn test_all_color256_fg_and_bg_color_combos() {
        let mut output = Vec::new();
        let mut expected = Vec::new();

        for (fg_color, bg_color) in (0..=255).zip(0..=255) {
            write_color256!(&mut output, fg_color, bg_color, "hi");
            expected.extend(
                format!("\x1b[38;5;{fg_color};48;5;{bg_color}mhi\x1b[0m").as_bytes()
            );
        }
        assert!(&output[..] == &expected[..]);
    }

    // Test RGB foreground colors
    #[test]
    fn test_rgb_fg_colors() {
        let mut output = Vec::new();
        let mut expected = Vec::new();

        for r in (0..=255).step_by(10) {
            for g in (0..=255).step_by(10) {
                for b in (0..=255).step_by(10) {
                    write_rgb!(&mut output, (r, g, b), (255, 255, 255), "hi");
                    expected.extend(
                        format!("\x1b[38;2;{r};{g};{b};48;2;255;255;255mhi\x1b[0m").as_bytes()
                    );
                }
            }
        }
        assert!(&output[..] == &expected[..]);
    }

    // Test RGB background colors
    #[test]
    fn test_rgb_bg_colors() {
        let mut output = Vec::new();
        let mut expected = Vec::new();

        for r in (0..=255).step_by(10) {
            for g in (0..=255).step_by(10) {
                for b in (0..=255).step_by(10) {
                    write_rgb!(&mut output, (255, 255, 255), (r, g, b), "hi");
                    expected.extend(
                        format!("\x1b[38;2;255;255;255;48;2;{r};{g};{b}mhi\x1b[0m").as_bytes()
                    );
                }
            }
        }
        assert!(&output[..] == &expected[..]);
    }

    // Test all write_styled foreground and background color combinations
    #[test]
    fn test_all_write_styled_fg_and_bg_color_combos() {
        const FG_COLORS: [(&str, &str); 17] = [
            ("current", ""),
            ("black",   "30"),
            ("red",     "31"),
            ("green",   "32"),
            ("yellow",  "33"),
            ("blue",    "34"),
            ("magenta", "35"),
            ("cyan",    "36"),
            ("white",   "37"),
            ("bright black",   "90"),
            ("bright red",     "91"),
            ("bright green",   "92"),
            ("bright yellow",  "93"),
            ("bright blue",    "94"),
            ("bright magenta", "95"),
            ("bright cyan",    "96"),
            ("bright white",   "97"),
        ];

        const BG_COLORS: [(&str, &str); 17] = [
            ("current", ""),
            ("black",   "40"),
            ("red",     "41"),
            ("green",   "42"),
            ("yellow",  "43"),
            ("blue",    "44"),
            ("magenta", "45"),
            ("cyan",    "46"),
            ("white",   "47"),
            ("bright black",   "100"),
            ("bright red",     "101"),
            ("bright green",   "102"),
            ("bright yellow",  "103"),
            ("bright blue",    "104"),
            ("bright magenta", "105"),
            ("bright cyan",    "106"),
            ("bright white",   "107"),
        ];

        let mut output = Vec::new();
        let mut expected = Vec::new();

        for fg_idx in 0..FG_COLORS.len() {
            for bg_idx in (0..BG_COLORS.len()).rev() {
                let fg_color = FG_COLORS[fg_idx].0;
                let bg_color = BG_COLORS[bg_idx].0;

                let fg_code = FG_COLORS[fg_idx].1;
                let bg_code = BG_COLORS[bg_idx].1;

                let style = format!("{fg_color} on {bg_color}");
                write_styled!(&mut output, style, style);
                expected.extend(
                    format!(
                        "\x1b[{}{}{}m{}\x1b[0m",
                        fg_code,
                        if fg_color != "current" && bg_color != "current" {
                            ";"
                        } else {
                            ""
                        },
                        bg_code,
                        style
                    )
                    .as_bytes(),
                );
            }
        }
        assert!(&output[..] == &expected[..]);
    }
}
