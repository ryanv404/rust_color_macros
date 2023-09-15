rust_color_macros
=================

[![Rust](https://github.com/ryanv404/rust_color_macros/actions/workflows/rust.yml/badge.svg)](https://github.com/ryanv404/rust_color_macros/actions/workflows/rust.yml)

A simple Rust library containing easy-to-use macros for writing colored
text to a buffer or to the terminal.


Contents
========

* [Features](#Features)
* [Usage](#Usage)
    * [Basic Color Mode](#Basic-Color-Mode)
        * [Macros and Examples](#Basic-Color-Mode-Macros-and-Examples)
    * [256-Color Mode](#256-Color-Mode)
        * [Macros and Examples](#256-Color-Mode-Macros-and-Examples)
    * [24-Bit RGB Color Mode](#24-Bit-RGB-Color-Mode).
        * [Macros and Examples](#24-Bit-RGB-Color-Mode-Macros-and-Examples)
* [More Examples](#More-Examples)


Features
========

Set foreground and background colors using simple to use macros that are
reminiscent of the most popular Rust standard library macros.

* `write_styled`/`write_color256`/`write_rgb` write colored text to a buffer.
* `print_styled`/`print_color256`/`print_rgb` print colored text to stdout.
* `eprint_styled`/`eprint_color256`/`eprint_rgb` print colored text to stderr.

Note that each macro has a newline version (e.g. `println_styled`).


Usage
=====

Basic Color Mode
----------------

Foreground and background colors are set using an "X on Y" style string,
where X and Y are each one of the following color options:

black, red, green, yellow, blue, magenta, cyan, white, bright black,
bright red, bright green, bright yellow, bright blue, bright magenta,
bright cyan, bright white, or current.

Entering "X" alone (e.g. "red") for the style string sets the foreground color
without altering the background, while entering "on Y" (e.g. "on red") for the
style string sets the background color without altering the foreground.

Basic Color Mode Macros and Examples
------------------------------------

`write_styled!(buffer, style_string, text)`

```rust
use color_macros::write_styled;

let mut buffer: Vec<u8> = vec![];

// Write bright red text on the currently set background color to a buffer.
write_styled!(&mut buffer, "bright red on current", "test");

assert_eq!(buffer.as_slice(), b"\x1b[91mtest\x1b[0m");

// `writeln_styled!()` is the same as `write_styled!()` but with
// a newline appended to the end.
```

`print_styled!(style_string, text)`

```rust
use color_macros::print_styled;

// Prints red text on a white background to stdout.
// Note that the style string is not case sensitive.
print_styled!("ReD on WhITe", "test");

// `println_styled!()` is the same as `print_styled!()` but with
// a newline appended to the end.
```

`eprint_styled!(style_string, text)`

```rust
use color_macros::eprint_styled;

// Prints bright green text on a bright yellow background to stderr.
eprint_styled!("bright green on bright yellow", "omg why?");

// `eprintln_styled!()` is the same as `eprint_styled!()` but with
// a newline appended to the end.
```

256-Color Mode
--------------

Foreground and background colors are set using color numbers ranging
from 0 - 255.

256-Color Mode Macros and Examples
----------------------------------

`write_color256!(buffer, fg_color_num, bg_color_num, text)`

```rust
use color_macros::write_color256;

let mut buffer: Vec<u8> = vec![];

// Write red text on a white background to a buffer.
write_color256!(&mut buffer, 196, 255, "test");

assert_eq!(buffer.as_slice(), b"\x1b[38;5;196;48;5;255mtest\x1b[0m");

// `writeln_color256!()` is the same as `write_color256!()` but with
// a newline appended to the end.
```

`print_color256!(fg_color_num, bg_color_num, text)`

```rust
use color_macros::print_color256;

// Prints red text on a white background to stdout.
print_color256!(196, 255, "test");

// `println_color256!()` is the same as `print_color256!()` but with
// a newline appended to the end.
```

`eprint_color256!(fg_color_num, bg_color_num, text)`

```rust
use color_macros::eprint_color256;

// Prints red text on a white background to stderr.
eprint_color256!(196, 255, "test");

// `eprintln_color256!()` is the same as `eprint_color256!()` but with
// a newline appended to the end.
```

24-Bit RGB Color Mode
---------------------

Foreground and background colors are set using tuples containing red, green, and
blue color values, each ranging from 0 - 255.

24-Bit RGB Color Mode Macros and Examples
-----------------------------------------

`write_rgb!(buffer, (fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use color_macros::write_rgb;

let mut buffer: Vec<u8> = vec![];

// Write red text on a white background to a buffer.
write_rgb!(&mut buffer, (211, 0, 0), (255, 255, 255), "test");

assert_eq!(buffer.as_slice(), b"\x1b[38;2;211;0;0;48;2;255;255;255mtest\x1b[0m");

// `writeln_rgb!()` is the same as `write_rgb!()` but with a
// newline appended to the end.
```

`print_rgb!((fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use color_macros::print_rgb;

// Print red text on a white background to stdout.
print_rgb!((211, 0, 0), (255, 255, 255), "test");

// `println_rgb!()` is the same as `print_rgb!()` but with a
// newline appended to the end.
```

`eprint_rgb!((fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use color_macros::eprint_rgb;

// Print red text on a white background to stderr.
eprint_rgb!((211, 0, 0), (255, 255, 255), "test");

// `eprintln_rgb!()` is the same as `eprint_rgb!()` but with a
// newline appended to the end.
```


More Examples
=============

See the `examples/` directory for more examples of the library in action.
