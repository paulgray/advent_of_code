extern crate permutohedron;

use std::fs::File;
use std::io::prelude::*;
use permutohedron::LexicalPermutation;

fn parameter(mode : i32, param : i32, p: &mut Vec<i32>) -> i32 {
    match mode {
        0 => return p[param as usize],
        1 => return param,
        y => panic!("Unknown mode {}", y),
    } 
}

fn execute(p: &mut Vec<i32>, ip : &mut usize, phase : i32, input : i32, skip_phase : bool) -> (i32, bool) {
    let mut output = 0;
    let mut inputs = phase;
    let is_done;
    
    if skip_phase {
        inputs = input;
    }

    println!("Running with phase {}, input {}", phase, input);

    loop {
        if p[*ip] == 99 {
            is_done = true;
            break;
        }

        let p1 = p[*ip+1];
        let p2 = p[*ip+2];
        let p3 = p[*ip+3] as usize;

        let opcode = (p[*ip] % 10) +  10 * ((p[*ip] / 10) % 10);
        let m1 = (p[*ip] / 100) % 10;
        let m2 = (p[*ip] / 1000) % 10;

        match opcode {
            2  => { p[p3] = parameter(m1, p1, p) * parameter(m2, p2, p); *ip += 4 },
            1  => { p[p3] = parameter(m1, p1, p) + parameter(m2, p2, p); *ip += 4 },
            3  => { p[p1 as usize] = inputs; inputs = input; *ip += 2 },
            4  => { output = parameter(m1, p1, p); *ip += 2; is_done = false; break; },
            5  => { if parameter(m1, p1, p) != 0 { *ip = parameter(m2, p2, p) as usize; } else { *ip += 3; } },
            6  => { if parameter(m1, p1, p) == 0 { *ip = parameter(m2, p2, p) as usize; } else { *ip += 3; } },
            7  => { if parameter(m1, p1, p) < parameter(m2, p2, p) { p[p3] = 1; } else { p[p3] = 0; }; *ip += 4; },
            8  => { if parameter(m1, p1, p) == parameter(m2, p2, p) { p[p3] = 1; } else { p[p3] = 0; }; *ip += 4; },
            y  => println!("Case not handled {}", y),
        }
    }

    return (output, is_done);
}

fn chain_execute(p: &mut Vec<i32>, phases : Vec<i32>) -> i32 {
    let mut p0 = p.clone();
    let mut p1 = p.clone();
    let mut p2 = p.clone();
    let mut p3 = p.clone();
    let mut p4 = p.clone();

    let mut ip0 = 0;
    let mut ip1 = 0;
    let mut ip2 = 0;
    let mut ip3 = 0;
    let mut ip4 = 0;

    let mut output : i32 = 0;
    let mut skip_phase : bool = false;

    loop {
        let (tmp_output0, is_done) = execute(&mut p0, &mut ip0, phases[0], output, skip_phase);
        let (tmp_output1, _) = execute(&mut p1, &mut ip1, phases[1], tmp_output0, skip_phase);
        let (tmp_output2, _) = execute(&mut p2, &mut ip2, phases[2], tmp_output1, skip_phase);
        let (tmp_output3, _) = execute(&mut p3, &mut ip3, phases[3], tmp_output2, skip_phase);
        let (tmp_output4, _) = execute(&mut p4, &mut ip4, phases[4], tmp_output3, skip_phase);
        if is_done {
            return output;
        }

        output = tmp_output4;
        skip_phase = true;
    }
}

fn find_max_output(p: &mut Vec<i32>) -> i32 {
    let mut phases : Vec<i32> = vec![5, 6, 7, 8, 9];
    let mut max = 0;
    loop {
        let output = chain_execute(p, phases.clone());
        println!("Final output {}", output);
        if output > max {
            max = output;
        }

        if !phases.next_permutation() {
            break;
        }
    }

    return max;
}

fn main() {
    let mut file = File::open("/tmp/input7").unwrap();
    let mut input = String::new();

    file.read_to_string(&mut input).unwrap();
    let mut p = input.split(",")
                     .map(|opcode| opcode.trim_end().parse::<i32>().unwrap())
                     .collect::<Vec<i32>>();

    let max = find_max_output(&mut p);
    println!("Max: {}", max);
}

#[test]
fn test() {
    let mut input = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
    assert_eq!(139629729, find_max_output(&mut input));

    /*input = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
    assert_eq!(54321, find_max_output(&mut input));

    input = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
    assert_eq!(65210, find_max_output(&mut input));*/
}