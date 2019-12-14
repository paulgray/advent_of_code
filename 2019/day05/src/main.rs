use std::fs::File;
use std::io::prelude::*;

fn parameter(mode : i32, param : i32, p: &mut Vec<i32>) -> i32 {
    match mode {
        0 => return p[param as usize],
        1 => return param,
        y => panic!("Unknown mode {}", y),
    }
}

fn execute(p: &mut Vec<i32>, input : i32) -> i32 {
    let mut ip = 0;
    let mut output = 0;

    loop {
        if p[ip] == 99 {
            break;
        }

        let p1 = p[ip+1];
        let p2 = p[ip+2];
        let p3 = p[ip+3] as usize;

        let opcode = (p[ip] % 10) +  10 * ((p[ip] / 10) % 10);
        let m1 = (p[ip] / 100) % 10;
        let m2 = (p[ip] / 1000) % 10;

        match opcode {
            2  => { p[p3] = parameter(m1, p1, p) * parameter(m2, p2, p); ip += 4 },
            1  => { p[p3] = parameter(m1, p1, p) + parameter(m2, p2, p); ip += 4 },
            3  => { p[p1 as usize] = input; ip += 2 },
            4  => { output = parameter(m1, p1, p); println!("Output {}", output); ip += 2; },
            5  => { if parameter(m1, p1, p) != 0 { ip = parameter(m2, p2, p) as usize; } else { ip += 3; } },
            6  => { if parameter(m1, p1, p) == 0 { ip = parameter(m2, p2, p) as usize; } else { ip += 3; } },
            7  => { if parameter(m1, p1, p) < parameter(m2, p2, p) { p[p3] = 1; } else { p[p3] = 0; }; ip += 4; },
            8  => { if parameter(m1, p1, p) == parameter(m2, p2, p) { p[p3] = 1; } else { p[p3] = 0; }; ip += 4; },
            y  => println!("Case not handled {}", y),
        }
    }

    return output;
}

fn main() {

    let mut file = File::open("/tmp/input5").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let mut p = input.split(",")
                     .map(|opcode| opcode.trim_end().parse::<i32>().unwrap())
                     .collect::<Vec<i32>>();

    execute(&mut p, 5);
}

#[test]
fn test() {
    let mut input = vec![3,9,8,9,10,9,4,9,99,-1,8];
    assert_eq!(0, execute(&mut input, 5));
    assert_eq!(0, execute(&mut input, 15));
    assert_eq!(1, execute(&mut input, 8));

    input = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    assert_eq!(0, execute(&mut input, 0));
    assert_eq!(1, execute(&mut input, 11));

    input = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    assert_eq!(999, execute(&mut input, 3));
    assert_eq!(1000, execute(&mut input, 8));
    assert_eq!(1001, execute(&mut input, 18));
}
