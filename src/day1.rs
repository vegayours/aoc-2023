use aoc_runner_derive::aoc;

fn solve(input: &str, parse_digits_fn: fn(&str) -> Vec<u32>) -> u32 {
    input
        .lines()
        .map(|l| {
            let digits = parse_digits_fn(l);
            10 * digits.first().unwrap() + digits.last().unwrap()
        })
        .sum()
}

#[aoc(day01, part1)]
pub fn part1(input: &str) -> u32 {
    solve(input, |l| l.chars().flat_map(|c| c.to_digit(10)).collect())
}

fn parse_part2(l: &str) -> Vec<u32> {
    let prefixes = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut digits = Vec::new();
    let mut buf = l;
    while !buf.is_empty() {
        if let Some(x) = buf.chars().take(1).flat_map(|c| c.to_digit(10)).next() {
            digits.push(x);
        } else if let Some((i, _)) = prefixes
            .iter()
            .enumerate()
            .filter(|&(_, p)| buf.starts_with(p))
            .take(1)
            .next()
        {
            digits.push(i as u32 + 1);
        }
        buf = &buf[1..];
    }
    digits
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    solve(input, parse_part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let example = include_str!("examples/day01_1.txt");
        assert_eq!(part1(example), 142);
    }

    #[test]
    fn test_example_part2() {
        let example = include_str!("examples/day01_2.txt");
        assert_eq!(part2(example), 281);
    }
}
