use std::io::Write;

use color_macros::{
    eprint_color, eprintln_color, print_color, println_color, write_color, Color::*,
};

fn main() {
    let mut output1: Vec<u8> = vec![];
    let mut output2: Vec<u8> = vec![];

    write_color!(&mut output1, Red, White, " THIS WAS PRINTED ON ");
    write_color!(&mut output2, White, Red, " THIS WAS PRINTED ON ");

    print_color!(
        Red,
        White,
        format!("[ {}", String::from_utf8(output1).unwrap())
    );
    print_color!(Cyan, White, "STDOUT ");
    println_color!(Red, White, " ]");

    eprint_color!(
        White,
        Red,
        format!("[ {}", String::from_utf8(output2).unwrap())
    );
    eprint_color!(Blue, Red, "STDERR ");
    eprintln_color!(White, Red, " ]");
}
