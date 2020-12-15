use std::cmp;
use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day11)]
fn input_generator(input: &str) -> (usize, Vec<char>) {

    (
        input.lines().count(),
        input
            .replace('\n', "")
            .chars()
            .collect()
    )

/*
    input
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect()
*/
}

const STEPS: [(i32, i32); 8] = [
    (1,1),  (1,0),  (1,-1), (0,1),
    (0,-1), (-1,1), (-1,0), (-1,-1)
];

fn count_neighbor(pos: &(i32, i32), maxk: i32, shape: &(i32, i32), data: &[char]) -> usize {

    STEPS
        .iter()
        .filter(|(di, dj)| {
            
            let seat = (1..=maxk)
                .map(|k| (pos.0 + di * k, pos.1 + dj * k))
                .take_while(|(x, y)| x >= &0 && x < &shape.0 && y >= &0 && y < &shape.1 )
                .map(|(x, y)| data.get( (x * shape.1 + y) as usize ).unwrap() )
                .skip_while(|&&seat| seat == '.')
                .next();

            seat.is_some() && seat.unwrap() == &'#'
                
        })
        .count()
}

#[aoc(day11, part1)]
fn solvor_part1(inputs: &(usize, Vec<char>)) -> usize {

    let mut status: Vec<char> = inputs.1.iter().map(|x| x.clone() ).collect();
    let row = inputs.0 as i32;
    let col = status.len() as i32 / row;

    loop {

        let mut changed_num = 0i32;

        status = (0..row)
            .cartesian_product(0..col)
            .map(|(i, j)| {
                
                let neighbors_cnt = count_neighbor(&(i, j), 1, &(row, col), &status);

                match status[(i*col + j) as usize] {
                    '#' => if neighbors_cnt >= 4 {changed_num += 1; 'L'} else {'#'},
                    'L' => if neighbors_cnt == 0 {changed_num += 1; '#'} else {'L'},
                    '.' => '.',
                     _  => '.',
                }


            })
            .collect();

        if changed_num == 0 { break; }

    }

    status
        .iter()
        .filter(|x| **x == '#')
        .count()
    
}

#[aoc(day11, part2)]
fn solvor_part2(inputs: &(usize, Vec<char>)) -> usize {

    let mut status: Vec<char> = inputs.1.iter().map(|x| x.clone() ).collect();
    let row = inputs.0 as i32;
    let col = status.len() as i32 / row;

    loop {

        let mut changed_num = 0i32;

        status = (0..row)
            .cartesian_product(0..col)
            .map(|(i, j)| {

                let neighbors_cnt = count_neighbor(&(i, j), cmp::max(row, col), &(row, col), &status);

                match status[(i*col + j) as usize] {
                    '#' => if neighbors_cnt >= 5 {changed_num += 1; 'L'} else {'#'},
                    'L' => if neighbors_cnt == 0 {changed_num += 1; '#'} else {'L'},
                    '.' => '.',
                     _  => '.',
                }


            })
            .collect();

        // println!("{}", status.iter().collect::<String>());

        if changed_num == 0 { break; }

    }

    status
        .iter()
        .filter(|x| **x == '#')
        .count()

}
