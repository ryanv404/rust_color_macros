use color_macros::{print_color256, println_color256};

fn main() {
    for num in 0..=15 {
        match num {
            7 | 15 => println_color256!(232, num, format!("[{:03}]", num)),
            0 | 8 => print_color256!(7, num, format!("[{:03}]", num)),
            _ => print_color256!(232, num, format!("[{:03}]", num)),
        }
    }

    for num in (16..=27)
        .chain(52..=63)
        .chain(88..=99)
        .chain(124..=135)
        .chain(160..=171)
        .chain(196..=207)
    {
        match num {
            27 | 63 | 99 | 135 | 171 | 207 => {
                println_color256!(7, num, format!("[{:03}]", num));
            }
            _ => print_color256!(7, num, format!("[{:03}]", num)),
        }
    }

    for num in (28..=39)
        .chain(64..=75)
        .chain(100..=111)
        .chain(136..=147)
        .chain(172..=183)
        .chain(208..=219)
    {
        match num {
            39 | 75 | 111 | 147 | 183 | 219 => {
                println_color256!(232, num, format!("[{:03}]", num));
            }
            _ => print_color256!(232, num, format!("[{:03}]", num)),
        }
    }

    for num in (40..=51)
        .chain(76..=87)
        .chain(112..=123)
        .chain(148..=159)
        .chain(184..=195)
        .chain(220..=231)
    {
        match num {
            51 | 87 | 123 | 159 | 195 | 231 => {
                println_color256!(232, num, format!("[{:03}]", num));
            }
            _ => print_color256!(232, num, format!("[{:03}]", num)),
        }
    }

    for num in 232..=255 {
        match num {
            232..=242 => print_color256!(7, num, format!("[{:03}]", num)),
            243 => println_color256!(7, num, format!("[{:03}]", num)),
            255 => println_color256!(232, num, format!("[{:03}]", num)),
            _ => print_color256!(232, num, format!("[{:03}]", num)),
        }
    }
}
