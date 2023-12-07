use std::fs;

use advent_2023_common::get_args;
use day07::models::Hand;
use day07::parser::parse_data;

fn solve_part_one(data: &[Hand]) -> usize {
    // Copy the data in order to sort it
    let mut hands: Vec<Hand> = data.to_vec();

    // Sort the hands
    hands.sort_by_cached_key(|hand| (hand.get_type(), hand.cards.clone()));

    // Compute the result
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum()
}

fn solve_part_two(data: &[Hand]) -> usize {
    // Copy the data in order to sort it
    let mut hands: Vec<Hand> = data.to_vec();

    // Change J score in order to make it the worst card
    for hand in &mut hands {
        for card in &mut hand.cards {
            if *card == 11 {
                *card = 0
            }
        }
    }

    // Sort the hands
    hands.sort_by_cached_key(|hand| (hand.get_best_type(), hand.cards.clone()));

    // Compute the result
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum()
}

fn main() {
    let args = get_args();
    let data: String = fs::read_to_string(&args.path).unwrap();

    let parsed_data = parse_data(&data);

    if args.verbose {
        println!("{:#?}", &parsed_data);
    }

    println!("Part one solution: {}", solve_part_one(&parsed_data));
    println!("Part two solution: {}", solve_part_two(&parsed_data));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT_EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(&parse_data(INPUT_EXAMPLE)), 6440);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE)), 5905);
    }
}
