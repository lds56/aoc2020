use aoc_runner_derive::{aoc, aoc_generator};

use super::util;
use std::error;

use num::Integer;

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Result<(i32, Vec<i32>), Box<dyn error::Error>> {

    let mut strs = input.split('\n');

    Ok((
        strs.next().ok_or(util::AocError)?.parse::<i32>()?,
        strs.next().ok_or(util::AocError)?.split(',').map(|str| str.parse::<i32>().unwrap_or(-1) ).collect()
    ))

}

#[aoc(day13, part1)]
fn solvor_part1(res: &(i32, Vec<i32>)) -> i32 {

    let (timestamp, nums) = res;

    let ans = nums
        .iter()
        .filter(|x| **x != -1)
        .map(|&x| (x.clone(), (x - timestamp % x) % x) )
        .min_by(|a, b| a.1.cmp(&b.1) )
        .unwrap();

    ans.0 * ans.1
}

#[aoc(day13, part2)]
fn solvor_part2(res: &(i32, Vec<i32>)) -> i64 {

    let (_, nums) = res;

    nums
        .iter()
        .zip(0..nums.len())
        .filter(|(x, _)| **x != -1)
        .map(|(x, i)| (*x as i64, (((x - i as i32) % x + x) % x) as i64))
        .fold((0i64, 1i64), |acc, (x, rmd)| {
            let mut sum = acc.0;
            while sum % x != rmd {
                sum += acc.1;
            }
            (sum, acc.1.lcm(&x))
        })
        .0

}
