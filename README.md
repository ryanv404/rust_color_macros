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

* `write_styled!(buffer, style_string, text)`
* `writeln_styled!(buffer, style_string, text)`

```rust
use color_macros::write_styled;

let mut buffer: Vec<u8> = vec![];

// Write bright red text on the currently set background color
// to a buffer.
//
// Note: the style string is not case sensitive.
write_styled!(&mut buffer, "briGHt ReD on cURrenT", "test");

assert_eq!(buffer.as_slice(), b"\x1b[91mtest\x1b[0m");
```

* `print_styled!(style_string, text)`
* `println_styled!(style_string, text)`

```rust
use color_macros::print_styled;

// Prints red text on a white background to stdout with
// a newline appended to the end.
println_styled!("Red on White", "test");
```

* `eprint_styled!(style_string, text)`
* `eprintln_styled!(style_string, text)`

```rust
use color_macros::eprint_styled;

// Prints bright green text on a bright yellow background to stderr.
eprint_styled!("bright green on bright yellow", "omg why?");
```

256-Color Mode
--------------

Foreground and background colors are set using color numbers ranging
from 0 - 255.

256-Color Mode Macros and Examples
----------------------------------

* `write_color256!(buffer, fg_color_num, bg_color_num, text)`
* `writeln_color256!(buffer, fg_color_num, bg_color_num, text)`

```rust
use color_macros::write_color256;

let mut buffer: Vec<u8> = vec![];

// Write red text on a white background to a buffer.
write_color256!(&mut buffer, 196, 255, "test");

assert_eq!(
    buffer.as_slice(),
    b"\x1b[38;5;196;48;5;255mtest\x1b[0m"
);
```

* `print_color256!(fg_color_num, bg_color_num, text)`
* `println_color256!(fg_color_num, bg_color_num, text)`

```rust
use color_macros::print_color256;

// Prints red text on a white background to stdout.
print_color256!(196, 255, "test");
```

* `eprint_color256!(fg_color_num, bg_color_num, text)`
* `eprintln_color256!(fg_color_num, bg_color_num, text)`

```rust
use color_macros::eprint_color256;

// Prints red text on a white background to stderr.
eprint_color256!(196, 255, "test");
```

24-Bit RGB Color Mode
---------------------

Foreground and background colors are set using tuples containing red, green, and
blue color values, each ranging from 0 - 255.

24-Bit RGB Color Mode Macros and Examples
-----------------------------------------

* `write_rgb!(buffer, (fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`
* `writeln_rgb!(buffer, (fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use color_macros::write_rgb;

let mut buffer: Vec<u8> = vec![];

// Write red text on a white background to a buffer.
write_rgb!(&mut buffer, (211, 0, 0), (255, 255, 255), "test");

assert_eq!(
    buffer.as_slice(),
    b"\x1b[38;2;211;0;0;48;2;255;255;255mtest\x1b[0m"
);
```

* `print_rgb!((fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`
* `println_rgb!((fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use color_macros::print_rgb;

// Print red text on a white background to stdout.
print_rgb!((211, 0, 0), (255, 255, 255), "test");
```

* `eprint_rgb!((fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`
* `eprintln_rgb!((fg_r, fg_g, fg_b), (bg_r, bg_g, bg_b), text)`

```rust
use color_macros::eprint_rgb;

// Print red text on a white background to stderr with
// a newline appended to the end.
eprintln_rgb!((211, 0, 0), (255, 255, 255), "test");
```


More Examples
=============

See the `examples/` directory for more examples of the library in action.
