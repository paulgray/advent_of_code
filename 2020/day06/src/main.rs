use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn count_orbits(lines : Vec<&str>) -> i32 {
    let mut planets : HashMap<&str, &str> = HashMap::new();

    lines.iter().for_each(|s| { 
                                    let parts = s.trim().split(")").collect::<Vec<&str>>();
                                    if parts.len() > 1 {
                                        planets.insert(parts[1], parts[0]);
                                    }
                              });

    let mut count = 0;
    for planet in planets.keys() {
        let mut orbit = planets.get(planet).unwrap();
        count += 1;

        loop {
            if !planets.contains_key(orbit) {
                break;
            }
            orbit = planets.get(orbit).unwrap();

            count += 1;
        }
    }

    return count;
}

fn main() {
    let mut file = File::open("/tmp/input6").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let planets = input.split("\n").collect::<Vec<&str>>();

    let count = count_orbits(planets);
    println!("Orbits: {}", count);
}

#[test]
fn test() {
    let input = vec!["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"];
    assert_eq!(42, count_orbits(input));
}