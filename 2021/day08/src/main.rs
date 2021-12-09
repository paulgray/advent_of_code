use std::collections::HashMap;

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

fn find_four(four: &str, nines: &Vec<&str>) -> usize {
    let a = four.chars().nth(0).unwrap();
    let b = four.chars().nth(1).unwrap();
    let c = four.chars().nth(2).unwrap();
    let d = four.chars().nth(3).unwrap();

    let mut i = 0;
    for nine in nines {
        if nine.find(a).is_some()
            && nine.find(b).is_some()
            && nine.find(c).is_some()
            && nine.find(d).is_some()
        {
            return i;
        }
        i += 1;
    }

    return 0;
}

fn find_one(one: &str, zeros: &Vec<&str>) -> usize {
    let a = one.chars().nth(0).unwrap();
    let b = one.chars().nth(1).unwrap();

    let mut i = 0;
    for zero in zeros {
        if zero.find(a).is_some() && zero.find(b).is_some() {
            return i;
        }
        i += 1;
    }

    return 0;
}

fn find_six(six: &str, fives: &Vec<&str>) -> usize {
    let mut i = 0;
    for five in fives {
        let a = five.chars().nth(0).unwrap();
        let b = five.chars().nth(1).unwrap();
        let c = five.chars().nth(2).unwrap();
        let d = five.chars().nth(3).unwrap();
        let e = five.chars().nth(4).unwrap();

        if six.find(a).is_some()
            && six.find(b).is_some()
            && six.find(c).is_some()
            && six.find(d).is_some()
            && six.find(e).is_some()
        {
            return i;
        }

        i += 1;
    }

    return 0;
}

fn decode_input(left: &Vec<String>, output: &Vec<String>) -> i32 {
    let mut mapping: HashMap<&str, i32> = HashMap::new();

    // let's identify 1, 4, 7, 8 digits

    // a difference between 1 and 7 is the top segment
    let one = left
        .iter()
        .filter(|n| n.len() == 2)
        .map(|n| n.as_str())
        .collect::<Vec<&str>>()[0];
    let four = left
        .iter()
        .filter(|&n| n.len() == 4)
        .map(|n| n.as_str())
        .collect::<Vec<&str>>()[0];
    let seven = left
        .iter()
        .filter(|&n| n.len() == 3)
        .map(|n| n.as_str())
        .collect::<Vec<&str>>()[0];
    let eight = left
        .iter()
        .filter(|&n| n.len() == 7)
        .map(|n| n.as_str())
        .collect::<Vec<&str>>()[0];

    // nine is the one with 6 segments, and 4 included
    let mut nines = left
        .iter()
        .filter(|&n| n.len() == 6)
        .map(|n| n.as_str())
        .collect::<Vec<&str>>();
    let nine_idx = find_four(&four, &nines);
    let nine = nines[nine_idx];
    nines.remove(nine_idx);

    // zero is the one with 6 segments, and 1 included
    let zero_idx = find_one(&one, &nines);
    let zero = nines[zero_idx];
    nines.remove(zero_idx);

    // six is the last one with 6 segments
    let six = nines[0];

    // three is the one with 5 segments and 1 included
    let mut threes = left
        .iter()
        .filter(|&n| n.len() == 5)
        .map(|n| n.as_str())
        .collect::<Vec<&str>>();
    let three_idx = find_one(&one, &threes);
    let three = threes[three_idx];
    threes.remove(three_idx);

    // five is included within six
    let five_idx = find_six(&six, &threes);
    let five = threes[five_idx];
    threes.remove(five_idx);

    // two is the only one left
    let two = threes[0];

    // initialize the values
    mapping.insert(zero, 0);
    mapping.insert(one, 1);
    mapping.insert(two, 2);
    mapping.insert(three, 3);
    mapping.insert(four, 4);
    mapping.insert(five, 5);
    mapping.insert(six, 6);
    mapping.insert(seven, 7);
    mapping.insert(eight, 8);
    mapping.insert(nine, 9);

    for (key, digit) in &mapping {
        println!("{} == {}", key, digit);
    }
    for o in output {
        println!("Looking for {}", o);
    }

    let sum = 1000 * mapping.get(output[0].as_str()).unwrap()
        + 100 * mapping.get(output[1].as_str()).unwrap()
        + 10 * mapping.get(output[2].as_str()).unwrap()
        + mapping.get(output[3].as_str()).unwrap();

    println!("Value: {}", sum);
    return sum;
}

fn star2(inputs: &str) {
    let lines: Vec<&str> = inputs.trim().split("\n").collect();
    let mut sum = 0;

    for line in lines {
        let input = line.split("|").collect::<Vec<&str>>();
        let res = decode_input(
            &input[0]
                .split_ascii_whitespace()
                .map(|x| {
                    let mut cs: Vec<char> = x.trim().chars().collect();
                    cs.sort();
                    cs.into_iter().collect::<String>()
                })
                .collect(),
            &input[1]
                .split_ascii_whitespace()
                .map(|x| {
                    let mut cs: Vec<char> = x.trim().chars().collect();
                    cs.sort();
                    cs.into_iter().collect::<String>()
                })
                .collect(),
        );

        sum += res;
    }

    println!("Sum: {}", sum);
}

fn main() {
    let contents =
        std::fs::read_to_string("test1").expect("Something went wrong when reading the input file");

    star1(&contents);
    star2(&contents);
}
