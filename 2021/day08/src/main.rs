fn star1(input: &str) {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut counter = 0;

    for line in lines {
        let output = line.trim().split("|").collect::<Vec<&str>>()[1]
            .trim()
            .split(" ")
            .map(|x| x.trim());

        for digit in output {
            match digit.len() {
                2 =>
                // this is "1"
                {
                    counter += 1
                }
                4 =>
                // this is "4"
                {
                    counter += 1
                }
                3 =>
                // this is "7"
                {
                    counter += 1
                }
                7 =>
                // this is "8"
                {
                    counter += 1
                }
                _ => (),
            }
        }
    }

    println!("Counter: {}", counter);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
}
