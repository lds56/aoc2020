use std::cmp;
use std::collections::HashSet;
use itertools::Itertools;
use std::iter::FromIterator;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<i64> {

    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() )
        .collect()
}

#[aoc(day9, part1)]
fn solvor_part1(inputs: &[i64]) -> i64 {

    for x in 25..inputs.len() {
        
        let sum_set: HashSet<i64> =
        HashSet::from_iter(inputs[(x-25)..x]
                           .iter()
                           .combinations(2)
                           .map(|v| v[0] + v[1]));

        if !sum_set.contains(&inputs[x]) {
            return inputs[x];
        }
    }

    0
}

#[aoc(day9, part2)]
fn solvor_part2(inputs: &[i64]) -> i64 {

    let mut minx: i64 = i64::MAX;
    let mut maxx: i64 = i64::MIN;

    let mut tail = 0usize;
    let mut head = 0usize;

    let mut sum: i64 = 0;
    let target: i64 = 3199139634;

    while head <= inputs.len() {

        if sum < target {
            sum += inputs[head];
            head += 1;
        }
        else if sum > target {
            sum -= inputs[tail];
            tail += 1;
        }
        else {
            for i in tail..head {
                minx = cmp::min(minx, inputs[i]);
                maxx = cmp::max(maxx, inputs[i]);
            }
            break;
        }
    }

    minx + maxx

}
