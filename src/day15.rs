// use super::util;
// use std::error;

use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<i32> { // Result<Vec<i32>, Box<dyn error::Error>> {

    input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap() )
        .collect()
}

#[aoc(day15, part1)]
fn solvor_part1(res: &[i32]) -> i32 {

    let mut num2idx = HashMap::new();
    let mut lastx: i32 = 0;

    (0..2020)
        .for_each(|idx| {

            let x = if idx < res.len() {
                res[idx].clone()
            } else {
                if num2idx.contains_key(&lastx) {
                    (idx - num2idx.get(&lastx).unwrap()) as i32
                } else {
                    0
                }
            };

            num2idx.insert(lastx, idx);
            lastx = x;
                                   
        });

    lastx
}
