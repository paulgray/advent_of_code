fn star1(input: &str) {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut acc = vec![0; lines[0].len()];
    lines.iter().for_each(|line| {
        for (idx, c) in line.char_indices() {
            if c == '1' {
                acc[idx] += 1;
            }
        }
    });

    let mut gamma = 0;
    let mut epsilon = 0;
    let majority = lines.len() / 2;
    (0..acc.len()).for_each(|pos| {
        gamma *= 2;
        epsilon *= 2;
        if acc[pos] > majority {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    });

    println!(
        "\nmajority: {}\ng: {}\ne: {}\nm: {}",
        majority,
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
