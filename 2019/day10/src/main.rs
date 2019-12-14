extern crate num;

use num::integer::gcd;
use std::collections::HashSet;
use std::f64;
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

fn asteroids_in_line(ay: usize, ax: usize, by: usize, bx: usize) -> Vec<(f64, f64)> {
    let dx = bx as i32 - ax as i32;
    let dy = by as i32 - ay as i32;

    let delta = std::cmp::max(dx.abs(), dy.abs()) as usize;

    let delta_x = dx as f64 / delta as f64;
    let delta_y = dy as f64 / delta as f64;

    return (1..=delta)
        .map(move |d| {
            (
                ax as f64 + delta_x * delta as f64,
                ay as f64 + delta_y * delta as f64,
            )
        })
        .collect();
}

fn find_asteroids(m: &Vec<Vec<char>>, y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut asteroids = Vec::new();
    let width = m[0].len();
    let height = m.len();

    for j in 0..height {
        for i in 0..width {
            if m[j][i] == '.' {
                continue;
            }

            let line = asteroids_in_line(y, x, j, i)
                .iter()
                .filter(|(a, b)| a.fract() == 0.0 && b.fract() == 0.0)
                .map(|(a, b)| (*a as usize, *b as usize));
        }
    }

    return asteroids;
}

fn annihiliate_asteroids(m: &Vec<Vec<char>>, y: usize, x: usize) -> Vec<&(usize, usize)> {
    let width = m[0].len();
    let height = m.len();
    let mut to_destroy: Vec<&(usize, usize)> = Vec::new();

    while to_destroy.len() < 200 {
        // at first find all asteroids visible from our current location
        let mut asteroids = find_asteroids(&m, y, x);
        asteroids.sort_by(|(ax, ay), (bx, by)| {
            let da = (ax - x, ay - y);
            let db = (bx - x, by - y);
            return angle((x, y), da).partial_cmp(&angle((x, y), db)).unwrap();
        });

        // remove all those asteroids one by one from the map
        for (ax, ay) in asteroids {
            m[ay][ax] = '.';
        }

        // add all asteroids to the list of destroyed items
        to_destroy.append(&asteroids);
    }

    return to_destroy;
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
