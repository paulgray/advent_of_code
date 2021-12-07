fn cost(position: i32, positions: &Vec<i32>) -> i32 {
    let mut acc: i32 = 0;

    for p in positions {
        acc += (position - p).abs();
    }

    return acc;
}

fn star1(input: &str) {
    let mut positions = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap_or_default())
        .collect::<Vec<i32>>();
    positions.sort();

    // let's find the best position
    let mut proposed = *positions.first().unwrap();
    let last = *positions.last().unwrap();
    let mut min_fuel = i32::MAX;

    while proposed < last {
        let fuel = cost(proposed, &positions);
        if fuel < min_fuel {
            println!("Found new min fuel! Position {}, cost {}", proposed, fuel);
            min_fuel = fuel;
        }

        proposed += 1;
    }

    println!("Min fuel: {}", min_fuel);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
