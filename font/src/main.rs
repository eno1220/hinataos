use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("font.txt").unwrap();
    let mut fonts :[[u8; 16]; 256] = [[0; 16]; 256];
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut row_count = 0;
    let mut font_count = 0;

    for row in contents.split("\n") {
        if row.starts_with("0x"){
            row_count = 0;
            let row_trimmed = row.trim_start_matches("0x");
            let row_int = u8::from_str_radix(row_trimmed, 16).unwrap();
            font_count = row_int as usize;
        }
        if !row.starts_with(".") && !row.starts_with("*") {
            continue;
        }
        let mut row_bits = 0;
        for i in 0..8 {
            if row.chars().nth(i) == Some('*') {
                row_bits |= 1 << (7 - i);
            }
        }
        fonts[font_count][row_count] = row_bits;
        row_count += 1;
    }

    println!("pub const FONT: [[u8; 16]; 256] = {:?};", fonts);
}
