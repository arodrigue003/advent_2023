use std::fs;

use advent_2023_common::get_args;
use day05::logic;
use day05::models::{Almanac, Range};
use day05::parser::parse_data;
use itertools::Itertools;

fn solve_part_one(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .map(|seed| logic::map_value(*seed, almanac))
        .min()
        .unwrap()
}

fn solve_part_two(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .tuples()
        .flat_map(|(start, size)| {
            logic::map_range(
                Range {
                    start: *start,
                    end: *start + *size,
                },
                almanac,
            )
        })
        .map(|range| range.start)
        .min()
        .unwrap()
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
    use day05::models::Mapping;

    use super::*;

    static INPUT_EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_parse_data() {
        // Mapping order is not the same because we sorted the mapping during the parsing
        assert_eq!(
            parse_data(INPUT_EXAMPLE),
            Almanac {
                seeds: vec![79, 14, 55, 13,],
                mappings: vec![
                    vec![Mapping::new(50, 52, 48), Mapping::new(98, 50, 2),],
                    vec![
                        Mapping::new(0, 39, 15),
                        Mapping::new(15, 0, 37),
                        Mapping::new(52, 37, 2),
                    ],
                    vec![
                        Mapping::new(0, 42, 7),
                        Mapping::new(7, 57, 4),
                        Mapping::new(11, 0, 42),
                        Mapping::new(53, 49, 8),
                    ],
                    vec![Mapping::new(18, 88, 7), Mapping::new(25, 18, 70),],
                    vec![
                        Mapping::new(45, 81, 19),
                        Mapping::new(64, 68, 13),
                        Mapping::new(77, 45, 23),
                    ],
                    vec![Mapping::new(0, 1, 69), Mapping::new(69, 0, 1),],
                    vec![Mapping::new(56, 60, 37), Mapping::new(93, 56, 4),],
                ],
            }
        )
    }

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(&parse_data(INPUT_EXAMPLE)), 35);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE)), 46);
    }
}
