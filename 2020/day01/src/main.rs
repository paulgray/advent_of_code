use std::fs::File;
use std::io::{BufReader, BufRead};

fn get_fuel(mass: i32) -> i32 {
    let mut sum = 0;
    let mut fuel = (mass / 3) - 2;

    while fuel > 0 {
        sum += fuel;
        fuel = (fuel / 3) - 2;
    }

    return sum;
}

fn main() {
    let filename = "/tmp/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let sum = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap())
                            .map(get_fuel)
                            .fold(0, |acc, fuel| acc + fuel);

    println!("Fuel: {}", sum);
}
