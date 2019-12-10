extern crate num;

use num::integer::gcd;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn count_detected(m: &Vec<Vec<char>>, y: usize, x: usize) -> usize {
    let width = m[0].len();
    let height = m.len();
    let mut targets = HashSet::new();

    for j in 0..height {
        for i in 0..width {
            if m[j][i] == '.' || (x == i && y == j) {
                continue;
            }

            let mut dx = x as i32 - i as i32;
            let mut dy = y as i32 - j as i32;
            let distance = gcd(dx.abs(), dy.abs());
            dx /= distance;
            dy /= distance;
            targets.insert((dx, dy));
        }
    }

    return targets.len();
}

fn find_best_location(m: &Vec<Vec<char>>) -> usize {
    let mut max_count = 0;
    let width = m[0].len();
    let height = m.len();

    for y in 0..height {
        for x in 0..width {
            if m[y][x] == '.' {
                continue;
            }

            let detected = count_detected(&m, y, x);
            if detected > max_count {
                max_count = detected;
            }
        }
    }

    return max_count;
}

fn prepare_input(s: &str) -> Vec<Vec<char>> {
    return s
        .split("\n")
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .filter(|line| line.len() > 0)
        .collect::<Vec<Vec<char>>>();
}

fn main() {
    let mut file = File::open("/tmp/input10").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let m = prepare_input(&input);

    let max = find_best_location(&m);
    println!("Max {}", max);
}

#[test]
fn test() {
    let mut input = prepare_input(
        &".#..#
    .....
    #####
    ....#
    ...##",
    );
    assert_eq!(8, find_best_location(&input));

    input = prepare_input(
        &"......#.#.
    #..#.#....
    ..#######.
    .#.#.###..
    .#..#.....
    ..#....#.#
    #..#....#.
    .##.#..###
    ##...#..#.
    .#....####",
    );
    assert_eq!(33, find_best_location(&input));

    input = prepare_input(
        &"#.#...#.#.
    .###....#.
    .#....#...
    ##.#.#.#.#
    ....#.#.#.
    .##..###.#
    ..#...##..
    ..##....##
    ......#...
    .####.###.",
    );
    assert_eq!(35, find_best_location(&input));

    input = prepare_input(
        &".#..#..###
    ####.###.#
    ....###.#.
    ..###.##.#
    ##.##.#.#.
    ....###..#
    ..#.#..#.#
    #..#.#.###
    .##...##.#
    .....#.#..",
    );
    assert_eq!(41, find_best_location(&input));

    input = prepare_input(
        &".#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##",
    );
    assert_eq!(210, find_best_location(&input));
}
