use std::io::Write;

use color_macros::{
    print_styled, println_styled, write_styled,
    eprint_styled, eprintln_styled,
    Color::*
};

fn main() {
    let mut output1: Vec<u8> = vec![];
    let mut output2: Vec<u8> = vec![];

    write_styled!(&mut output1, Yellow, BrightBlue, " THIS WAS PRINTED ON ");
    write_styled!(&mut output2, Yellow, BrightBlue, " THIS WAS PRINTED ON ");

    print_styled!(White, BrightMagenta, format!("[ {}", String::from_utf8(output1).unwrap()));
    print_styled!(BrightGreen, BrightBlue, "STDOUT ");
    println_styled!(White, BrightMagenta, " ]");

    eprint_styled!(White, BrightMagenta, format!("[ {}", String::from_utf8(output2).unwrap()));
    eprint_styled!(BrightYellow, BrightBlue, "STDERR ");
    eprintln_styled!(White, BrightMagenta, " ]");
}
