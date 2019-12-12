use std::collections::{HashMap, HashSet};
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

fn get_input(key: (i32, i32), inputs: &HashMap<(i32, i32), i32>) -> i64 {
    if inputs.contains_key(&key) {
        return *inputs.get(&key).unwrap() as i64;
    }
    return 0;
}

fn set_output(key: (i32, i32), value: i32, inputs: &mut HashMap<(i32, i32), i32>) {
    inputs.insert(key, value);
}

fn change_direction(direction: char, turn: i32) -> char {
    if turn == 0 {
        // turning left by 90 degrees
        match direction {
            'u' => return 'l',
            'd' => return 'r',
            'l' => return 'd',
            'r' => return 'u',
            y => println!("Unknown direction {}", y),
        }
    } else {
        // turning right by 90 degrees
        match direction {
            'u' => return 'r',
            'd' => return 'l',
            'l' => return 'u',
            'r' => return 'd',
            y => println!("Unknown direction {}", y),
        }
    }

    return 'X';
}

fn print_output(output: &HashMap<(i32, i32), i32>) {
    let mut min_x: i32 = 0;
    let mut min_y: i32 = 0;
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;
    // at first let's normalize the dimensions
    for (x, y) in output.keys() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    //println!("{}", output.keys());

    // print the entire sign
    for j in min_y..=max_y {
        for i in min_x..=max_x {
            if output.contains_key(&(i, j)) && *output.get(&(i, j)).unwrap() == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn execute(p: &mut Vec<i64>, inputs: &mut HashMap<(i32, i32), i32>) -> usize {
    let mut ip: usize = 0;
    let mut relative_base: i64 = 0;
    let mut x = 0;
    let mut y = 0;
    let mut direction = 'u';
    let mut read_direction = false;
    let mut painted = HashSet::new();
    painted.insert((0, 0));
    inputs.insert((0, 0), 1);

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
                set_value(p, relative_base, ip, m1, 1, get_input((x, y), &inputs));
                ip += 2;
            }
            4 => {
                let output = get_value(p, relative_base, ip, m1, 1);
                if !read_direction {
                    // we're outputting the color
                    set_output((x, y), output as i32, inputs);
                    painted.insert((x, y));
                } else {
                    // we're changing direction
                    direction = change_direction(direction, output as i32);

                    // move robot in that direction
                    match direction {
                        'u' => y -= 1,
                        'd' => y += 1,
                        'l' => x -= 1,
                        'r' => x += 1,
                        z => println!("Unknown direction {}", z),
                    }
                }
                read_direction = !read_direction;
                //println!("Output {}", output);
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

    return painted.len();
}

fn main() {
    let mut file = File::open("/tmp/input11").unwrap();
    let mut intcode = String::new();

    file.read_to_string(&mut intcode).unwrap();
    let mut p = intcode
        .split(",")
        .map(|opcode| opcode.trim_end().parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut inputs = HashMap::new();
    let painted = execute(&mut p, &mut inputs);
    print_output(&inputs);

    println!("Painted {} panels", painted);
}

#[test]
fn test() {}
