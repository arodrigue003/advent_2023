use std::fs;

use advent_2023_common::get_args;
use day04::models::Game;
use day04::parser::parse_data;

fn solve_part_one(data: &[Game]) -> u32 {
    data.iter()
        .map(|game| game.draw.intersection(&game.winning).count())
        .filter(|correct| *correct > 0)
        .map(|correct| 1 << (correct - 1))
        .sum()
}

fn solve_part_two(data: &[Game]) -> u32 {
    let game_count = data.len();
    let mut copies = vec![1; game_count];

    for (i, game) in data.iter().enumerate() {
        let correct = game.draw.intersection(&game.winning).count();

        for j in i + 1..game_count.min(i + 1 + correct) {
            copies[j] += copies[i];
        }
    }

    copies.iter().sum()
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
    use std::collections::HashSet;

    use day04::models::Game;
    use day04::parser::parse_data;

    use super::*;

    static INPUT_EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_parse_data() {
        assert_eq!(
            parse_data("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            vec![Game {
                index: 1,
                winning: HashSet::from([41, 48, 83, 86, 17]),
                draw: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
            }]
        );
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(&parse_data(INPUT_EXAMPLE)), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE)), 30);
    }
}
