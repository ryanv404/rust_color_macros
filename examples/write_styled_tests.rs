use std::fmt::Write;

use color_macros::write_styled;

fn main() {
    // Tests every combination of basic foreground and background colors
    // against the expected output.

    const FG_COLORS: [(&str, &str); 17] = [
        ("black", "30"),
        ("red", "31"),
        ("green", "32"),
        ("yellow", "33"),
        ("blue", "34"),
        ("magenta", "35"),
        ("cyan", "36"),
        ("white", "37"),
        ("bright black", "90"),
        ("bright red", "91"),
        ("bright green", "92"),
        ("bright yellow", "93"),
        ("bright blue", "94"),
        ("bright magenta", "95"),
        ("bright cyan", "96"),
        ("bright white", "97"),
        ("current", ""),
    ];

    const BG_COLORS: [(&str, &str); 17] = [
        ("black", "40"),
        ("red", "41"),
        ("green", "42"),
        ("yellow", "43"),
        ("blue", "44"),
        ("magenta", "45"),
        ("cyan", "46"),
        ("white", "47"),
        ("bright black", "100"),
        ("bright red", "101"),
        ("bright green", "102"),
        ("bright yellow", "103"),
        ("bright blue", "104"),
        ("bright magenta", "105"),
        ("bright cyan", "106"),
        ("bright white", "107"),
        ("current", ""),
    ];

    let mut fail = String::new();
    let mut pass = String::new();
    let mut output = Vec::new();
    let mut expected = Vec::new();

    write_styled!(&mut fail, "bright red", "\u{2718}");
    write_styled!(&mut pass, "bright green", "\u{2713}");

    println!("All Combinations of Basic Foreground and Background Colors");
    println!("       (write_styled!() Output vs Expected Output)\n");

    for fg_idx in 0..FG_COLORS.len() {
        for bg_idx in (0..BG_COLORS.len()).rev() {
            let fg_color = FG_COLORS[fg_idx].0;
            let bg_color = BG_COLORS[bg_idx].0;
            let fg_code = FG_COLORS[fg_idx].1;
            let bg_code = BG_COLORS[bg_idx].1;

            output.clear();
            expected.clear();

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

            println!(
                "[ {} ] {} vs {}",
                if &output[..] == &expected[..] {
                    &pass
                } else {
                    &fail
                },
                String::from_utf8(output.clone()).unwrap(),
                String::from_utf8(expected.clone()).unwrap()
            );
        }
    }
}
