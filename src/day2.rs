use aoc_runner_derive::{aoc, aoc_generator};

use std::cmp;

struct Round {
    r: i64,
    g: i64,
    b: i64,
}

struct Game {
    id: i64,
    rounds: Vec<Round>,
}

fn parse_round(l: &str) -> Round {
    let parts: Vec<&str> = l.split(", ").collect();
    let mut round = Round { r: 0, g: 0, b: 0 };
    for p in parts {
        let items: Vec<&str> = p.split(" ").collect();
        let val: i64 = items[0].parse().unwrap();
        match items[1] {
            "red" => round.r += val,
            "green" => round.g += val,
            "blue" => round.b += val,
            x => panic!("Wrong color: {}", x),
        }
    }
    round
}

fn parse_game(index: i64, l: &str) -> Game {
    let rounds = l.split(": ").into_iter().skip(1).next().unwrap();

    Game {
        id: index,
        rounds: rounds.split("; ").into_iter().map(parse_round).collect(),
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| parse_game((i + 1) as i64, l))
        .collect()
}

#[aoc(day2, part1)]
fn part1(games: &[Game]) -> i64 {
    games
        .iter()
        .filter(|g| {
            g.rounds
                .iter()
                .all(|round| round.r <= 12 && round.g <= 13 && round.b <= 14)
        })
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(games: &[Game]) -> i64 {
    games
        .iter()
        .map(|g| {
            let (r, g, b) = g.rounds.iter().fold((0, 0, 0), |(r, g, b), round| {
                (
                    cmp::max(r, round.r),
                    cmp::max(g, round.g),
                    cmp::max(b, round.b),
                )
            });
            r * g * b
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("examples/day02.txt");
        assert_eq!(part1(&parse(example)), 8);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("examples/day02.txt");
        assert_eq!(part2(&parse(example)), 2286);
    }
}
