use std::fs::File;
use std::io::prelude::*;

fn get_value(p : &mut Vec<i64>, relative_base : i64, ip : usize, mode : i32, param_no : usize) -> i64 {
    let address = get_ip(p, relative_base, ip, mode, param_no);
    return p[address];
}

fn set_value(p : &mut Vec<i64>, relative_base : i64, ip : usize, mode : i32, param_no : usize, value : i64) {
    let address = get_ip(p, relative_base, ip, mode, param_no);
    p[address] = value;
}

fn get_ip(p : &mut Vec<i64>, relative_base : i64, ip : usize, mode : i32, param_no : usize) -> usize {
    match mode {
        0 => return p[ip + param_no] as usize,
        1 => return ip + param_no,
        2 => return (relative_base + p[ip + param_no]) as usize,
        y => panic!("Unknown mode {}", y),
    }
}

fn execute(p : &mut Vec<i64>, input : i64) -> i64 {
    let mut ip : usize = 0;
    let mut relative_base : i64 = 0;
    let mut output = 0;

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
            1  => { 
                    let l = get_value(p, relative_base, ip, m1, 1); 
                    let r = get_value(p, relative_base, ip, m2, 2);
                    set_value(p, relative_base, ip, m3, 3, l + r); 
                    ip += 4;
                  },
            2  => { 
                    let l = get_value(p, relative_base, ip, m1, 1); 
                    let r = get_value(p, relative_base, ip, m2, 2);
                    set_value(p, relative_base, ip, m3, 3, l * r); 
                    ip += 4;
                  },
            3  => { 
                    set_value(p, relative_base, ip, m1, 1, input); 
                    ip += 2;
                  },
            4  => { 
                    output = get_value(p, relative_base, ip, m1, 1); 
                    println!("Output {}", output); 
                    ip += 2; 
                  },
            5  => { 
                    if get_value(p, relative_base, ip, m1, 1) != 0 { 
                        ip = get_value(p, relative_base, ip, m2, 2) as usize; 
                    } else { 
                        ip += 3; 
                    } 
                  },
            6  => { 
                    if get_value(p, relative_base, ip, m1, 1) == 0 { 
                        ip = get_value(p, relative_base, ip, m2, 2) as usize; 
                    } else { 
                        ip += 3; 
                    } 
                  },
            7  => { 
                    if get_value(p, relative_base, ip, m1, 1) < get_value(p, relative_base, ip, m2, 2) { 
                        set_value(p, relative_base, ip, m3, 3, 1); 
                    } else { 
                        set_value(p, relative_base, ip, m3, 3, 0); 
                    }; 
                    ip += 4; 
                  },
            8  => { 
                    if get_value(p, relative_base, ip, m1, 1) == get_value(p, relative_base, ip, m2, 2) { 
                        set_value(p, relative_base, ip, m3, 3, 1); 
                    } else { 
                        set_value(p, relative_base, ip, m3, 3, 0); 
                    }; 
                    ip += 4; 
                  },
            9  => { 
                    let delta = get_value(p, relative_base, ip, m1, 1); 
                    relative_base += delta; 
                    ip += 2; 
                  }, 
            y  => println!("Case not handled {}", y),
        }
    }

    return output;
}

fn main() {
    let mut file = File::open("/tmp/input9").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let mut p = input.split(",")
                     .map(|opcode| opcode.trim_end().parse::<i64>().unwrap())
                     .collect::<Vec<i64>>();

    execute(&mut p, 2);
}

#[test]
fn test() {
    let mut p = vec![3,9,8,9,10,9,4,9,99,-1,8];
    assert_eq!(0, execute(&mut p, 5));
    assert_eq!(0, execute(&mut p, 15));
    assert_eq!(1, execute(&mut p, 8));

    p = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    assert_eq!(0, execute(&mut p, 0));
    assert_eq!(1, execute(&mut p, 11));

    p = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
    assert_eq!(999, execute(&mut p, 3));
    assert_eq!(1000, execute(&mut p, 8));
    assert_eq!(1001, execute(&mut p, 18));

    p = vec![1102,34915192,34915192,7,4,7,99,0];
    assert_eq!(1219070632396864, execute(&mut p, 0));

    p = vec![104,1125899906842624,99];
    assert_eq!(1125899906842624, execute(&mut p, 0));

    p = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    assert_eq!(99, execute(&mut p, 0));
}