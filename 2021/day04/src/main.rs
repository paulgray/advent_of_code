struct Seq {
    numbers: Vec<u32>,
    hits: u32,
}

struct Board {
    rows: Vec<Seq>,
    columns: Vec<Seq>,
    unmarked: Vec<u32>,
}

fn parse_boards(lines: &Vec<&str>) -> Vec<Board> {
    let mut boards: Vec<Board> = Vec::new();

    let mut i = 2;
    while i < lines.len() {
        let mut j = 0;
        let mut board = Board {
            rows: Vec::new(),
            columns: Vec::new(),
            unmarked: Vec::new(),
        };

        // construct a row
        while j < 5 {
            let numbers = lines[i]
                .trim()
                .split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap_or_default())
                .collect::<Vec<u32>>();
            board.unmarked.append(&mut numbers.clone());
            let row = Seq {
                numbers: numbers,
                hits: 0,
            };
            board.rows.push(row);

            j += 1;
            i += 1;
        }

        // reconstruct columns, based on rows
        j = 0;
        while j < 5 {
            let column = Seq {
                numbers: vec![
                    board.rows[j].numbers[0],
                    board.rows[j].numbers[1],
                    board.rows[j].numbers[2],
                    board.rows[j].numbers[3],
                    board.rows[j].numbers[4],
                ],
                hits: 0,
            };
            board.columns.push(column);

            j += 1;
        }

        boards.push(board);
        i += 1;
    }

    return boards;
}

fn print_board(board: &Board) {
    for row in &board.rows {
        println!(
            "{} {} {} {} {}",
            row.numbers[0], row.numbers[1], row.numbers[2], row.numbers[3], row.numbers[4]
        );
    }
    println!();
}

fn draw_number(n: u32, boards: &mut Vec<Board>) {
    for board in boards {
        for row in &mut board.rows {
            if row.numbers.contains(&n) {
                row.hits += 1;
            }
        }

        for column in &mut board.columns {
            if column.numbers.contains(&n) {
                column.hits += 1;
            }
        }

        board.unmarked.retain(|&x| x != n);
    }
}

fn print_score(n: u32, board: &Board) {
    let s: u32 = board.unmarked.iter().sum();
    println!("Sum {}, winning number {}, solution {}", s, n, s * n);
    print_board(board);
}

fn validate_boards(n: u32, boards: &Vec<Board>) -> bool {
    for board in boards {
        for row in &board.rows {
            if row.hits == 5 {
                print_score(n, &board);
                return true;
            }
        }
        for column in &board.columns {
            if column.hits == 5 {
                print_score(n, &board);
                return true;
            }
        }
    }

    return false;
}

fn star1(input: &str) {
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let numbers = lines[0]
        .trim()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap_or_default())
        .collect::<Vec<u32>>();

    let mut boards = parse_boards(&lines);

    for number in numbers {
        draw_number(number, &mut boards);

        if validate_boards(number, &boards) {
            break;
        }
    }

    //for board in boards {
    //    print_board(&board);
    //}
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
