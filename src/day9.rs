use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Input = Vec<Vec<i64>>;

#[aoc_generator(day9)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split(' ').flat_map(|x| x.parse()).collect())
        .collect()
}

fn next_diff(input: &[i64]) -> Vec<i64> {
    input
        .iter()
        .copied()
        .tuple_windows()
        .map(|(prev, next)| next - prev)
        .collect()
}

fn sequence_next(input: &Vec<i64>) -> i64 {
    let mut diffs = Vec::new();

    let mut seq = input.to_owned();

    while seq.len() > 1 && !seq.iter().all(|x| *x == 0) {
        diffs.push(seq.last().copied().unwrap());
        seq = next_diff(&seq)
    }
    diffs.iter().sum()
}

fn sequence_prev(input: &Vec<i64>) -> i64 {
    let mut firsts = Vec::new();

    let mut seq = input.to_owned();

    while seq.len() > 1 && !seq.iter().all(|x| *x == 0) {
        firsts.push(seq[0]);
        seq = next_diff(&seq)
    }
    let mut res = 0;
    for x in firsts.into_iter().rev() {
        res = x - res;
    }
    res
}

#[aoc(day9, part1)]
fn part1(input: &Input) -> i64 {
    input.iter().map(sequence_next).sum()
}

#[aoc(day9, part2)]
fn part2(input: &Input) -> i64 {
    input.iter().map(sequence_prev).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("examples/day09.txt");
        assert_eq!(part1(&parse(example)), 114);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("examples/day09.txt");
        assert_eq!(part2(&parse(example)), 2);
    }
}
