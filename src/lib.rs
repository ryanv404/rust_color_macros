//! `color_macros`
//!
//! A library of simple macros for writing colored text to a buffer or to the terminal.
//!
//!
//! ## Features
//! Set foreground and background colors using macros that are similar to standard Rust
//! macros.

#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]

/// Color options
#[derive(PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Color256(u8),
    Rgb(u8, u8, u8),
    Current,
    Reset,
}

/// Write formatted text to a buffer (no new line is appended at the end).
#[macro_export]
macro_rules! write_styled {
    ($dst:expr, Color256($fg:expr), Color256($bg:expr), $msg:expr) => {
        $dst.write_all(
            format!(
                "\u{001b}[{};{}m{}\u{001b}[0m",
                $crate::Color::Color256($fg).to_fg_code(),
                $crate::Color::Color256($bg).to_bg_code(),
                $msg
            )
            .as_bytes(),
        )
        .unwrap();
        $dst.flush().unwrap();
    };
    ($dst:expr, Rgb($fg_r:expr, $fg_g:expr, $fg_b:expr),
    Rgb($bg_r:expr, $bg_g:expr, $bg_b:expr), $msg:expr) => {
        $dst.write_all(
            format!(
                "\u{001b}[{};{}m{}\u{001b}[0m",
                $crate::Color::Rgb($fg_r, $fg_g, $fg_b).to_fg_code(),
                $crate::Color::Rgb($bg_r, $bg_g, $bg_b).to_bg_code(),
                $msg
            )
            .as_bytes(),
        )
        .unwrap();
        $dst.flush().unwrap();
    };
    ($dst:expr, $fg:expr, $bg:expr, $msg:expr) => {
        $dst.write_all(
            format!(
                "\u{001b}[{}{}{}m{}\u{001b}[0m",
                $fg.to_fg_code(),
                $fg.get_separator(&$bg),
                $bg.to_bg_code(),
                $msg
            )
            .as_bytes(),
        )
        .unwrap();
        $dst.flush().unwrap();
    };
}

/// Write formatted text to a buffer, a new line is appended at the end.
#[macro_export]
macro_rules! writeln_styled {
    ($dst:expr, Color256($fg:expr), Color256($bg:expr), $msg:expr) => {
        $dst.write_all(
            format!(
                "\u{001b}[{};{}m{}\u{001b}[0m\n",
                $crate::Color::Color256($fg).to_fg_code(),
                $crate::Color::Color256($bg).to_bg_code(),
                $msg
            )
            .as_bytes(),
        )
        .unwrap();
        $dst.flush().unwrap();
    };
    ($dst:expr, Rgb($fg_r:expr, $fg_g:expr, $fg_b:expr),
    Rgb($bg_r:expr, $bg_g:expr, $bg_b:expr), $msg:expr) => {
        $dst.write_all(
            format!(
                "\u{001b}[{};{}m{}\u{001b}[0m\n",
                $crate::Color::Rgb($fg_r, $fg_g, $fg_b).to_fg_code(),
                $crate::Color::Rgb($bg_r, $bg_g, $bg_b).to_bg_code(),
                $msg
            )
            .as_bytes(),
        )
        .unwrap();
        $dst.flush().unwrap();
    };
    ($dst:expr, $fg:expr, $bg:expr, $msg:expr) => {
        $dst.write_all(
            format!(
                "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
                $fg.to_fg_code(),
                $fg.get_separator(&$bg),
                $bg.to_bg_code(),
                $msg
            )
            .as_bytes(),
        )
        .unwrap();
        $dst.flush().unwrap();
    };
}

