use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn count_orbits(lines : Vec<&str>) -> (i32, i32) {
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

    let mut my_chain = Vec::new();
    let mut santa_chain = Vec::new();
    let mut orbit = planets.get(&"YOU").unwrap();
    loop {
        my_chain.push(orbit);
        if !planets.contains_key(orbit) {
            break;
        }
        orbit = planets.get(orbit).unwrap();
    }
    
    orbit = planets.get(&"SAN").unwrap();
    loop {
        santa_chain.push(orbit);
        if !planets.contains_key(orbit) {
            break;
        }
        orbit = planets.get(orbit).unwrap();
    }

    let mut xfers : i32 = 0;
    for (idx, planet) in my_chain.iter().enumerate() {
        let common = santa_chain.iter().position(|x| x == planet);
        if common.is_some() {
            xfers = idx as i32 + common.unwrap() as i32;
            break;
        }
    }

    return (count, xfers);
}

fn main() {
    let mut file = File::open("/tmp/input6").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let planets = input.split("\n").collect::<Vec<&str>>();

    let (count, xfers) = count_orbits(planets);
    println!("Orbits: {}, {}", count, xfers);
}

#[test]
fn test() {
    let input = vec!["COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU", "I)SAN"];
    assert_eq!((54, 4), count_orbits(input));
}