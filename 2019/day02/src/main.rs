use std::fs::File;
use std::io::prelude::*;

fn execute(p: &mut Vec<i32>) {
    let mut ip = 0;
    loop {
        if p[ip] == 99 {
            break;
        }

        let l = p[ip+1] as usize;
        let r = p[ip+2] as usize;
        let res = p[ip+3] as usize;

        match p[ip] {
            1  => p[res] = p[l] + p[r],
            2  => p[res] = p[l] * p[r],
            y  => println!("Case not handled {}", y),
        }

        ip += 4;
    }
}

fn main() {
    let mut file = File::open("/tmp/input2").unwrap();
    let mut input = String::new();
    let mut noun = 0;
    let mut verb = 0;

    file.read_to_string(&mut input).unwrap();
    let orig = input.split(",")
                    .map(|opcode| opcode.trim_end().parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

    loop {
        let mut p = orig.clone();
        
        p[1] = noun;
        p[2] = verb;

        execute(&mut p);

        if p[0] == 19690720 {
            println!("Result {}", 100 * noun + verb);
            break;
        }

        noun += 1;
        if noun == 100 {
            noun = 0;
            verb += 1;
        }
    }
}

#[test]
fn test() {
    let mut input = vec![1,0,0,0,99];
    execute(&mut input);
    assert_eq!(vec![2,0,0,0,99], input);
    
    input = vec![2,3,0,3,99];
    execute(&mut input);
    assert_eq!(vec![2,3,0,6,99], input);

    input = vec![2,4,4,5,99,0];
    execute(&mut input);
    assert_eq!(vec![2,4,4,5,99,9801], input);

    input = vec![1,1,1,4,99,5,6,0,99];
    execute(&mut input);
    assert_eq!(vec![30,1,1,4,2,5,6,0,99], input);
}