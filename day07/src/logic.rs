use std::collections::HashMap;

use crate::models::{Hand, HandType};

fn get_type(cards: &[u8]) -> HandType {
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

impl Hand {
    pub fn get_type(&self) -> HandType {
        get_type(&self.cards)
    }

    pub fn get_best_type(&self) -> HandType {
        (2..=14)
            .map(|joker_substitution| {
                get_type(
                    &self
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
}
