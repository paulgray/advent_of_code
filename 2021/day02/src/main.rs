fn star1(input: &str) {
    let coords = input
        .trim()
        .split("\n")
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|i| (i[0], i[1].parse::<u32>().unwrap_or_default()))
        .fold((0, 0), |(horizontal, depth), (dir, modifier)| match dir {
            "forward" => (horizontal + modifier, depth),
            "down" => return (horizontal, depth + modifier),
            "up" => return (horizontal, depth - modifier),
            &_ => return (horizontal, depth),
        });

    println!("{}", coords.0 * coords.1);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
