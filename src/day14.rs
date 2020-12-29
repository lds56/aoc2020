use std::collections::HashMap;
use std::collections::VecDeque;

use super::util;
use std::error;

use aoc_runner_derive::{aoc, aoc_generator};

struct Program {
    mask: String,
    mems: Vec<(usize, i32)>,
}

impl Program {
    fn add(&mut self, elem: (usize, i32)) {
        self.mems.push(elem);
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Result<Vec<Program>, Box<dyn error::Error>> {

    let mut programs: Vec<Program> = vec![];

    for line in input.lines() {

        let mut parts = line.split(" = ");
        let (key, val) = (parts.next().ok_or(util::AocError)?, parts.next().ok_or(util::AocError)?);

        if key.starts_with("mask") {
            
            programs.push(Program{
                mask: val.to_string(),
                mems: vec![],
            });
        }
        else if key.starts_with("mem") {

            let mem_idx = key
                .strip_prefix("mem[").ok_or(util::AocError)?
            .strip_suffix("]").ok_or(util::AocError)?;

            programs
                .last_mut().ok_or(util::AocError)?
                .add((
                    mem_idx.parse::<usize>()?,
                    val.parse::<i32>()?,
                ));
        }
        
    }

    Ok(programs)
}

#[aoc(day14, part1)]
fn solvor_part1(programs: &[Program]) -> i64 {

    let mut res = HashMap::new();

    programs
        .iter()
        .for_each(|p| {

            let mask_pair = p.mask
                .chars()
                .fold((0i64, 0i64), |acc, x| {
                    ((acc.0 << 1) + if x == '1' { 1 } else { 0 },
                     (acc.1 << 1) + if x == '0' { 0 } else { 1 })
                });

            p.mems
                .iter()
                .for_each(|(idx, x)| {
                    res.insert(
                        idx.clone(), (*x as i64 | mask_pair.0) & mask_pair.1
                    );
                });


        });

    res
        .values()
        .sum::<i64>()

}

#[aoc(day14, part2)]
fn solvor_part2(programs: &[Program]) -> i64 {

    let mut res = HashMap::new();

    programs
        .iter()
        .for_each(|p| {

            let mut xs = vec![];
            
            let masks = p.mask
                .chars()
                .enumerate()
                .fold((0i64, 0i64), |acc, (idx, x)| {
                    xs.push(p.mask.len() - idx - 1);
                    ((acc.0 << 1) + if x == '1' { 1 } else { 0 },
                     (acc.1 << 1) + if x == '0' { 0 } else { 1 })
                });

            p.mems
                .iter()
                .for_each(|(addr, x)| {

                    let mut new_addrs = vec![(*addr as i64 | masks.0) & masks.1];

                    for idx in &xs {

                    }

                    new_addrs
                        .iter()
                        .for_each(|new_addr| {
                            res.insert(new_addr, x.clone() as i64);
                        })
                });
        });

    res
        .values()
        .sum::<i64>()
}
