use std::fs;

use advent_2023_common::get_args;
use day09::models::SensorReport;
use day09::parser::parse_data;
use itertools::Itertools;

fn generate_next_line(line: &[i64]) -> Vec<i64> {
    line.iter()
        .tuple_windows()
        .map(|(left, right)| *right - *left)
        .collect()
}

fn solve_part_one(data: &SensorReport) -> i64 {
    let mut global_res = 0;
    for value_history in &data.values_history {
        let mut value_expansion = vec![value_history.clone()];

        loop {
            if value_expansion[value_expansion.len() - 1]
                .iter()
                .all(|value| *value == 0)
            {
                break;
            }
            value_expansion.push(generate_next_line(
                &value_expansion[value_expansion.len() - 1],
            ))
        }

        let mut res = 0;
        for i in (0..value_expansion.len()).rev() {
            res += value_expansion[i][value_expansion[i].len() - 1]
        }
        global_res += res;
    }
    global_res
}

fn solve_part_two(data: &SensorReport) -> i64 {
    let mut global_res = 0;
    for value_history in &data.values_history {
        let mut value_expansion = vec![value_history.clone()];

        loop {
            if value_expansion[value_expansion.len() - 1]
                .iter()
                .all(|value| *value == 0)
            {
                break;
            }
            value_expansion.push(generate_next_line(
                &value_expansion[value_expansion.len() - 1],
            ))
        }

        let mut res = 0;
        for i in (0..value_expansion.len()).rev() {
            res = value_expansion[i][0] - res;
        }
        global_res += res;
    }
    global_res
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

    static INPUT_EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(&parse_data(INPUT_EXAMPLE)), 114);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE)), 2);
    }
}
