fn print_population(day: u32, population: &Vec<u64>) {
    let count: u64 = population.iter().sum();
    print!("After\t{}\tdays count is {}: ", day, count);

    for (pos, count) in population.iter().enumerate() {
        print!("{}x{},", count, pos);
    }
    println!();
}

fn simulate(days: u32, population: &mut Vec<u64>) {
    for day in 0..=days {
        let mut temp: Vec<u64> = vec![0; 9];

        print_population(day, population);

        for (age, count) in population.iter().enumerate() {
            // we spawn a new fish
            if age == 0 {
                // internal fish timer resets to 6
                temp[6] += count;
                // and a new fish is created
                temp[8] += count;
            } else {
                temp[age - 1] += count;
            }
        }

        population.clear();
        population.append(&mut temp);
    }
}

fn star1(input: &str) {
    let days = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap_or_default())
        .collect::<Vec<usize>>();

    // the structure to keep lanterfish in is
    // map[days left] = count of lanternfish
    //
    // where days left is a number between 0..=8
    let mut population: Vec<u64> = vec![0; 9];

    // populate the initial herd
    for day in days {
        population[day] += 1;
    }

    // start simulation
    simulate(80, &mut population);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
