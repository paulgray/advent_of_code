use std::collections::HashMap;
use std::f64;
use std::fs::File;
use std::io::prelude::*;

fn parse(s: &str) -> (HashMap<&str, Vec<&str>>, HashMap<&str, i32>) {
    let mut reactions = HashMap::new();
    let mut quantities = HashMap::new();

    s.split("\n").for_each(|line| {
        let parts = line.split(" => ").collect::<Vec<&str>>();
        let result = parts[1].split(" ").collect::<Vec<&str>>();
        let requirements = parts[0].split(", ").collect::<Vec<&str>>();

        println!(
            "Inserted {} -> {} elements; and {} -> {}",
            result[1],
            requirements.iter().count(),
            result[1],
            result[0].parse::<i32>().unwrap()
        );
        reactions.insert(result[1], requirements);
        quantities.insert(result[1], result[0].parse::<i32>().unwrap());
    });

    return (reactions, quantities);
}

fn resolve(
    needed: i32,
    chemical: &str,
    reactions: &HashMap<&str, Vec<&str>>,
    quantities: &HashMap<&str, i32>,
    mut surplus: &HashMap<&str, i32>,
) -> i32 {
    println!("Resolving for {} {}", needed, chemical);
    let ingredients = reactions.get(chemical).unwrap();
    let result_quantity = quantities.get(chemical).unwrap();

    let mut accumulated;
    if surplus.contains_key(chemical) {
        accumulated = *surplus.get(chemical).unwrap();
    } else {
        accumulated = 0;
    }
    if accumulated > needed {
        accumulated -= needed;
        surplus.insert(chemical, accumulated);

        return 0;
    }

    let needed_reactions = (needed as f64 / *result_quantity as f64).ceil() as i32;
    let leftovers = *result_quantity - needed_reactions;
    let ore: i32 = ingredients
        .iter()
        .map(|ingredient| {
            println!("Required ingredient {}", ingredient.trim());
            let result = ingredient.trim().split(" ").collect::<Vec<&str>>();
            let quantity = result[0].parse::<i32>().unwrap();
            let mut chem = result[1].trim();

            if chem == "ORE" {
                return quantity;
            } else {
                return resolve(quantity, &chem, &reactions, &quantities, &mut surplus);
            }
        })
        .sum();

    return needed_reactions * ore;
}

fn find_ore(s: &str) -> i32 {
    let (reactions, quantities) = parse(&s);
    let mut surplus = HashMap::new();
    let ore_needed = resolve(1, &"FUEL", &reactions, &quantities, &mut surplus);

    return ore_needed;
}

fn main() {
    let mut file = File::open("/tmp/input14").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let ore = find_ore(&input);
    println!("Ore needed {}", ore);
}

#[test]
fn test() {
    let input = "10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL";
    assert_eq!(31, find_ore(input));
}
