fn star1(input: &str) {
    println!(
        "{}",
        input
            .split("\n")
            .fold((0, u32::MAX), |(count, prev), line| {
                let n: u32 = line.parse().unwrap_or_default();
                if n > prev {
                    return (count + 1, n);
                } else {
                    return (count, n);
                }
            })
            .0
    );
}

fn star2(input: &str) {
    let max = u32::MAX;
    println!(
        "{}",
        input
            .split("\n")
            .map(|line| line.parse().unwrap_or_default())
            .collect::<Vec<u32>>()
            .windows(3)
            .fold((0, max), |(count, prev), curr| {
                let sum = curr[0] + curr[1] + curr[2];
                if sum > prev {
                    return (count + 1, sum);
                }
                return (count, sum);
            })
            .0
    );
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");
    star1(&contents);
    star2(&contents);
}
