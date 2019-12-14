use std::fs::File;
use std::io::prelude::*;

fn count_floors(s: &str) -> i32 {
    let mut pos = 1;
    let mut floor = 0;

    for c in s.chars() {
        if c == ')' {
            floor -= 1;
        } else {
            floor += 1;
        }

        if floor == -1 {
            return pos;
        }

        pos += 1;
    }

    return 0;
}

fn main() {
    let mut file = File::open("/tmp/2015.1").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let floor = count_floors(&input.trim());
    println!("Position {}", floor);
}
