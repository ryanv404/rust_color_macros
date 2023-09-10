#[macro_export]
macro_rules! write_styled {
    ($dst:expr, $fg:ident, $bg:ident, $msg:literal) => {
        $dst.write_all(
            format!(
                "\u{001b}[{}{}{}m{}\u{001b}[0m",
                $crate::Color::get_fg_code($crate::Color::$fg),
                if $crate::Color::$fg != $crate::Color::Current &&
                   $crate::Color::$bg != $crate::Color::Current {
                    ";"
                } else { 
                    ""
                },
                $crate::Color::get_bg_code($crate::Color::$bg),
                $msg
            )
            .as_bytes()
        ).unwrap();
        $dst.flush().unwrap();
    };
}

#[macro_export]
macro_rules! writeln_styled {
    ($dst:expr, $fg:ident, $bg:ident, $msg:literal) => {
        $dst.write_all(
            format!(
                "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
                $crate::Color::get_fg_code($crate::Color::$fg),
                if $crate::Color::$fg != $crate::Color::Current &&
                   $crate::Color::$bg != $crate::Color::Current {
                    ";"
                } else { 
                    ""
                },
                $crate::Color::get_bg_code($crate::Color::$bg),
                $msg
            )
            .as_bytes()
        ).unwrap();
        $dst.flush().unwrap();
    };
}

#[macro_export]
macro_rules! print_styled {
    ($fg:ident, $bg:ident, $msg:literal) => {
        let mut stdout = std::io::stdout().lock();
        stdout.write_all(
            format!(
                "\u{001b}[{}{}{}m{}\u{001b}[0m",
                $crate::Color::get_fg_code($crate::Color::$fg),
                if $crate::Color::$fg != $crate::Color::Current &&
                   $crate::Color::$bg != $crate::Color::Current {
                    ";"
                } else { 
                    ""
                },
                $crate::Color::get_bg_code($crate::Color::$bg),
                $msg
            )
            .as_bytes()
        ).unwrap();
        stdout.flush().unwrap();
    };
}

#[macro_export]
macro_rules! println_styled {
    ($fg:ident, $bg:ident, $msg:literal) => {
        let mut stdout = std::io::stdout().lock();
        stdout.write_all(
            format!(
                "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
                $crate::Color::get_fg_code($crate::Color::$fg),
                if $crate::Color::$fg != $crate::Color::Current &&
                   $crate::Color::$bg != $crate::Color::Current {
                    ";"
                } else { 
                    ""
                },
                $crate::Color::get_bg_code($crate::Color::$bg),
                $msg
            )
            .as_bytes()
        ).unwrap();
        stdout.flush().unwrap();
    };
}

#[macro_export]
macro_rules! eprint_styled {
    ($fg:ident, $bg:ident, $msg:literal) => {
        let mut stderr = std::io::stderr().lock();
        stderr.write_all(
            format!(
                "\u{001b}[{}{}{}m{}\u{001b}[0m",
                $crate::Color::get_fg_code($crate::Color::$fg),
                if $crate::Color::$fg != $crate::Color::Current &&
                   $crate::Color::$bg != $crate::Color::Current {
                    ";"
                } else { 
                    ""
                },
                $crate::Color::get_bg_code($crate::Color::$bg),
                $msg
            )
            .as_bytes()
        ).unwrap();
        stderr.flush().unwrap();
    };
}

#[macro_export]
macro_rules! eprintln_styled {
    ($fg:ident, $bg:ident, $msg:literal) => {
        let mut stderr = std::io::stderr().lock();
        stderr.write_all(
            format!(
                "\u{001b}[{}{}{}m{}\u{001b}[0m\n",
                $crate::Color::get_fg_code($crate::Color::$fg),
                if $crate::Color::$fg != $crate::Color::Current &&
                   $crate::Color::$bg != $crate::Color::Current {
                    ";"
                } else { 
                    ""
                },
                $crate::Color::get_bg_code($crate::Color::$bg),
                $msg
            )
            .as_bytes()
        ).unwrap();
        stderr.flush().unwrap();
    };
}

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
}

impl Color {
    pub fn get_fg_code(color: Color) -> String {
        match color {
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
            Self::Color256(num) => {
                format!("38;5;{}",
                    if num < 10 {
                        (num + b'0') as char
                    } else {
                        char::from_digit(num.into(), 10).unwrap()
                    }
                )
            },
            Self::Rgb(r, g, b) => {
                format!("38;2;{};{};{}",
                    if r < 10 { (r + b'0') as char } else { char::from_digit(r.into(), 10).unwrap() },
                    if g < 10 { (g + b'0') as char } else { char::from_digit(g.into(), 10).unwrap() },
                    if b < 10 { (b + b'0') as char } else { char::from_digit(b.into(), 10).unwrap() }
                )
            },
            Self::Current => "".to_string()
        }
    }

    pub fn get_bg_code(color: Color) -> String {
        match color {
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
            Self::Color256(num) => {
                format!("48;5;{}",
                    if num < 10 {
                        (num + b'0') as char
                    } else {
                        char::from_digit(num.into(), 10).unwrap()
                    }
                )
            },
            Self::Rgb(r, g, b) => {
                format!("48;2;{};{};{}",
                    if r < 10 { (r + b'0') as char } else { char::from_digit(r.into(), 10).unwrap() },
                    if g < 10 { (g + b'0') as char } else { char::from_digit(g.into(), 10).unwrap() },
                    if b < 10 { (b + b'0') as char } else { char::from_digit(b.into(), 10).unwrap() }
                )
            },
            Self::Current => "".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::write_styled;

    macro_rules! test_color {
        ($name:ident: $fg:ident, $bg:ident, $msg:literal => $expected:literal) => {
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
//    test_color!(color256: Color256(123), Current, "hi" => b"\x1b[38;5;123mhi\x1b[0m");
//    test_color!(bg_color256: Current, Color256(52), "hi" => b"\x1b[48;5;52mhi\x1b[0m");

    // RGB tests
//    test_color!(rgb: Rgb(192, 123, 23), Current, "hi" => b"\x1b[38;2;192;123;23mhi\x1b[0m");
//    test_color!(bg_rgb: Current, Rgb(192, 123, 23), "hi" => b"\x1b[48;2;192;123;23mhi\x1b[0m");

    // Mixed tests
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
