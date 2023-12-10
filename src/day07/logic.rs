use crate::day07::models::{Hand, HandType};
use std::collections::HashMap;

fn get_type_from_cards(cards: &[u8]) -> HandType {
    let mut cards_set: HashMap<u8, u8> = HashMap::new();
    for card in cards {
        *cards_set.entry(*card).or_default() += 1;
    }

    match cards_set.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if cards_set.values().any(|count| *count == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        3 => {
            if cards_set.values().any(|count| *count == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => unreachable!(),
    }
}

pub fn get_type(hand: &Hand) -> HandType {
    get_type_from_cards(&hand.cards)
}

pub fn get_best_type(hand: &Hand) -> HandType {
    (2..=14)
        .map(|joker_substitution| {
            get_type_from_cards(
                &hand
                    .cards
                    .iter()
                    .cloned()
                    .map(|card| if card == 0 { joker_substitution } else { card })
                    .collect::<Vec<_>>(),
            )
        })
        .max()
        .unwrap()
}

pub fn solve_part_one(data: &[Hand]) -> usize {
    // Copy the data in order to sort it
    let mut hands: Vec<Hand> = data.to_vec();

    // Sort the hands
    hands.sort_by_cached_key(|hand| (get_type(hand), hand.cards.clone()));

    // Compute the result
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum()
}

pub fn solve_part_two(data: &[Hand]) -> usize {
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
    hands.sort_by_cached_key(|hand| (get_best_type(hand), hand.cards.clone()));

    // Compute the result
    hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx + 1) * hand.bid)
        .sum()
}
