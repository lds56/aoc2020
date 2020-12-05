use regex::Regex;

use aoc_runner_derive::{aoc, aoc_generator};

struct PwdPolicy {
    c: char,
    cnt: (usize, usize),
    pwd: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<PwdPolicy> {

    let re = Regex::new(r"(\d+)-(\d+) ([a-zA-Z]): (\w+)").unwrap();

    input
        .lines()
        .map(|input| {

            let caps = re.captures(input).unwrap();

            PwdPolicy {
                c: caps.get(3).unwrap().as_str().chars().next().unwrap(),
                cnt: (caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                      caps.get(2).unwrap().as_str().parse::<usize>().unwrap()),
                pwd: caps.get(4).unwrap().as_str().to_string(),
            }
        })
        .collect::<Vec<PwdPolicy>>()
}

#[aoc(day2, part1)]
fn solvor_part1(inputs: &[PwdPolicy]) -> usize {

    // for input in inputs.iter() {
       // println!("{}-{}, {}, {}", input.cnt.0, input.cnt.1, input.c, input.pwd);
    // }

    inputs
        .iter()
        .filter(|policy| {
            let cnt = policy.pwd.matches(policy.c).count();
            cnt >= policy.cnt.0 && cnt <= policy.cnt.1
        })
        .count()
}

#[aoc(day2, part2)]
fn solvor_part2(inputs: &[PwdPolicy]) -> usize {

    inputs
        .iter()
        .filter(|policy| {
            if let Some(x) = policy.pwd.chars().nth(policy.cnt.0 - 1) {
                if let Some(y) = policy.pwd.chars().nth(policy.cnt.1 - 1) {
                    return x == policy.c && y != policy.c ||
                           x != policy.c && y == policy.c
                }
            }
            false
        })
        .count()
}
