fn cost(position: i32, positions: &Vec<i32>, costs: &Vec<u128>) -> u128 {
    let mut acc: u128 = 0;

    for p in positions {
        let distance = (position - p).abs();

        if distance > 10000 {
            // cost is way too high
            return u128::MAX;
        }
        acc += costs[distance as usize];
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
    let mut min_fuel = u128::MAX;

    // compute costs for the first 10000 positions or so
    let mut costs: Vec<u128> = Vec::new();
    costs.push(0);
    for n in 1..=10000 {
        costs.push(n + costs.last().unwrap());
    }

    while proposed < last {
        let fuel = cost(proposed, &positions, &costs);
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
