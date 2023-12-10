use crate::common::{test_part_one_common, test_part_two_common};
use crate::day02::models::{Game, GameSubset};
use crate::day02::parser::parse_input;
use crate::day02::Day02;

static INPUT_EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

impl GameSubset {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
}

#[test]
fn test_parse_data() {
    assert_eq!(
        parse_input("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
        vec![Game {
            index: 1,
            subsets: vec![
                GameSubset::new(4, 0, 3),
                GameSubset::new(1, 2, 6),
                GameSubset::new(0, 2, 0)
            ],
        }]
    );
}

#[test]
fn test_part_one() {
    test_part_one_common(Day02::new(), INPUT_EXAMPLE, 8);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day02::new(), INPUT_EXAMPLE, 2286);
}
