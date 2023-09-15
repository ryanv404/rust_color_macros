# rust_color_macros
[![Rust](https://github.com/ryanv404/rust_color_macros/actions/workflows/rust.yml/badge.svg)](https://github.com/ryanv404/rust_color_macros/actions/workflows/rust.yml)

A Rust library of simple macros for writing colored text to a buffer or
to the terminal.

## Features
Set foreground and background colors using macros that are reminiscent of
the Rust standard library macros.

There are three different color modes: [basic color mode](#Basic-Color-Mode), [256-color mode](#256-Color-Mode), [24-bit RGB color mode](#24-Bit-RGB-Color-Mode).

### Basic Color Mode

Foreground and background colors are set using named options:

`Color::Black`, `Color::Red`, `Color::Green`, `Color::Yellow`, `Color::Blue`,
`Color::Magenta`, `Color::Cyan`, `Color::White`, `Color::BrBlack`, `Color::BrRed`,
`Color::BrGreen`, `Color::BrYellow`, `Color::BrBlue`, `Color::BrMagenta`,
`Color::BrCyan`, `Color::BrWhite`, and `Color::Current`.

Macro signature: `write_color!(buffer, fg_color, bg_color, text)`

```rust
use std::io::Write;
use color_macros::{write_color, Color};

let mut buffer: Vec<u8> = vec![];

// Write red text on a white background to a buffer.
write_color!(&mut buffer, Color::Red, Color::White, "test");

assert_eq!(buffer.as_slice(), b"\x1b[31;47mtest\x1b[0m");

// `writeln_color!()` is the same as `write_color!()` with
// a newline appended to the end.
```

Macro signature: `print_color!(fg_color, bg_color, text)`

```rust
use std::io::Write;
use color_macros::{print_color, Color};

// Prints red text on a white background to stdout.
print_color!(Color::Red, Color::White, "test");

// `println_color!()` is the same as `print_color!()` with
// a newline appended to the end.
```

Macro signature: `eprint_color!(fg_color, bg_color, text)`

```rust
use std::io::Write;
use color_macros::{eprint_color, Color};

// Prints red text on a white background to stderr.
eprint_color!(Color::Red, Color::White, "test");

// `eprintln_color!()` is the same as `eprint_color!()` with
// a newline appended to the end.
```

### 256-Color Mode

Foreground and background colors are set using color numbers ranging
from 0 - 255.

Macro signature: `write_color256!(buffer, fg_color_num, bg_color_num, text)`

```rust
use std::io::Write;
use color_macros::write_color256;

let mut buffer: Vec<u8> = vec![];

// Write red text on a white background to a buffer.
write_color256!(&mut buffer, 196, 255, "test");

assert_eq!(buffer.as_slice(), b"\x1b[38;5;196;48;5;255mtest\x1b[0m");

// `writeln_color256!()` is the same as `write_color256!()` with
// a newline appended to the end.
```

Macro signature: `print_color256!(fg_color_num, bg_color_num, text)`

```rust
use std::io::Write;
use color_macros::print_color256;

// Prints red text on a white background to stdout.
print_color256!(196, 255, "test");

// `println_color256!()` is the same as `print_color256!()` with
// a newline appended to the end.
```

Macro signature: `eprint_color256!(fg_color_num, bg_color_num, text)`

```rust
use std::io::Write;
use color_macros::eprint_color256;

// Prints red text on a white background to stderr.
eprint_color256!(196, 255, "test");

// `eprintln_color256!()` is the same as `eprint_color256!()` with
// a newline appended to the end.
```

### 24-Bit RGB Color Mode

Foreground and background colors are set using tuples containing red, green, and
blue values. The valid range for each value is 0 - 255.

Macro signature: `write_rgb!(buffer, (fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use std::io::Write;
use color_macros::write_rgb;

let mut buffer: Vec<u8> = vec![];

// Write red text on a white background to a buffer.
write_rgb!(&mut buffer, (211, 0, 0), (255, 255, 255), "test");

assert_eq!(buffer.as_slice(), b"\x1b[38;2;211;0;0;48;2;255;255;255mtest\x1b[0m");

// `writeln_rgb!()` is the same as `write_rgb!()` with a
// newline appended to the end.
```

Macro signature: `print_rgb!((fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use std::io::Write;
use color_macros::print_rgb;

// Print red text on a white background to stdout.
print_rgb!((211, 0, 0), (255, 255, 255), "test");

// `println_rgb!()` is the same as `print_rgb!()` with a
// newline appended to the end.
```

Macro signature: `eprint_rgb!((fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use std::io::Write;
use color_macros::eprint_rgb;

// Print red text on a white background to stderr.
eprint_rgb!((211, 0, 0), (255, 255, 255), "test");

// `eprintln_rgb!()` is the same as `eprint_rgb!()` with a
// newline appended to the end.
```
