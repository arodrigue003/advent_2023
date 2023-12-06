use std::fs;

use advent_2023_common::get_args;
use day06::models::Races;
use day06::parser::parse_data;

// Check if value has an integer square root and returns it.
fn perfect_square_root(value: i64) -> Option<i64> {
    let res = ((value as f64).sqrt() + 0.5) as i64;
    if res * res == value {
        Some(res)
    } else {
        None
    }
}

// Compute the minimal value for which we would go further than the reference.
// This correspond to the lower integer root of the inequality x(t-x)>=d
fn compute_lower(time: i64, distance: i64) -> i64 {
    let delta = time * time - 4 * distance;

    match perfect_square_root(delta) {
        None => ((time as f64 - (delta as f64).sqrt()) / 2f64).ceil() as i64,
        Some(psr) => (time - psr) / 2 + 1,
    }
}

// Compute the maximum value for which we would go further than the reference.
// This correspond to the upper integer root of the inequality x(t-x)>=d
fn compute_higher(time: i64, distance: i64) -> i64 {
    let delta = time * time - 4 * distance;

    match perfect_square_root(delta) {
        None => ((time as f64 + (delta as f64).sqrt()) / 2f64).floor() as i64,
        Some(psr) => (time + psr - 1) / 2,
    }
}

fn solve_part_one(data: &Races) -> i64 {
    data.races
        .iter()
        .map(|race| {
            compute_higher(race.time, race.distance) - compute_lower(race.time, race.distance) + 1
        })
        .product()
}

fn solve_part_two(data: &Races) -> i64 {
    let time = data.races.iter().fold(0, |acc, race| {
        acc * 10i64.pow(race.time.ilog10() + 1) + race.time
    });
    let distance = data.races.iter().fold(0, |acc, race| {
        acc * 10i64.pow(race.distance.ilog10() + 1) + race.distance
    });

    compute_higher(time, distance) - compute_lower(time, distance) + 1
}

fn main() {
    let args = get_args();
    let data: String = fs::read_to_string(&args.path).unwrap();
    let parsed_data = parse_data(&data);

    if args.verbose {
        println!("{:#?}", &parsed_data);
    }

    println!("Part one solution: {}", solve_part_one(&parsed_data));
    // Should take 3-4 minutes
    println!("Part two solution: {}", solve_part_two(&parsed_data));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT_EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(&parse_data(INPUT_EXAMPLE)), 288);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE)), 71503);
    }
}
