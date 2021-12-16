use std::collections::HashMap;

fn print_map(map: &Vec<Vec<u32>>) {
    for row in map {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn build_map(max_x: usize, max_y: usize) -> Vec<Vec<u32>> {
    let mut map: Vec<Vec<u32>> = Vec::new();

    // construct the initial map with empty values
    let mut row: Vec<u32> = Vec::new();
    let mut i = 0;
    while i < max_x {
        row.push(0);
        i += 1;
    }

    let mut j = 0;
    while j < max_y {
        map.push(row.clone());
        j += 1;
    }

    return map;
}

fn fill_map(input: &Vec<&str>, max_x: usize, max_y: usize, map: &mut Vec<Vec<u32>>) {
    // now replicate the map 5x to the right and to the bottom
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let val = c.to_digit(10).unwrap();

            for mul_x in 0..5 {
                for mul_y in 0..5 {
                    let mut final_val = val + mul_x + mul_y;
                    if final_val > 9 {
                        // we need to wrap it
                        final_val -= 9;
                    }
                    map.get_mut(max_y * mul_y as usize + y).unwrap()[max_x * mul_x as usize + x] =
                        final_val;
                }
            }
        }
    }
}

fn find_path(
    pos: (usize, usize),
    max_x: usize,
    max_y: usize,
    unvisited: &mut Vec<(usize, usize)>,
    costs: &mut HashMap<(usize, usize), u32>,
    map: &Vec<Vec<u32>>,
) {
    if unvisited.len() % 10 == 0 {
        println!("Unvisited len: {}", unvisited.len());
    }

    // for the current node, consider all unvisited neighbors
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if pos.0 > 0 && unvisited.contains(&(pos.0 - 1, pos.1)) {
        neighbors.push((pos.0 - 1, pos.1));
    }
    if pos.0 < max_x - 1 && unvisited.contains(&(pos.0 + 1, pos.1)) {
        neighbors.push((pos.0 + 1, pos.1));
    }
    if pos.1 > 0 && unvisited.contains(&(pos.0, pos.1 - 1)) {
        neighbors.push((pos.0, pos.1 - 1));
    }
    if pos.1 < max_y - 1 && unvisited.contains(&(pos.0, pos.1 + 1)) {
        neighbors.push((pos.0, pos.1 + 1));
    }
    // calculate costs
    let current_cost = *costs.get(&pos).unwrap();
    for neighbor in neighbors {
        let cost = current_cost + map[neighbor.1][neighbor.0];
        let existing_cost = costs.get(&neighbor).unwrap();

        if cost < *existing_cost {
            costs.insert(neighbor, cost);
        }
    }

    let idx = unvisited.iter().position(|x| *x == pos).unwrap();
    unvisited.remove(idx);

    // check for reaching the end
    if pos != (max_x - 1, max_y) {
        // selected an unvisited node that's marked with the
        // lowest cost and continue
        let mut next = (0, 0);
        let mut lowest_cost = u32::MAX;

        for (node, cost) in costs.iter() {
            if unvisited.contains(&node) && *cost < lowest_cost {
                next = *node;
                lowest_cost = *cost;
            }
        }

        if next != (0, 0) {
            find_path(next, max_x, max_y, unvisited, costs, &map);
        }
    }
}

fn star1(input: &str) {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut max_x = lines[0].len();
    let mut max_y = lines.len();

    let mut map: Vec<Vec<u32>> = build_map(5 * max_x, 5 * max_y);
    fill_map(&lines, max_x, max_y, &mut map);
    max_x *= 5;
    max_y *= 5;

    print_map(&map);

    // we're going from (0, 0) to (max_x-1, max_y-1)
    let mut unvisited: Vec<(usize, usize)> = Vec::new();
    let mut costs: HashMap<(usize, usize), u32> = HashMap::new();

    // mark all nodes as unvisited and set the initial costs
    for x in 0..max_x {
        for y in 0..max_y {
            unvisited.push((x, y));
            costs.insert((x, y), u32::MAX);
        }
    }
    costs.insert((0, 0), 0);

    find_path((0, 0), max_x, max_y, &mut unvisited, &mut costs, &map);
    println!(
        "Cost to {},{}: {}",
        max_x - 1,
        max_y - 1,
        costs.get(&(max_x - 1, max_y - 1)).unwrap()
    );
}

fn main() {
    let contents =
        std::fs::read_to_string("test0").expect("Something went wrong when reading the input file");

    star1(&contents);
}
