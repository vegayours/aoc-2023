use aoc_runner_derive::{aoc, aoc_generator};

struct Game {
    time: i64,
    record: i64,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Game> {
    let mut lines = input.lines();

    let mut parse_line = || {
        lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|x| x.parse::<i64>().unwrap())
    };

    parse_line()
        .zip(parse_line())
        .map(|(time, record)| Game { time, record })
        .collect()
}

fn win_count(game: &Game) -> i64 {
    (1..game.time)
        .filter(|charge| (game.time - charge) * charge > game.record)
        .count() as i64
}

#[aoc(day6, part1)]
fn part1(input: &[Game]) -> i64 {
    input.iter().map(win_count).product()
}

#[aoc(day6, part2)]
fn part2(input: &[Game]) -> i64 {
    let time: i64 = input
        .iter()
        .map(|g| g.time.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    let record: i64 = input
        .iter()
        .map(|g| g.record.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();
    win_count(&Game { time, record })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("examples/day06.txt");
        assert_eq!(part1(&parse(example)), 288);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("examples/day06.txt");
        assert_eq!(part2(&parse(example)), 71503);
    }
}
