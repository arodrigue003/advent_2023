use crate::day07::models::Hand;

static ZERO_VALUE: u8 = b'0';

pub fn parse_hand(line: &str) -> Hand {
    let (left, right) = line.split_once(' ').unwrap();
    let cards = left
        .chars()
        .map(|char| match char {
            '2'..='9' => char as u8 - ZERO_VALUE,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => unreachable!(),
        })
        .collect();

    Hand {
        cards,
        bid: right.parse().unwrap(),
    }
}

pub fn parse_input(input: String) -> Vec<Hand> {
    input.lines().map(parse_hand).collect()
}
