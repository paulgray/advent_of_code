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

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
