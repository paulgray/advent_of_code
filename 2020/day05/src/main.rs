use std::fs::File;
use std::io::prelude::*;

fn parameter(mode : i32, param : i32, p: &mut Vec<i32>) -> i32 {
    match mode {
        0 => return p[param as usize],
        1 => return param,
        y => panic!("Unknown mode {}", y),
    } 
}

fn execute(p: &mut Vec<i32>) {
    let mut ip = 0;
    let mut output = 0;

    loop {
        if p[ip] == 99 {
            break;
        }

        let input = 1;
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
            4  => { output = parameter(m1, p1, p); ip += 2; println!("Output {}", output); },
            y  => println!("Case not handled {}", y),
        }
    }
}

fn main() {

    let mut file = File::open("/tmp/input5").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let mut p = input.split(",")
                     .map(|opcode| opcode.trim_end().parse::<i32>().unwrap())
                     .collect::<Vec<i32>>();

    execute(&mut p);
}

#[test]
fn test() {

}