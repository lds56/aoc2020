use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<char>> {

    input
        .lines()
        .map(|line| line.chars().map(|res| res ).collect() )
        .collect()
}

fn count_tree(graph: &[Vec<char>], step: &(usize, usize)) -> usize {

    let mut pos: (usize, usize) = (0, 0);
    let row = graph.len();
    let col = graph[0].len();
    let mut cnt: usize = 0;

    while pos.0 != row - 1 {

        pos = (pos.0 + step.0, pos.1 + step.1);
        if graph[pos.0][pos.1 % col] == '#' {
            cnt += 1;
        }
    }

    cnt

}

#[aoc(day3, part1)]
fn solvor_part1(graph: &[Vec<char>]) -> usize {

    count_tree(graph, &(1usize, 3usize))
}

#[aoc(day3, part2)]
fn solvor_part2(graph: &[Vec<char>]) -> i64 {

    let steps: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    steps
        .iter()
        .map(|step| count_tree(graph, &step) as i64 )
        .product()
}
