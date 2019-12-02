use std::fs::File;
use std::io::prelude::*;

fn execute(p: &mut Vec<i32>) {
    let mut i = 0;
    loop {
        if p[i] == 99 {
            break;
        }

        let l = p[i+1] as usize;
        let r = p[i+2] as usize;
        let res = p[i+3] as usize;

        match p[i] {
            1  => p[res] = p[l] + p[r],
            2  => p[res] = p[l] * p[r],
            y  => println!("Case not handled {}", y),
        }

        i += 4;
    }
}

fn main() {
    let mut file = File::open("/tmp/input2").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();

    let mut p = input.split(",").map(|opcode| opcode.trim_end().parse::<i32>().unwrap())
                     .collect::<Vec<i32>>();
    p[1] = 12;
    p[2] = 2;

    execute(&mut p);

    println!("Result {}", p[0]);
}