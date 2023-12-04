use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Vec<u8>>;

#[aoc_generator(day3)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .into_iter()
        .map(|l| Vec::from(l.as_bytes()))
        .collect()
}

struct Number {
    l: usize,
    r: usize,
    i: usize,
    value: i64,
}

fn parse_numbers(input: &Input) -> Vec<Number> {
    let mut result = Vec::new();
    let m = input.len();
    let n = input[0].len();

    for i in 0..m {
        for j in 0..n {
            if input[i][j].is_ascii_digit() && (j == 0 || !input[i][j - 1].is_ascii_digit()) {
                let mut number = 0;

                let mut k = j;
                while k < n && input[i][k].is_ascii_digit() {
                    number = number * 10 + (input[i][k] - b'0') as i64;
                    k = k + 1;
                }

                let l = if j > 0 { j - 1 } else { j };
                let r = if k < n { k } else { k - 1 };

                result.push(Number {
                    l,
                    r,
                    i,
                    value: number,
                });
            }
        }
    }

    result
}

#[aoc(day3, part1)]
fn part1(input: &Input) -> i64 {
    let m = input.len();

    parse_numbers(input)
        .into_iter()
        .filter(|&Number { l, r, i, .. }| {
            let mut has_symbol = (input[i][l] != b'.' && !input[i][l].is_ascii_digit())
                || (input[i][r] != b'.' && !input[i][r].is_ascii_digit());

            for p in l..r + 1 {
                if i > 0 {
                    has_symbol = has_symbol || input[i - 1][p] != b'.';
                }
                if i + 1 < m {
                    has_symbol = has_symbol || input[i + 1][p] != b'.';
                }
                if has_symbol {
                    break;
                }
            }
            has_symbol
        })
        .map(|n| n.value)
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Input) -> i64 {
    let mut map: HashMap<(usize, usize), Vec<i64>> = HashMap::new();
    let mut add_value = |i, j, value| {
        map.entry((i, j))
            .and_modify(|v| v.push(value))
            .or_insert(vec![value]);
    };
    for Number { l, r, i, value } in parse_numbers(input) {
        if input[i][l] == b'*' {
            add_value(i, l, value);
        }
        if input[i][r] == b'*' {
            add_value(i, r, value);
        }
        for p in l..r + 1 {
            if i > 0 && input[i - 1][p] == b'*' {
                add_value(i - 1, p, value);
            }
            if i + 1 < input.len() && input[i + 1][p] == b'*' {
                add_value(i + 1, p, value);
            }
        }
    }
    map.values()
        .into_iter()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("examples/day03.txt");
        assert_eq!(part1(&parse(example)), 4361);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("examples/day03.txt");
        assert_eq!(part2(&parse(example)), 467835);
    }
}