/// Write formatted text to stdout (no new line is appended at the end).
#[macro_export]
macro_rules! print_styled {
    (Color256($fg:expr), Color256($bg:expr), $msg:expr) => {
        {
            let mut stdout = std::io::stdout().lock();
            stdout
                .write_all(
                    format!(
                        "\u{001b}[{};{}m{}\u{001b}[0m",
                        $crate::Color::Color256($fg).to_fg_code(),
                        $crate::Color::Color256($bg).to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stdout.flush().unwrap();
        }
    };
    (Rgb($fg_r:expr, $fg_g:expr, $fg_b:expr),
     Rgb($bg_r:expr, $bg_g:expr, $bg_b:expr), $msg:expr) => {
        {
            let mut stdout = std::io::stdout().lock();
            stdout
                .write_all(
                    format!(
                        "\u{001b}[{};{}m{}\u{001b}[0m",
                        $crate::Color::Rgb($fg_r, $fg_g, $fg_b).to_fg_code(),
                        $crate::Color::Rgb($bg_r, $bg_g, $bg_b).to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stdout.flush().unwrap();
        }
    };
    ($fg:expr, $bg:expr, $msg:expr) => {
        {
            let mut stdout = std::io::stdout().lock();
            stdout
                .write_all(
                    format!(
                        "\u{001b}[{}{}{}m{}\u{001b}[0m",
                        $fg.to_fg_code(),
                        $fg.get_separator(&$bg),
                        $bg.to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stdout.flush().unwrap();
        }
    };
}

/// Write formatted text to stdout, a new line is appended at the end.
#[macro_export]
macro_rules! println_styled {
    (Color256($fg:expr), Color256($bg:expr), $msg:expr) => {
        {
            let mut stdout = std::io::stdout().lock();
            stdout
                .write_all(
                    format!(
                        "\u{001b}[{};{}m{}\u{001b}[0m\n",
                        $crate::Color::Color256($fg).to_fg_code(),
                        $crate::Color::Color256($bg).to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stdout.flush().unwrap();
        }
    };
    (Rgb($fg_r:expr, $fg_g:expr, $fg_b:expr),
     Rgb($bg_r:expr, $bg_g:expr, $bg_b:expr), $msg:expr) => {
        {
            let mut stdout = std::io::stdout().lock();
            stdout
                .write_all(
                    format!(
                        "\u{001b}[{};{}m{}\u{001b}[0m\n",
                        $crate::Color::Rgb($fg_r, $fg_g, $fg_b).to_fg_code(),
                        $crate::Color::Rgb($bg_r, $bg_g, $bg_b).to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stdout.flush().unwrap();
        }
    };
    ($fg:expr, $bg:expr, $msg:expr) => {
        {
            let mut stdout = std::io::stdout().lock();
            stdout
                .write_all(
                    format!(
                        "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
                        $fg.to_fg_code(),
                        $fg.get_separator(&$bg),
                        $bg.to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stdout.flush().unwrap();
        }
    };
}

/// Write formatted text to stderr (no new line is appended at the end).
#[macro_export]
macro_rules! eprint_styled {
    (Color256($fg:expr), Color256($bg:expr), $msg:expr) => {
        {
            let mut stderr = std::io::stderr().lock();
            stderr
                .write_all(
                    format!(
                        "\u{001b}[{};{}m{}\u{001b}[0m",
                        $crate::Color::Color256($fg).to_fg_code(),
                        $crate::Color::Color256($bg).to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stderr.flush().unwrap();
        }
    };
    (Rgb($fg_r:expr, $fg_g:expr, $fg_b:expr),
     Rgb($bg_r:expr, $bg_g:expr, $bg_b:expr), $msg:expr) => {
        {
            let mut stderr = std::io::stderr().lock();
            stderr
                .write_all(
                    format!(
                        "\u{001b}[{};{}m{}\u{001b}[0m",
                        $crate::Color::Rgb($fg_r, $fg_g, $fg_b).to_fg_code(),
                        $crate::Color::Rgb($bg_r, $bg_g, $bg_b).to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stderr.flush().unwrap();
        }
    };
    ($fg:expr, $bg:expr, $msg:expr) => {
        {
            let mut stderr = std::io::stderr().lock();
            stderr
                .write_all(
                    format!(
                        "\u{001b}[{}{}{}m{}\u{001b}[0m",
                        $fg.to_fg_code(),
                        $fg.get_separator(&$bg),
                        $bg.to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stderr.flush().unwrap();
        }
    };
}

/// Write formatted text to stderr, a new line is appended at the end.
#[macro_export]
macro_rules! eprintln_styled {
    (Color256($fg:expr), Color256($bg:expr), $msg:expr) => {
        {
            let mut stderr = std::io::stderr().lock();
            stderr
                .write_all(
                    format!(
                        "\u{001b}[{};{}m{}\u{001b}[0m\n",
                        $crate::Color::Color256($fg).to_fg_code(),
                        $crate::Color::Color256($bg).to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stderr.flush().unwrap();
        }
    };
    (Rgb($fg_r:expr, $fg_g:expr, $fg_b:expr),
     Rgb($bg_r:expr, $bg_g:expr, $bg_b:expr), $msg:expr) => {
        {
            let mut stderr = std::io::stderr().lock();
            stderr
                .write_all(
                    format!(
                        "\u{001b}[{};{}m{}\u{001b}[0m\n",
                        $crate::Color::Rgb($fg_r, $fg_g, $fg_b).to_fg_code(),
                        $crate::Color::Rgb($bg_r, $bg_g, $bg_b).to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stderr.flush().unwrap();
        }
    };
    ($fg:expr, $bg:expr, $msg:expr) => {
        {
            let mut stderr = std::io::stderr().lock();
            stderr
                .write_all(
                    format!(
                        "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
                        $fg.to_fg_code(),
                        $fg.get_separator(&$bg),
                        $bg.to_bg_code(),
                        $msg
                    )
                    .as_bytes(),
                )
                .unwrap();
            stderr.flush().unwrap();
        }
    };
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
    pub fn to_fg_code(&self) -> String {
        match *self {
            Self::Black => "30".to_string(),
            Self::Red => "31".to_string(),
            Self::Green => "32".to_string(),
            Self::Yellow => "33".to_string(),
            Self::Blue => "34".to_string(),
            Self::Magenta => "35".to_string(),
            Self::Cyan => "36".to_string(),
            Self::White => "37".to_string(),
            Self::BrightBlack => "90".to_string(),
            Self::BrightRed => "91".to_string(),
            Self::BrightGreen => "92".to_string(),
            Self::BrightYellow => "93".to_string(),
            Self::BrightBlue => "94".to_string(),
            Self::BrightMagenta => "95".to_string(),
            Self::BrightCyan => "96".to_string(),
            Self::BrightWhite => "97".to_string(),
            Self::Color256(num) => format!("38;5;{num}"),
            Self::Rgb(r, g, b) => format!("38;2;{r};{g};{b}"),
            Self::Current => String::new(),
            Self::Reset => "0".to_string(),
        }
    }

    /// Returns the numeric portion of the background ANSI color code.
    #[must_use]
    pub fn to_bg_code(&self) -> String {
        match *self {
            Self::Black => "40".to_string(),
            Self::Red => "41".to_string(),
            Self::Green => "42".to_string(),
            Self::Yellow => "43".to_string(),
            Self::Blue => "44".to_string(),
            Self::Magenta => "45".to_string(),
            Self::Cyan => "46".to_string(),
            Self::White => "47".to_string(),
            Self::BrightBlack => "100".to_string(),
            Self::BrightRed => "101".to_string(),
            Self::BrightGreen => "102".to_string(),
            Self::BrightYellow => "103".to_string(),
            Self::BrightBlue => "104".to_string(),
            Self::BrightMagenta => "105".to_string(),
            Self::BrightCyan => "106".to_string(),
            Self::BrightWhite => "107".to_string(),
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

    use super::write_styled;
    use super::Color::*;

    macro_rules! test_color {
        ($name:ident: Color256($fg:tt), Color256($bg:tt),
         $msg:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let mut output: Vec<u8> = vec![];
                write_styled!(&mut output, Color256($fg), Color256($bg), $msg);
                assert_eq!(output.as_slice(), $expected);
            }
        };
        ($name:ident:
         Rgb($fg_r:tt, $fg_g:tt, $fg_b:tt),
         Rgb($bg_r:tt, $bg_g:tt, $bg_b:tt),
         $msg:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let mut output: Vec<u8> = vec![];
                write_styled!(
                    &mut output,
                    Rgb($fg_r, $fg_g, $fg_b),
                    Rgb($bg_r, $bg_g, $bg_b),
                    $msg
                );
                assert_eq!(output.as_slice(), $expected);
            }
        };
        ($name:ident: $fg:tt, $bg:tt, $msg:literal => $expected:literal) => {
            #[test]
            fn $name() {
                let mut output: Vec<u8> = vec![];
                write_styled!(&mut output, $fg, $bg, $msg);
                assert_eq!(output.as_slice(), $expected);
            }
        };
    }

    // Basic foreground color tests
    test_color!(black: Black, Current, "hi" => b"\x1b[30mhi\x1b[0m");
    test_color!(red: Red, Current, "hi" => b"\x1b[31mhi\x1b[0m");
    test_color!(green: Green, Current, "hi" => b"\x1b[32mhi\x1b[0m");
    test_color!(yellow: Yellow, Current, "hi" => b"\x1b[33mhi\x1b[0m");
    test_color!(blue: Blue, Current, "hi" => b"\x1b[34mhi\x1b[0m");
    test_color!(magenta: Magenta, Current, "hi" => b"\x1b[35mhi\x1b[0m");
    test_color!(cyan: Cyan, Current, "hi" => b"\x1b[36mhi\x1b[0m");
    test_color!(white: White, Current, "hi" => b"\x1b[37mhi\x1b[0m");
    test_color!(bright_black: BrightBlack, Current, "hi" => b"\x1b[90mhi\x1b[0m");
    test_color!(bright_red: BrightRed, Current, "hi" => b"\x1b[91mhi\x1b[0m");
    test_color!(bright_green: BrightGreen, Current, "hi" => b"\x1b[92mhi\x1b[0m");
    test_color!(bright_yellow: BrightYellow, Current, "hi" => b"\x1b[93mhi\x1b[0m");
    test_color!(bright_blue: BrightBlue, Current, "hi" => b"\x1b[94mhi\x1b[0m");
    test_color!(bright_magenta: BrightMagenta, Current, "hi" => b"\x1b[95mhi\x1b[0m");
    test_color!(bright_cyan: BrightCyan, Current, "hi" => b"\x1b[96mhi\x1b[0m");
    test_color!(bright_white: BrightWhite, Current, "hi" => b"\x1b[97mhi\x1b[0m");

    // Basic background color tests
    test_color!(bg_black: Current, Black, "hi" => b"\x1b[40mhi\x1b[0m");
    test_color!(bg_red: Current, Red, "hi" => b"\x1b[41mhi\x1b[0m");
    test_color!(bg_green: Current, Green, "hi" => b"\x1b[42mhi\x1b[0m");
    test_color!(bg_yellow: Current, Yellow, "hi" => b"\x1b[43mhi\x1b[0m");
    test_color!(bg_blue: Current, Blue, "hi" => b"\x1b[44mhi\x1b[0m");
    test_color!(bg_magenta: Current, Magenta, "hi" => b"\x1b[45mhi\x1b[0m");
    test_color!(bg_cyan: Current, Cyan, "hi" => b"\x1b[46mhi\x1b[0m");
    test_color!(bg_white: Current, White, "hi" => b"\x1b[47mhi\x1b[0m");
    test_color!(bg_bright_black: Current, BrightBlack, "hi" => b"\x1b[100mhi\x1b[0m");
    test_color!(bg_bright_red: Current, BrightRed, "hi" => b"\x1b[101mhi\x1b[0m");
    test_color!(bg_bright_green: Current, BrightGreen, "hi" => b"\x1b[102mhi\x1b[0m");
    test_color!(bg_bright_yellow: Current, BrightYellow, "hi" => b"\x1b[103mhi\x1b[0m");
    test_color!(bg_bright_blue: Current, BrightBlue, "hi" => b"\x1b[104mhi\x1b[0m");
    test_color!(bg_bright_magenta: Current, BrightMagenta, "hi" => b"\x1b[105mhi\x1b[0m");
    test_color!(bg_bright_cyan: Current, BrightCyan, "hi" => b"\x1b[106mhi\x1b[0m");
    test_color!(bg_bright_white: Current, BrightWhite, "hi" => b"\x1b[107mhi\x1b[0m");

    // Color256 tests
    test_color!(color256_1: Color256(123), Color256(52), "hi" => b"\x1b[38;5;123;48;5;52mhi\x1b[0m");
    test_color!(color256_2: Color256(12), Color256(255), "hi" => b"\x1b[38;5;12;48;5;255mhi\x1b[0m");

    // RGB tests
    test_color!(rgb_1:
        Rgb(192, 123, 23),
        Rgb(231, 12, 73),
        "hi" => b"\x1b[38;2;192;123;23;48;2;231;12;73mhi\x1b[0m"
    );
    test_color!(rgb_2:
        Rgb(92, 13, 239),
        Rgb(132, 1, 177),
        "hi" => b"\x1b[38;2;92;13;239;48;2;132;1;177mhi\x1b[0m"
    );

    // Mixed named color tests
    test_color!(blk_on_cyan: Black, Cyan, "hi" => b"\x1b[30;46mhi\x1b[0m");
    test_color!(grn_on_purp: Green, Magenta, "hi" => b"\x1b[32;45mhi\x1b[0m");
    test_color!(red_on_wht: Red, White, "hi" => b"\x1b[31;47mhi\x1b[0m");
    test_color!(blue_on_ylw: Blue, Yellow, "hi" => b"\x1b[34;43mhi\x1b[0m");
    test_color!(br_blk_on_cyan: BrightBlack, Cyan, "hi" => b"\x1b[90;46mhi\x1b[0m");
    test_color!(br_grn_on_purp: BrightGreen, Magenta, "hi" => b"\x1b[92;45mhi\x1b[0m");
    test_color!(br_red_on_wht: BrightRed, White, "hi" => b"\x1b[91;47mhi\x1b[0m");
    test_color!(br_blue_on_ylw: BrightBlue, Yellow, "hi" => b"\x1b[94;43mhi\x1b[0m");
    test_color!(blk_on_br_cyan: Black, BrightCyan, "hi" => b"\x1b[30;106mhi\x1b[0m");
    test_color!(grn_on_br_purp: Green, BrightMagenta, "hi" => b"\x1b[32;105mhi\x1b[0m");
    test_color!(red_on_br_wht: Red, BrightWhite, "hi" => b"\x1b[31;107mhi\x1b[0m");
    test_color!(blue_on_br_ylw: Blue, BrightYellow, "hi" => b"\x1b[34;103mhi\x1b[0m");
    test_color!(br_blk_on_br_cyan: BrightBlack, BrightCyan, "hi" => b"\x1b[90;106mhi\x1b[0m");
    test_color!(br_grn_on_br_purp: BrightGreen, BrightMagenta, "hi" => b"\x1b[92;105mhi\x1b[0m");
    test_color!(br_red_on_br_wht: BrightRed, BrightWhite, "hi" => b"\x1b[91;107mhi\x1b[0m");
    test_color!(br_blue_on_br_ylw: BrightBlue, BrightYellow, "hi" => b"\x1b[94;103mhi\x1b[0m");
}
