use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn map_char_p1(c: char) -> char {
    match c {
        'T' => 'A',
        'J' => 'B',
        'Q' => 'C',
        'K' => 'D',
        'A' => 'E',
        '2'..='9' => c,
        _ => panic!("Wrong char {c}"),
    }
}

fn map_char_p2(c: char) -> char {
    match c {
        'T' => 'A',
        'J' => '0',
        'Q' => 'C',
        'K' => 'D',
        'A' => 'E',
        '2'..='9' => c,
        _ => panic!("Wrong char {c}"),
    }
}

struct Hand {
    cards: String,
    bid: i64,
}

fn parse(input: &str, map_char: fn(char) -> char) -> Vec<Hand> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(' ').collect();
            Hand {
                cards: parts[0].chars().map(map_char).collect(),
                bid: parts[1].parse().unwrap(),
            }
        })
        .collect()
}

fn score_hand_p1(hand: &Hand) -> i64 {
    let mut count: HashMap<char, u32> = HashMap::new();
    for c in hand.cards.chars() {
        *count.entry(c).or_default() += 1;
    }
    count
        .values()
        .map(|v| 4_i64.pow(*v))
        .sum()
}

fn score_hand_p2(hand: &Hand) -> i64 {
    let mut count: HashMap<char, u32> = HashMap::new();
    for c in hand.cards.chars() {
        *count.entry(c).or_default() += 1;
    }
    if let Some(add) = count.remove(&'0') {
        if let Some((k, _)) = count.iter().max_by_key(|(_, v)| *v) {
            count.entry(*k).and_modify(|v| *v += add);
        } else {
            count.insert('0', add);
        }
    }
    count
        .values()
        .map(|v| 4_i64.pow(*v))
        .sum()
}

fn solve(input: &str, map_char: fn(char) -> char, score_hand: fn(hand: &Hand) -> i64) -> i64 {
    let mut input = parse(input, map_char);
    input.sort_by(|a, b| (score_hand(a), &a.cards).cmp(&(score_hand(b), &b.cards)));
    input
        .iter()
        .enumerate()
        .map(|(index, hand)| ((index as i64) + 1) * hand.bid)
        .sum()
}

#[aoc(day7, part1)]
fn part1(input: &str) -> i64 {
    solve(input, map_char_p1, score_hand_p1)
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i64 {
    solve(input, map_char_p2, score_hand_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let example = include_str!("examples/day07.txt");
        assert_eq!(part1(example), 6440);
    }

    #[test]
    fn part2_example() {
        let example = include_str!("examples/day07.txt");
        assert_eq!(part2(example), 5905);
    }
}
