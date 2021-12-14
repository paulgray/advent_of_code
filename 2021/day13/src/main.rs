fn build_map(x: usize, y: usize) -> Vec<Vec<bool>> {
    let mut map = Vec::new();

    let mut row: Vec<bool> = Vec::new();
    let mut i = 0;
    while i < x {
        row.push(false);
        i += 1;
    }

    let mut j = 0;
    while j < y {
        map.push(row.clone());
        j += 1;
    }

    return map;
}

fn print_map(map: &Vec<Vec<bool>>, max_x: usize, max_y: usize) {
    for y in 0..max_y {
        let row = &map[y];
        for x in 0..max_x {
            if row[x] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn count_dots(map: &Vec<Vec<bool>>, max_x: usize, max_y: usize) -> u32 {
    let mut c = 0;

    for y in 0..max_y {
        let row = &map[y];
        for x in 0..max_x {
            if row[x] {
                c += 1;
            }
        }
    }

    return c;
}

fn star1(input: &str) {
    let lines = input
        .trim()
        .split("\n")
        .map(|line| line.trim())
        .collect::<Vec<&str>>();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut coords: Vec<(usize, usize)> = Vec::new();
    let mut i = 0;

    // parse dots coordinates
    loop {
        let line = lines[i];
        i += 1;

        // this is a separator
        if line.is_empty() {
            break;
        }

        let s = line.split(",").collect::<Vec<&str>>();
        let (x, y) = (
            s[0].parse::<usize>().unwrap(),
            s[1].parse::<usize>().unwrap(),
        );
        coords.push((x, y));

        // find our initial map size
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }
    max_x += 1;
    max_y += 1;

    // parse folding instructions, format:
    // (is_fold_along_y_axis, position)
    let mut folds: Vec<(bool, usize)> = Vec::new();
    while i < lines.len() {
        let line = lines[i];
        let pos = line.split("=").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();
        folds.push((line.starts_with("fold along y"), pos));

        i += 1;
    }

    // prepare our map
    let mut map = build_map(max_x, max_y);
    for (x, y) in coords {
        let row = map.get_mut(y).unwrap();
        row[x] = true;
    }
    print_map(&map, max_x, max_y);

    // start folding the map
    for (is_y, position) in folds {
        // this is a fold along the y axis
        if is_y {
            // in this case the 'x' coord remains unchanged
            // and we need to mark true coordinates (x, y || position - y)
            //
            // the size of our map shrinks from
            // (max_x, max_y)
            //   to
            // (max_x, max_y/2)
            for y in 0..position {
                let mut res: Vec<bool> = Vec::new();

                let curr_row = &map[y];
                let mirrored_row = &map[max_y - y - 1];

                for x in 0..max_x {
                    res.push(curr_row[x] || mirrored_row[x]);
                }

                map[y] = res;
            }
            max_y /= 2;
        } else {
            // this time 'y' coords stays the same
            // coords that are marked true: (x || position - x, y)
            //
            // the size of map shrinks from
            // (max_x, max_y)
            //   to
            // (max_x/2, max_y)
            for y in 0..max_y {
                let row = &mut map[y];

                for x in 0..position {
                    row[x] = row[x] || row[max_x - x - 1];
                }
            }
            max_x /= 2;
        }

        break;
    }

    println!("");
    print_map(&map, max_x, max_y);

    let count = count_dots(&map, max_x, max_y);
    println!("Count: {}", count);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
