fn star1(input: &str) {
    let score: u32 = input.split("\n").fold(0, |s, line| {
        let mut stack: Vec<char> = Vec::new();
        let mut line_score = 0;
        for c in line.trim().chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '<' => stack.push(c),
                ')' => {
                    let last = stack.pop();
                    if last.is_none() || last.unwrap() != '(' {
                        line_score = 3;
                        break;
                    }
                }
                ']' => {
                    let last = stack.pop();
                    if last.is_none() || last.unwrap() != '[' {
                        line_score = 57;
                        break;
                    }
                }
                '}' => {
                    let last = stack.pop();
                    if last.is_none() || last.unwrap() != '{' {
                        line_score = 1197;
                        break;
                    }
                }
                '>' => {
                    let last = stack.pop();
                    if last.is_none() || last.unwrap() != '<' {
                        line_score = 25137;
                        break;
                    }
                }
                _ => (),
            }
        }

        s + line_score
    });

    println!("Score: {}", score);
}

fn star2(input: &str) {
    let mut scores: Vec<u128> = Vec::new();
    input.split("\n").for_each(|line| {
        let mut stack: Vec<char> = Vec::new();
        let mut keep = true;
        for c in line.trim().chars() {
            match c {
                '(' => stack.push(c),
                '[' => stack.push(c),
                '{' => stack.push(c),
                '<' => stack.push(c),
                ')' => {
                    let last = stack.pop();
                    if last.is_none() || last.unwrap() != '(' {
                        keep = false;
                        break;
                    }
                }
                ']' => {
                    let last = stack.pop();
                    if last.is_none() || last.unwrap() != '[' {
                        keep = false;
                        break;
                    }
                }
                '}' => {
                    let last = stack.pop();
                    if last.is_none() || last.unwrap() != '{' {
                        keep = false;
                        break;
                    }
                }
                '>' => {
                    let last = stack.pop();
                    if last.is_none() || last.unwrap() != '<' {
                        keep = false;
                        break;
                    }
                }
                _ => (),
            }
        }

        if keep && !stack.is_empty() {
            let mut line_score: u128 = 0;
            stack.reverse();
            for leftover in stack {
                line_score *= 5;

                line_score += match leftover {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
            }

            scores.push(line_score);
        }
    });

    scores.sort();
    let mid_idx = scores.len() / 2;

    println!("Score: {}", scores[mid_idx]);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
    star2(&contents);
}
