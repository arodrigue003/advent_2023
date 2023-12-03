use std::fs;

use advent_2023_common::get_args;

static ZERO_VALUE: u32 = '0' as u32;

fn solve_part_one(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let (_, first, last) = line
                .chars()
                .filter_map(|char| {
                    if char.is_ascii_digit() {
                        Some(char as u32 - ZERO_VALUE)
                    } else {
                        None
                    }
                })
                .fold((false, 0, 0), |(first_done, first, _), value| {
                    if first_done {
                        (true, first, value)
                    } else {
                        (true, value, value)
                    }
                });
            first * 10 + last
        })
        .sum()
}

fn solve_part_two(data: &str) -> u32 {
    data.lines()
        .map(|line| {
            let (_, first, last) = line
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
                .chars()
                .filter_map(|char| {
                    if char.is_ascii_digit() {
                        Some(char as u32 - ZERO_VALUE)
                    } else {
                        None
                    }
                })
                .fold((false, 0, 0), |(first_done, first, _), value| {
                    if first_done {
                        (true, first, value)
                    } else {
                        (true, value, value)
                    }
                });
            first * 10 + last
        })
        .sum()
}

fn main() {
    let args = get_args();
    let data: String = fs::read_to_string(args.path).unwrap();

    println!("Part one solution: {}", solve_part_one(&data));
    println!("Part two solution: {}", solve_part_two(&data));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT_EXAMPLE_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    static INPUT_EXAMPLE_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(INPUT_EXAMPLE_1), 142);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(INPUT_EXAMPLE_2), 281);
    }
}
