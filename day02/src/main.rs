use std::fs;

use advent_2023_common::get_args;
use day02::models::Game;
use day02::parser;

fn solve_part_one(data: &[Game]) -> u32 {
    data.iter()
        .filter(|game| {
            game.subsets
                .iter()
                .all(|subset| subset.red <= 12 && subset.green <= 13 && subset.blue <= 14)
        })
        .map(|game| game.index)
        .sum()
}

fn solve_part_two(data: &[Game]) -> u32 {
    data.iter()
        .map(|game| {
            game.subsets.iter().map(|subset| subset.red).max().unwrap()
                * game
                    .subsets
                    .iter()
                    .map(|subset| subset.green)
                    .max()
                    .unwrap()
                * game.subsets.iter().map(|subset| subset.blue).max().unwrap()
        })
        .sum()
}

fn main() {
    let args = get_args();
    let data: String = fs::read_to_string(&args.path).unwrap();
    let parsed_data = parser::parse_data(&data);

    if args.verbose {
        println!("{:#?}", &parsed_data);
    }

    println!("Part one solution: {}", solve_part_one(&parsed_data));
    println!("Part two solution: {}", solve_part_two(&parsed_data));
}

#[cfg(test)]
mod tests {
    use day02::models::GameSubset;
    use day02::parser::parse_data;

    use super::*;

    static INPUT_EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_parse_data() {
        assert_eq!(
            parse_data("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            vec![Game {
                index: 1,
                subsets: vec![
                    GameSubset {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    GameSubset {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    GameSubset {
                        red: 0,
                        green: 2,
                        blue: 0,
                    }
                ],
            }]
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(&parse_data(INPUT_EXAMPLE)), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE)), 2286);
    }
}
