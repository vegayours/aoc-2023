use std::collections::HashMap;

use gcd::Gcd;

use aoc_runner_derive::{aoc, aoc_generator};

struct Node {
    left: String,
    right: String,
}

type Graph = HashMap<String, Node>;

struct Input {
    dir: String,
    graph: Graph,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let mut lines = input.lines();

    let direction = lines.next().unwrap().to_string();

    lines.next();

    let graph = lines
        .map(|l| {
            let mut parts = l.split(" = ");
            let src = parts.next().unwrap().to_string();
            let mut children = parts
                .next()
                .unwrap()
                .strip_prefix('(')
                .unwrap()
                .strip_suffix(')')
                .unwrap()
                .split(", ");
            (
                src,
                Node {
                    left: children.next().unwrap().to_string(),
                    right: children.next().unwrap().to_string(),
                },
            )
        })
        .collect();

    Input {
        dir: direction,
        graph,
    }
}

#[aoc(day8, part1)]
fn part1(input: &Input) -> i64 {
    let mut seq = input.dir.bytes().cycle();
    let mut cnt = 0;

    let mut key: &str = "AAA";

    while key != "ZZZ" {
        cnt += 1;
        let node = input.graph.get(key).unwrap();
        match seq.next() {
            Some(b'R') => {
                key = &node.right;
            }
            Some(b'L') => {
                key = &node.left;
            }
            x => panic!("Wrong dir: {:?}", x),
        }
    }

    cnt
}

fn find_cycle(input: &Input, start: &String) -> (Vec<usize>, usize) {
    let mut seq = input.dir.bytes().enumerate().cycle();
    let mut cnt = 0;

    let mut key = start;
    let mut visited: HashMap<(&String, usize), usize> = HashMap::new();

    let (cycle_start, cycle_len) = loop {
        let (i, d) = seq.next().unwrap();
        match visited.get(&(key, i)) {
            Some(val) => break (*val, cnt - *val),
            _ => {}
        };
        visited.insert((key, i), cnt);

        cnt += 1;
        let node = input.graph.get(key).unwrap();
        match d {
            b'R' => {
                key = &node.right;
            }
            b'L' => key = &node.left,
            x => panic!("Wrong dir: {:?}", x),
        }
    };
    let starts: Vec<usize> = visited
        .into_iter()
        .filter(|((key, _), pos)| key.ends_with('Z') && *pos >= cycle_start)
        .map(|(_, pos)| pos)
        .collect();
    (starts, cycle_len)
}

fn get_lcm(a: usize, b: usize) -> usize {
    let gcd = a.gcd(b);
    a * (b / gcd)
}

// After debugging all starts reach only 1 vertex with 'Z' and it starts at cycle len.
// This is easily solved as least common multiple of all cycle lengths.
#[aoc(day8, part2)]
fn part2(input: &Input) -> i64 {
    let keys = input.graph.keys().filter(|k| k.ends_with('A'));
    let mut lcm = 1;
    for key in keys {
        let (_, cycle_len) = find_cycle(input, key);
        lcm = get_lcm(lcm, cycle_len);
    }
    lcm as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("examples/day08.txt");
        assert_eq!(part1(&parse(example)), 2);
    }

    #[test]
    fn part1_example_2() {
        let example = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&parse(example)), 6);
    }

    #[test]
    fn part2_example() {
        let example = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(part2(&parse(example)), 6);
    }
}
