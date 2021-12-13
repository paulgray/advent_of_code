fn print_map(map: &Vec<Vec<i32>>) {
    for row in map {
        for cell in row {
            if *cell < 0 {
                print!(".");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
    println!();
}

fn build_map(max_x: usize, max_y: usize) -> Vec<Vec<i32>> {
    let mut map: Vec<Vec<i32>> = Vec::new();

    // construct the initial map with empty values
    let mut row: Vec<i32> = Vec::new();
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
        border[i] = i32::MIN;
        i += 1;
    }

    // bottom row border
    border = map.get_mut(max_y - 1).unwrap();
    i = 0;
    while i < max_x {
        border[i] = i32::MIN;
        i += 1;
    }

    // left and right column borders
    i = 0;
    while i < max_y {
        let r = map.get_mut(i).unwrap();
        r[0] = i32::MIN;
        r[max_x - 1] = i32::MIN;
        i += 1;
    }

    return map;
}

fn fill_map(input: &Vec<&str>, map: &mut Vec<Vec<i32>>) {
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.get_mut(y + 1).unwrap()[x + 1] = c.to_digit(10).unwrap() as i32;
        }
    }
}

fn increase_energy(map: &mut Vec<Vec<i32>>) {
    for row in map {
        for cell in row {
            *cell += 1;
        }
    }
}

fn increase_surroundings(x: usize, y: usize, map: &mut Vec<Vec<i32>>) {
    {
        let prev_row = map.get_mut(y - 1).unwrap();

        *prev_row.get_mut(x - 1).unwrap() += 1;
        *prev_row.get_mut(x).unwrap() += 1;
        *prev_row.get_mut(x + 1).unwrap() += 1;
    }

    {
        let curr_row = map.get_mut(y).unwrap();

        *curr_row.get_mut(x - 1).unwrap() += 1;
        *curr_row.get_mut(x + 1).unwrap() += 1;
    }

    {
        let next_row = map.get_mut(y + 1).unwrap();

        *next_row.get_mut(x - 1).unwrap() += 1;
        *next_row.get_mut(x).unwrap() += 1;
        *next_row.get_mut(x + 1).unwrap() += 1;
    }
}

fn reset_cells(map: &mut Vec<Vec<i32>>) {
    for row in map {
        for cell in row {
            if *cell > 9 {
                *cell = 0;
            }
        }
    }
}

fn flash(map: &mut Vec<Vec<i32>>) -> u32 {
    let max_x = map.get_mut(0).unwrap().len();
    let max_y = map.len();
    let mut flashed: Vec<(usize, usize)> = Vec::new();

    loop {
        let mut once = false;

        // any cell with energy level > 9 will flash
        for y in 1..max_y - 1 {
            for x in 1..max_x - 1 {
                let cell = map[y][x];
                let coords = (x, y);

                // do not flash twice
                if cell > 9 && !flashed.contains(&coords) {
                    once = true;
                    flashed.push(coords);

                    // increase energy levels in neighbors
                    increase_surroundings(x, y, map);
                }
            }
        }

        if !once {
            break;
        }
    }

    // reset flashed cells back to 0
    reset_cells(map);

    return flashed.len() as u32;
}

fn simulate(map: &mut Vec<Vec<i32>>) -> u32 {
    // first, the energy level of each cell increases by 1
    increase_energy(map);

    // then start flashing
    let flashes = flash(map);

    return flashes;
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

    // let's fill the borders with i32::MIN
    let mut map: Vec<Vec<i32>> = build_map(max_x, max_y);
    fill_map(&lines, &mut map);

    print_map(&map);

    // run simulation
    let mut flashes = 0;
    let steps = 100;
    for _ in 0..steps {
        let count = simulate(&mut map);
        flashes += count;
    }

    println!("Flashes: {}", flashes);

    print_map(&map);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
