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
    let file = File::open("/tmp/input").unwrap();
    let reader = BufReader::new(file);
    let sum: i32 = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap())
                            .map(get_fuel)
                            .sum();

    println!("Fuel: {}", sum);
}

#[test]
fn test() {
    assert_eq!(2, get_fuel(14));
    assert_eq!(966, get_fuel(1969));
    assert_eq!(50346, get_fuel(100756));
}