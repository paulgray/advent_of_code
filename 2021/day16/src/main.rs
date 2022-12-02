fn to_binary(c: char) -> u8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
    }
}

fn star1(input: &str) {
    input.chars().collect::<Vec<char>>().map(|c| to_binary(c));
}

fn main() {
    let contents =
        std::fs::read_to_string("test0").expect("Something went wrong when reading the input file");

    star1(&contents.trim());
}
