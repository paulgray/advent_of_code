use std::collections::HashMap;

fn expand(polymer: &mut HashMap<(char, char), u64>, rules: &HashMap<(char, char), char>) {
    let mut expanded: HashMap<(char, char), u64> = HashMap::new();

    for ((a, b), counter) in polymer.clone() {
        let pair = (a, b);

        if rules.contains_key(&pair) {
            let output = rules.get(&pair).unwrap();

            // as a result of the reaction we're going to get
            // (a, output) and (output, b) pairs
            let left_pair = (a, *output);
            let right_pair = (*output, b);

            let left_count = expanded.entry(left_pair).or_insert(0);
            *left_count += counter;

            let right_count = expanded.entry(right_pair).or_insert(0);
            *right_count += counter;
        } else {
            // the pair remains the same
            let new_counter = expanded.entry(pair).or_insert(0);
            *new_counter += counter;
        }
    }

    polymer.clone_from(&expanded);
}

fn parse_polymer(input: &Vec<char>) -> HashMap<(char, char), u64> {
    let mut polymer = HashMap::new();

    input.windows(2).for_each(|pair| {
        let counter = polymer.entry((pair[0], pair[1])).or_insert(0);
        *counter += 1;
    });

    return polymer;
}

fn get_quantities(initial: &str, polymer: &HashMap<(char, char), u64>) -> u64 {
    let mut quantities: HashMap<char, u64> = HashMap::new();

    // all entries - except for the very first and very last element
    // are double counted
    for ((a, b), count) in polymer.iter() {
        let total_a = quantities.entry(*a).or_insert(0);
        *total_a += count;

        let total_b = quantities.entry(*b).or_insert(0);
        *total_b += count;
    }

    // we need to identify the first and last element
    let chars = initial.chars().collect::<Vec<char>>();
    let first = chars.first().unwrap();
    let last = chars.last().unwrap();
    let first_count = quantities.entry(*first).or_default();
    *first_count += 1;
    let last_count = quantities.entry(*last).or_default();
    *last_count += 1;

    let mut counts: Vec<u64> = Vec::new();
    quantities.values().for_each(|v| counts.push(*v));
    counts.sort();

    return counts.last().unwrap() / 2 - counts.first().unwrap() / 2;
}

fn star1(input: &str) {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    // positions of the elements in the polymer don't really matter
    // what matters is the pairs of adjacent elements
    //
    // let's model the entire thing as a hash map of
    // (element a, element b) -> count of pairs
    let mut polymer: HashMap<(char, char), u64> =
        parse_polymer(&lines[0].chars().collect::<Vec<char>>());
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    let mut i = 2;
    while i < lines.len() {
        let chunks = lines[i].split(" -> ").collect::<Vec<&str>>();
        let mut left = chunks[0].chars();
        rules.insert(
            (left.next().unwrap(), left.next().unwrap()),
            chunks[1].chars().next().unwrap(),
        );

        i += 1;
    }

    // start expanding the polymer
    for step in 0..10 {
        expand(&mut polymer, &rules);

        let l: u64 = polymer.values().sum();
        println!("Polymer length after step {}: {}", step + 1, l);
    }

    // get quantities of elements in the polymer
    let res = get_quantities(&lines[0], &polymer);

    println!("Result: {}", res);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
