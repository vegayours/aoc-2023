use aoc_runner_derive::{aoc, aoc_generator};

use std::collections::{HashMap, HashSet};

struct Game {
    win: HashSet<i64>,
    your: Vec<i64>,
}

fn parse_game(line: &str) -> Game {
    let src = line.split(": ").nth(1).unwrap();
    let parts: Vec<&str> = src.split(" | ").collect();
    Game {
        win: parts[0]
            .split_ascii_whitespace()
            .map(|l| l.parse().unwrap())
            .collect(),
        your: parts[1]
            .split_ascii_whitespace()
            .map(|l| l.parse().unwrap())
            .collect(),
    }
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

fn score_part1(cnt: usize) -> i64 {
    if cnt == 0 {
        0
    } else {
        1 << (cnt - 1)
    }
}

#[aoc(day4, part1)]
fn part1(input: &[Game]) -> i64 {
    let mut sum = 0;
    for game in input {
        let matching = game.your.iter().filter(|x| game.win.contains(x)).count();
        sum += score_part1(matching);
    }
    sum
}

#[aoc(day4, part2)]
fn part2(input: &[Game]) -> i64 {
    let mut m: HashMap<usize, usize> = HashMap::new();
    for (i, game) in input.iter().enumerate() {
        let matching = game.your.iter().filter(|x| game.win.contains(x)).count();
        let card = i + 1;
        let cnt = {
            let _cnt = m.entry(card).or_default();
            *_cnt += 1;
            *_cnt
        };
        for j in 1..=matching {
            *m.entry(card + j).or_default() += cnt;
        }
    }
    m.values().sum::<usize>() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("examples/day04.txt");
        assert_eq!(part1(&parse(example)), 13);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("examples/day04.txt");
        assert_eq!(part2(&parse(example)), 30);
    }
}
