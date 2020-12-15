// use itertools::Itertools;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<(char, i32)> {

    input
        .lines()
        .map(|line| {
            let (cmd, num_str) = line.split_at(1);
            (cmd.chars().next().unwrap(), num_str.parse::<i32>().unwrap())
        })
        .collect()
}

const DIRS: [char; 4] = ['E', 'S', 'W', 'N'];

fn move_dir(dirch: &char, pos: &(i32, i32), d: i32) -> (i32, i32) {

    match dirch {
        'E' => (pos.0, pos.1 + d),
        'S' => (pos.0 - d, pos.1),
        'W' => (pos.0, pos.1 - d),
        'N' => (pos.0 + d, pos.1),
         _  =>  pos.clone(),
    }
}

#[aoc(day12, part1)]
fn solvor_part1(inputs: &[(char, i32)]) -> i32 {

    let status = inputs
        .iter()
        .fold((0, (0i32, 0i32)), |acc, (step, d)| {
            let (dir, pos) = acc;
            match *step {
                'E' | 'S' | 'W' | 'N' => (dir, move_dir(step, &pos, *d)),
                'L' => ((dir - d / 90 + 4) % 4, pos),
                'R' => ((dir + d / 90 + 4) % 4, pos),
                'F' => (dir, move_dir(&DIRS[dir as usize], &pos, *d)),
                 _  => (dir, pos),
            }
        });

    status.1.0.abs() + status.1.1.abs()

}

struct ShipStat {
    wp: (i32, i32),
    pos: (i32, i32),
}

impl ShipStat {
    pub fn new(wp: (i32, i32), pos: (i32, i32)) -> Self {
        Self {
            wp,
            pos,
        }
    }
}

fn rotate_dir(dir: &(i32, i32), angle: f32) -> (i32, i32) {
    let (sin, cos) = ((angle / 180f32 * std::f32::consts::PI).sin() as i32,
                      (angle / 180f32 * std::f32::consts::PI).cos() as i32);
    // println!("{}, {}", sin, cos);
    ( dir.0 * cos - dir.1 * sin, dir.0 * sin + dir.1 * cos )
}

#[aoc(day12, part2)]
fn solvor_part2(inputs: &[(char, i32)]) -> i32 {

    let init_stat = ShipStat {
        wp:  (10, 1),
        pos: (0, 0),
    };

    let status = inputs
        .iter()
        .fold(init_stat, |acc, (step, d)| {
            // println!("{}, {}/{}, {}", acc.wp.0, acc.wp.1, acc.pos.0, acc.pos.1);
            match *step {
                'E' => ShipStat::new ( (acc.wp.0 + d, acc.wp.1), acc.pos ),
                'S' => ShipStat::new ( (acc.wp.0, acc.wp.1 - d), acc.pos ),
                'W' => ShipStat::new ( (acc.wp.0 - d, acc.wp.1), acc.pos ),
                'N' => ShipStat::new ( (acc.wp.0, acc.wp.1 + d), acc.pos ),
                'L' => ShipStat::new ( rotate_dir(&acc.wp, d.clone() as f32), acc.pos ),
                'R' => ShipStat::new ( rotate_dir(&acc.wp, -d.clone() as f32), acc.pos ),
                'F' => ShipStat::new ( acc.wp, (acc.pos.0 + d * acc.wp.0, acc.pos.1 + d * acc.wp.1) ),
                 _  => acc,
            }
        });

    status.pos.0.abs() + status.pos.1.abs()
}
