use std::collections::HashMap;
use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<String> {

    input
        .split("\n\n")
        .map(|group| group.to_string() )
        .collect()
}

#[aoc(day6, part1)]
fn solvor_part1(inputs: &[String]) -> usize {

    inputs
        .iter()
        .map(|input| {

            let mut hs: HashSet<_> = HashSet::new();
            
            for c in input.replace('\n', "").chars() {
                hs.insert(c);
            }

            hs.len()

        })
        .sum()
}

#[aoc(day6, part2)]
fn solvor_part2(inputs: &[String]) -> usize {

    inputs
        .iter()
        .map(|input| {
            input
                .chars()
                .fold(HashMap::new(), |mut hm, c| {
                    *hm.entry(c).or_insert(0usize) += 1;
                    hm
                })
                .iter()
                .filter(|&(_, v)| *v == input.lines().count())
                .count()
        })
        .sum()
    
}
