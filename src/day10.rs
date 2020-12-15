use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<i32> {

    let mut v = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap() )
        .collect::<Vec<i32>>();

    v.sort();

    v
}

#[aoc(day10, part1)]
fn solvor_part1(inputs: &[i32]) -> i32 {

    let n = inputs.len();
    
    let cnt_map = (0..n-1)
        .zip(1..n)
        .map(|(i, j)| {
            inputs[j] - inputs[i]
        })
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    (cnt_map.get(&1).unwrap() + 1) * (cnt_map.get(&3).unwrap() + 1)
}

#[aoc(day10, part2)]
fn solvor_part2(inputs: &[i32]) -> i64 {

    let mut f = vec![0i64; inputs.len()+1];
    f[0] = 1;

    for i in 1..=inputs.len() {

        let start = if i < 3 {0} else {i-3};
        for j in start..i {

            let x = if j == 0 {0} else {inputs[j-1]};
            if x+3 < inputs[i-1] { continue; }

            f[i] += f[j];
        }
    }

    f[inputs.len()]
}
