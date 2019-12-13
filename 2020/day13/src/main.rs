use std::fs::File;
use std::io::prelude::*;

fn get_value(p: &mut Vec<i64>, relative_base: i64, ip: usize, mode: i32, param_no: usize) -> i64 {
    let address = get_ip(p, relative_base, ip, mode, param_no);
    return p[address];
}

fn set_value(
    p: &mut Vec<i64>,
    relative_base: i64,
    ip: usize,
    mode: i32,
    param_no: usize,
    value: i64,
) {
    let address = get_ip(p, relative_base, ip, mode, param_no);
    p[address] = value;
}

fn get_ip(p: &mut Vec<i64>, relative_base: i64, ip: usize, mode: i32, param_no: usize) -> usize {
    match mode {
        0 => return p[ip + param_no] as usize,
        1 => return ip + param_no,
        2 => return (relative_base + p[ip + param_no]) as usize,
        y => panic!("Unknown mode {}", y),
    }
}

fn execute(p: &mut Vec<i64>, input: i64) -> Vec<i64> {
    let mut ip: usize = 0;
    let mut relative_base: i64 = 0;
    let mut outputs = Vec::new();

    p.resize(10000, 0);

    loop {
        if p[ip] == 99 {
            break;
        }

        let opcode = (p[ip] % 10) + 10 * ((p[ip] / 10) % 10);
        let m1 = ((p[ip] / 100) % 10) as i32;
        let m2 = ((p[ip] / 1000) % 10) as i32;
        let m3 = ((p[ip] / 10000) % 10) as i32;

        match opcode {
            1 => {
                let l = get_value(p, relative_base, ip, m1, 1);
                let r = get_value(p, relative_base, ip, m2, 2);
                set_value(p, relative_base, ip, m3, 3, l + r);
                ip += 4;
            }
            2 => {
                let l = get_value(p, relative_base, ip, m1, 1);
                let r = get_value(p, relative_base, ip, m2, 2);
                set_value(p, relative_base, ip, m3, 3, l * r);
                ip += 4;
            }
            3 => {
                set_value(p, relative_base, ip, m1, 1, input);
                ip += 2;
            }
            4 => {
                let output = get_value(p, relative_base, ip, m1, 1);
                outputs.push(output);
                println!("Output {}", output);
                ip += 2;
            }
            5 => {
                if get_value(p, relative_base, ip, m1, 1) != 0 {
                    ip = get_value(p, relative_base, ip, m2, 2) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                if get_value(p, relative_base, ip, m1, 1) == 0 {
                    ip = get_value(p, relative_base, ip, m2, 2) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                if get_value(p, relative_base, ip, m1, 1) < get_value(p, relative_base, ip, m2, 2) {
                    set_value(p, relative_base, ip, m3, 3, 1);
                } else {
                    set_value(p, relative_base, ip, m3, 3, 0);
                };
                ip += 4;
            }
            8 => {
                if get_value(p, relative_base, ip, m1, 1) == get_value(p, relative_base, ip, m2, 2)
                {
                    set_value(p, relative_base, ip, m3, 3, 1);
                } else {
                    set_value(p, relative_base, ip, m3, 3, 0);
                };
                ip += 4;
            }
            9 => {
                let delta = get_value(p, relative_base, ip, m1, 1);
                relative_base += delta;
                ip += 2;
            }
            y => println!("Case not handled {}", y),
        }
    }

    return outputs;
}

fn main() {
    let mut file = File::open("/tmp/input13").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let mut p = input
        .split(",")
        .map(|opcode| opcode.trim_end().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let output = execute(&mut p, 2);
    let block_tiles = output
        .chunks(3)
        .filter(|chunk| *chunk.get(2).unwrap() == 2)
        .count();
    println!("Block tiles: {}", block_tiles);
}

#[test]
fn test() {}
