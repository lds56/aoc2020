use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<(i32, i32)> {

    input
        .lines()
        .map(|line| {

            let (first, last) = line.split_at(7);

            let first_bin = first.replace('B', "1").replace('F', "0");
            let last_bin = last.replace('R', "1").replace('L', "0");

            (i32::from_str_radix(&first_bin, 2).unwrap(),
             i32::from_str_radix(&last_bin, 2).unwrap())
        })
        .collect()
}

#[aoc(day5, part1)]
fn solvor_part1(inputs: &[(i32, i32)]) -> i32 {

    inputs
        .iter()
        .map(|(row, col)| row * 8  + col)
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn solvor_part2(inputs: &[(i32, i32)]) -> i32 {

    let mut numbers = inputs
        .iter()
        .map(|(row, col)| row * 8 + col)
        .collect::<Vec<i32>>();

    numbers.sort();

    if let Some((first, left)) = numbers.split_first() {

        let mut last = first.clone();
        
        for number in left.iter() {
            if last + 1 != *number {
                return last + 1;
            }
            last = last + 1;
        }
    }

    0
}
