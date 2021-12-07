struct Line {
    p1: (usize, usize),
    p2: (usize, usize),
}

fn parse_lines(input: &str) -> Vec<Line> {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut m: Vec<Line> = Vec::new();

    for line in lines {
        let parts = line.split(" -> ").collect::<Vec<&str>>();
        let p1 = parts[0]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap_or_default())
            .collect::<Vec<usize>>();
        let p2 = parts[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap_or_default())
            .collect::<Vec<usize>>();

        let l = Line {
            p1: (p1[0], p1[1]),
            p2: (p2[0], p2[1]),
        };
        m.push(l);
    }

    return m;
}

fn find_maxes(m: &Vec<Line>) -> (usize, usize) {
    let mut x = 0;
    let mut y = 0;

    for line in m {
        if line.p1.0 > x {
            x = line.p1.0;
        }
        if line.p2.0 > x {
            x = line.p2.0;
        }

        if line.p1.1 > y {
            y = line.p1.1;
        }
        if line.p2.1 > y {
            y = line.p2.1;
        }
    }

    return (x + 1, y + 1);
}

fn fill_map(lines: &Vec<Line>, map: &mut Vec<Vec<u32>>) {
    for line in lines {
        let dx = i32::abs(i32::try_from(line.p1.0).unwrap() - i32::try_from(line.p2.0).unwrap());
        let dy = i32::abs(i32::try_from(line.p1.1).unwrap() - i32::try_from(line.p2.1).unwrap());
        // consider only horizontal, vertical or diagonal lines
        if line.p1.0 != line.p2.0 && line.p1.1 != line.p2.1 && dx != dy {
            continue;
        }

        // this is a vertical line
        if line.p1.0 == line.p2.0 {
            let (s, e) = if line.p1.1 > line.p2.1 {
                (line.p2.1, line.p1.1)
            } else {
                (line.p1.1, line.p2.1)
            };

            for i in s..=e {
                let row = map.get_mut(i).unwrap();
                row[line.p1.0] += 1;
            }
        }

        // this is a horizontal line
        if line.p1.1 == line.p2.1 {
            let (s, e) = if line.p1.0 > line.p2.0 {
                (line.p2.0, line.p1.0)
            } else {
                (line.p1.0, line.p2.0)
            };

            for i in s..=e {
                let row = map.get_mut(line.p1.1).unwrap();
                row[i] += 1;
            }
        }

        // this is a diagonal line
        if dx == dy {
            println!(
                "Diagonal line ({}, {}) -> ({}, {})",
                line.p1.0, line.p1.1, line.p2.0, line.p2.1
            );

            // end x == sx + dx
            // starting point is always the one with lower 'x' coord
            let (p1, p2) = if line.p1.0 < line.p2.0 {
                (line.p1, line.p2)
            } else {
                (line.p2, line.p1)
            };

            // line is going down!
            if p1.1 < p2.1 {
                for i in 0..=dx {
                    let usize_i = usize::try_from(i).unwrap();
                    let row = map.get_mut(p1.1 + usize_i).unwrap();
                    row[p1.0 + usize_i] += 1;
                }
            }
            // line is going up
            else {
                for i in 0..=dx {
                    let usize_i = usize::try_from(i).unwrap();
                    let row = map.get_mut(p1.1 - usize_i).unwrap();
                    row[p1.0 + usize_i] += 1;
                }
            }
        }
    }
}

fn build_map(m: &Vec<Line>, max_x: usize, max_y: usize) -> Vec<Vec<u32>> {
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

    // make the first pass
    fill_map(&m, &mut map);

    return map;
}

fn print_map(map: &Vec<Vec<u32>>) {
    for row in map {
        for cell in row {
            if *cell == 0 {
                print!(".");
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }
}

fn count_overlapping(map: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;

    for row in map {
        for cell in row {
            if *cell > 1 {
                count += 1;
            }
        }
    }

    return count;
}

fn star1(input: &str) {
    let lines = parse_lines(input);

    let (max_x, max_y) = find_maxes(&lines);
    let map = build_map(&lines, max_x, max_y);

    //print_map(&map);

    println!("Overlapping points {}", count_overlapping(&map));
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
