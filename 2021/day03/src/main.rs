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

fn convert(s: &str) -> i32 {
    println!("{}", s);
    let mut val = 0;
    (0..s.len()).for_each(|pos| {
        val *= 2;
        if s.chars().nth(pos).unwrap() == '1' {
            val += 1;
        }
    });

    return val;
}

fn find_oxygen(lines: &mut Vec<&str>) -> i32 {
    let mut res = 0;

    (0..lines[0].len()).for_each(|pos| {
        let mut ones = 0;
        lines.iter().for_each(|line| {
            if line.chars().nth(pos).unwrap() == '1' {
                ones += 1
            }
        });

        let c = if 2 * ones >= lines.len() { '1' } else { '0' };
        println!("Number of 1s in position {}: {}/{}", pos, ones, lines.len());
        println!("Pos {} must be equal to {}", pos, c);

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            if line.chars().nth(pos).unwrap() != c {
                lines.remove(i);
            } else {
                i += 1;
            }
        }

        println!("There is {} positions left\n", lines.len());

        if lines.len() == 1 {
            res = convert(lines.first().unwrap());
            return;
        }
    });

    return res;
}

fn find_co2(lines: &mut Vec<&str>) -> i32 {
    let mut res = 0;

    (0..lines[0].len()).for_each(|pos| {
        let mut ones = 0;
        lines.iter().for_each(|line| {
            if line.chars().nth(pos).unwrap() == '1' {
                ones += 1
            }
        });

        let c = if 2 * ones >= lines.len() { '0' } else { '1' };
        println!("Number of 1s in position {}: {}/{}", pos, ones, lines.len());
        println!("Pos {} must be equal to {}", pos, c);

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i];
            if line.chars().nth(pos).unwrap() != c {
                lines.remove(i);
            } else {
                i += 1;
            }
        }

        println!("There is {} positions left\n", lines.len());

        if lines.len() == 1 {
            res = convert(lines.first().unwrap());
            return;
        }
    });

    return res;
}

fn star2(input: &str) {
    let mut lines = input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut oxygen_lines = lines.to_vec();
    let oxygen = find_oxygen(&mut oxygen_lines);
    let co2 = find_co2(&mut lines);

    println!(
        "Oxygen: {}, CO2: {}, life support rating {}",
        oxygen,
        co2,
        oxygen * co2
    );
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    //star1(&contents);
    star2(&contents);
}
