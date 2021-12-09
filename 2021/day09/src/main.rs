use std::collections::HashMap;

fn print_map(map: &Vec<Vec<u32>>) {
    for row in map {
        for cell in row {
            if *cell == u32::MAX {
                print!(".");
            } else {
                print!("{}", cell);
            }
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

    // now put the top row border
    i = 0;
    let mut border = map.get_mut(0).unwrap();
    while i < max_x {
        border[i] = u32::MAX;
        i += 1;
    }

    // bottom row border
    border = map.get_mut(max_y - 1).unwrap();
    i = 0;
    while i < max_x {
        border[i] = u32::MAX;
        i += 1;
    }

    // left and right column borders
    i = 0;
    while i < max_y {
        let r = map.get_mut(i).unwrap();
        r[0] = u32::MAX;
        r[max_x - 1] = u32::MAX;
        i += 1;
    }

    return map;
}

fn fill_map(input: &Vec<&str>, map: &mut Vec<Vec<u32>>) {
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.get_mut(y + 1).unwrap()[x + 1] = c.to_digit(10).unwrap();
        }
    }
}

fn star1(input: &str) {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    // this is a map size (x, y)
    //
    // to make sure we have identified local minima
    // let's expand the map with a border on each side
    let max_x = lines[0].len() + 2;
    let max_y = lines.len() + 2;

    // let's fill the borders with u32::MAX
    let mut map: Vec<Vec<u32>> = build_map(max_x, max_y);
    fill_map(&lines, &mut map);

    //print_map(&map);

    // find local minima
    let mut risks = 0;
    let mut basins: HashMap<(usize, usize), u32> = HashMap::new();
    for y in 1..max_y - 1 {
        let prev_row = map.get(y - 1).unwrap();
        let curr_row = map.get(y).unwrap();
        let next_row = map.get(y + 1).unwrap();

        for x in 1..max_x - 1 {
            let val = curr_row.get(x).unwrap();

            if val < prev_row.get(x).unwrap()
                && val < next_row.get(x).unwrap()
                && val < curr_row.get(x - 1).unwrap()
                && val < curr_row.get(x + 1).unwrap()
            {
                let coords = (x, y);
                basins.insert(coords, 0);
                risks += 1 + val;
            }
        }
    }

    println!("Risk levels = {}", risks);

    // also, each point will belong to a basin
    //    point coords -> basin coords
    let mut points: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut walls: Vec<(usize, usize)> = Vec::new();
    for m in basins.keys() {
        points.insert(*m, *m);
    }

    // iterate until we have some unassigned points left
    let counter_desired = (max_x - 2) * (max_y - 2);
    while points.len() < counter_desired {
        for y in 1..max_y - 1 {
            let prev_row = map.get(y - 1).unwrap();
            let curr_row = map.get(y).unwrap();
            let next_row = map.get(y + 1).unwrap();
            for x in 1..max_x - 1 {
                let coords = (x, y);
                if points.contains_key(&coords) {
                    // we already tracked this point
                    continue;
                }

                let val = curr_row.get(x).unwrap();
                if *val == 9 {
                    // this does not belong to any basin
                    points.insert(coords, coords);
                    walls.push(coords);
                }

                // the point is not assigned to anything
                // if any of the neighboring points is in
                // a basin and is not a 9 -> let's move it
                // to that basin
                let top = prev_row.get(x).unwrap();
                if val > top && points.contains_key(&(x, y - 1)) && *top < 9 {
                    points.insert(coords, *points.get(&(x, y - 1)).unwrap());
                }

                let bottom = next_row.get(x).unwrap();
                if val > bottom && points.contains_key(&(x, y + 1)) && *bottom < 9 {
                    points.insert(coords, *points.get(&(x, y + 1)).unwrap());
                }

                let left = curr_row.get(x - 1).unwrap();
                if val > left && points.contains_key(&(x - 1, y)) && *left < 9 {
                    points.insert(coords, *points.get(&(x - 1, y)).unwrap());
                }

                let right = curr_row.get(x + 1).unwrap();
                if val > right && points.contains_key(&(x + 1, y)) && *right < 9 {
                    points.insert(coords, *points.get(&(x + 1, y)).unwrap());
                }
            }
        }
    }

    // count basin sizes
    for (point, basin) in points {
        if walls.contains(&point) {
            continue;
        }

        basins.insert(basin, basins.get(&basin).unwrap() + 1);
    }

    // get basin sizes
    let mut sizes: Vec<&u32> = basins.values().collect();
    sizes.sort();
    sizes.reverse();

    println!("Sizes: {}", sizes[0] * sizes[1] * sizes[2]);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
