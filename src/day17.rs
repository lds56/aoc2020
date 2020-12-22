use std::ops;
use std::collections::HashSet;
use std::hash::Hash;

use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos3 {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Pos4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl ops::Add<Pos3> for Pos3 {
    type Output = Pos3;
    fn add(self, rhs: Pos3) -> Pos3 {
        Pos3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Pos3 {
    fn get_neighbors() -> Vec<Pos3> {

        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .filter(|((x, y), z)| !(x == &0 && y == &0 && z == &0))
            .map(|((x, y), z)| Pos3 {x: x, y: y, z: z})
            .collect()
    }
}

impl ops::Add<Pos4> for Pos4 {
    type Output = Pos4;
    fn add(self, rhs: Pos4) -> Pos4 {
        Pos4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Pos4 {
    fn get_neighbors() -> Vec<Pos4> {

        (-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .cartesian_product(-1..=1)
            .map(|(((x, y), z), w)| Pos4 {x: x, y: y, z: z, w: w} )
            .filter(|pos| !(pos.x == 0 && pos.y == 0 && pos.z == 0 && pos.w == 0))
            .collect()
    }
}


#[aoc_generator(day17)]
fn input_generator(input: &str) -> HashSet<(isize, isize, isize)> {

    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.
                chars()
                .enumerate()
                .filter(|(_, ch)| ch == &'#')
                .map(move |(j, _)| (i as isize, j as isize, 0))
        })
        .collect()
}

fn get_active_cnt<T>(neighbors: &[T], pos: &T, status: &HashSet<T>) -> usize
where T: ops::Add<Output = T> + Copy + Eq + Hash {

    neighbors
        .iter()
        .filter(|&step| status.contains(&(*pos + *step)) )
        .count()
}

fn solvor<T>(neighbors: &[T], status0: &HashSet<T>) -> usize
where T: ops::Add<Output = T> + Copy + Eq + Hash {

    let mut status = status0.clone();

    for _ in 0..6 {

        status = status
            .iter()
            .map(|pos| {

                let cnt = get_active_cnt::<T>(neighbors, &pos, &status);

                neighbors
                    .iter()
                    .map(|nb| *pos + *nb )
                    .fold(if cnt == 2 || cnt == 3 { vec![*pos] } else { vec![] }, |mut acc, p| {
                        if get_active_cnt::<T>(neighbors, &p, &status) == 3 {
                            acc.push(p);
                        }
                        acc
                    })

            })
            .flatten()
            .collect();
    }

    status.len()
}

#[aoc(day17, part1)]
fn solvor_part1(input: &HashSet<(isize, isize, isize)>) -> usize {

    let neighbors = Pos3::get_neighbors();

    let status0 = input.iter().map(|p| Pos3{x: p.0, y: p.1, z: p.2}).collect();

    solvor::<Pos3>(&neighbors, &status0)
}


#[aoc(day17, part2)]
fn solvor_part2(input: &HashSet<(isize, isize, isize)>) -> usize {

    let neighbors = Pos4::get_neighbors();

    let status0 = input.iter().map(|p| Pos4{x: p.0, y: p.1, z: p.2, w: 0}).collect();

    solvor::<Pos4>(&neighbors, &status0)
}
