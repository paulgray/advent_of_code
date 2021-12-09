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
                risks += 1 + val;
            }
        }
    }

    println!("Risk levels = {}", risks);
}

fn main() {
    let contents =
        std::fs::read_to_string("test0").expect("Something went wrong when reading the input file");

    star1(&contents);
}
