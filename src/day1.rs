use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {

    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap() )
        .collect::<Vec<i32>>()
        
}

#[aoc(day1, part1)]
pub fn solvor_part1(inputs: &[i32]) -> i64 {

    /*
    for x in inputs.iter() {
        println!("{}", x);
    }
    */

    let (_, ans) = inputs
        .iter()
        .flat_map(|&x| inputs.iter().map(move |&y| -> (i32, i64) { (x + y, (x * y).into()) }) )
        .find(|&(sum, _)| sum == 2020i32)
        .unwrap();

    ans
}

#[aoc(day1, part2)]
pub fn solvor_part2(inputs: &[i32]) -> i64 {

    let num_set: HashSet<i32> = inputs.iter().cloned().collect();

    let (last, prod) = inputs
        .iter()
        .flat_map(|&x| inputs.iter().map(move |&y| -> (i32, i64) { (2020 - x - y, (x * y).into()) }) )
        .find(|&(left, _)| num_set.contains(&left))
        .unwrap();

    prod * (last as i64)
}
