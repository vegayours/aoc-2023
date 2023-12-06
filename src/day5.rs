use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

#[derive(Debug)]
struct Range {
    src: i64,
    dst: i64,
    len: i64,
}

type Mapping = Vec<Range>;

#[derive(Debug)]
struct Input {
    seeds: Vec<i64>,
    mappings: Vec<Mapping>,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();

    lines.next();

    let mut mappings = Vec::new();

    loop {
        if lines.next().is_some() {
            let mut mapping = Mapping::new();
            loop {
                match lines.next() {
                    None | Some("") => {
                        break;
                    }
                    Some(range) => {
                        let parts: Vec<i64> =
                            range.split(' ').map(|x| x.parse().unwrap()).collect();
                        mapping.push(Range {
                            src: parts[1],
                            dst: parts[0],
                            len: parts[2],
                        })
                    }
                }
            }
            mappings.push(mapping);
        } else {
            break;
        }
    }

    Input { seeds, mappings }
}

fn map_range(range: &Range, src: i64) -> Option<i64> {
    if (range.src..range.src + range.len).contains(&src) {
        Some(range.dst + src - range.src)
    } else {
        None
    }
}

fn resolve_map(mapping: &Mapping, src: i64) -> i64 {
    for range in mapping {
        if let Some(dst) = map_range(range, src) {
            return dst;
        }
    }
    src
}

fn map_seed(input: &Input, seed: i64) -> i64 {
    let mut dst = seed;
    for map in &input.mappings {
        dst = resolve_map(map, dst);
    }
    dst
}

#[aoc(day5, part1)]
fn part1(input: &Input) -> i64 {
    input
        .seeds
        .iter()
        .map(|&s| map_seed(input, s))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> i64 {
    input
        .seeds
        .iter()
        .copied()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap();
            let len = chunk.next().unwrap();
            (start..start + len)
                .map(|s| map_seed(input, s))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("examples/day05.txt");
        assert_eq!(part1(&parse(example)), 35);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("examples/day05.txt");
        assert_eq!(part2(&parse(example)), 46);
    }
}
