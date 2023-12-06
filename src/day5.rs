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
            mapping.sort_by(|a, b| a.src.cmp(&b.src));
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

fn process_segment((mut start, end): (i64, i64), mapping: &Mapping) -> Vec<(i64, i64)> {
    let mut res = Vec::new();
    for range in mapping {
        if start < range.src {
            let consume = (end - start).min(range.src - start);
            if consume > 0 {
                res.push((start, start + consume));
                start += consume;
            }
        }
        let skip = (start - range.src).min(range.len);
        let consume = (end - start).min(range.len - skip);
        if consume > 0 {
            res.push((range.dst + skip, range.dst + skip + consume));
            start += consume;
        }
    }
    if start < end {
        res.push((start, end));
    }
    res
}

#[aoc(day5, part2)]
fn part2(input: &Input) -> i64 {
    let mut segments: Vec<(i64, i64)> = input
        .seeds
        .iter()
        .copied()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap();
            let len = chunk.next().unwrap();
            (start, start + len)
        })
        .collect();
    for mapping in &input.mappings {
        segments = segments
            .into_iter()
            .flat_map(|s| process_segment(s, mapping).into_iter())
            .collect();
    }

    segments.into_iter().map(|(s, _)| s).min().unwrap()
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
