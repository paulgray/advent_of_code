fn star1(input: &str) {
    let coords = input
        .trim()
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|i| (i[0], i[1].parse::<u32>().unwrap_or_default()))
        .fold((0, 0), |(horizontal, depth), (dir, modifier)| match dir {
            "forward" => (horizontal + modifier, depth),
            "down" => (horizontal, depth + modifier),
            "up" => (horizontal, depth - modifier),
            &_ => (horizontal, depth),
        });

    println!("{}", coords.0 * coords.1);
}

fn star2(input: &str) {
    let coords = input
        .trim()
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|i| (i[0], i[1].parse::<u32>().unwrap_or_default()))
        .fold(
            (0, 0, 0),
            |(horizontal, depth, aim), (dir, modifier)| match dir {
                "forward" => (horizontal + modifier, depth + aim * modifier, aim),
                "down" => (horizontal, depth, aim + modifier),
                "up" => (horizontal, depth, aim - modifier),
                &_ => (horizontal, depth, aim),
            },
        );

    println!("{}", coords.0 * coords.1);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
    star2(&contents);
}
