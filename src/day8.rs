use std::collections::HashSet;
// use std::cmp;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<(String, i32)> {

    input
        .lines()
        .map(|line| {
            let cmd = &line[..3];
            let num = line[5..].parse::<i32>().unwrap();
            (cmd.to_string(),
             if line.chars().nth(4).unwrap() == '-' { -num } else { num } )
        })
        .collect()
}

fn next(status: (i32, i32), instr: &[(String, i32)]) -> ((i32, i32), i32) {

    let (mut pc, mut acc) = status;
    let (cmd, num) = &instr[pc as usize];
    let mut maybe_pc = 0i32;

    pc += match cmd.as_str() {
        "nop" => { maybe_pc = pc + *num; 1 },
        "acc" => { acc += *num; 1 },
        "jmp" => { maybe_pc = pc + 1; *num },
        _ => 0,
    };

    ((pc, acc), maybe_pc)
    
}

#[aoc(day8, part1)]
fn solvor_part1(inputs: &[(String, i32)]) -> i32 {

    // for (cmd, num) in inputs.iter() {
       // println!("{} | {}", cmd, num);
    // }

    let mut visited: HashSet<i32> = HashSet::new();
    let mut status = (0i32, 0i32);
    
    while !visited.contains(&status.0) {
        // visit pc
        visited.insert(status.0);
        status = next(status, inputs).0;
    }

    status.1
}

#[aoc(day8, part2)]
fn solvor_part2(inputs: &[(String, i32)]) -> i32 {

    let mut visited: HashSet<i32> = HashSet::new();
    let mut status = (0i32, 0i32);

    while !visited.contains(&status.0) {
        visited.insert(status.0);

        let (ret_status, maybe_pc) = next(status, inputs);
        status = ret_status;

        let mut maybe_visited: HashSet<i32> = HashSet::new();
        let mut maybe_status = (maybe_pc, ret_status.1);
        
        while !visited.contains(&maybe_status.0) &&
              !maybe_visited.contains(&maybe_status.0) {

            maybe_visited.insert(maybe_status.0);
            maybe_status = next(maybe_status, inputs).0;
            if maybe_status.0 >= inputs.len() as i32 {
                return maybe_status.1;
            }
        }

    }

    0

}
