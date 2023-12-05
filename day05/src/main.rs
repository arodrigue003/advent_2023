use std::fs;

use advent_2023_common::get_args;
use day05::models::Almanac;
use day05::parser::parse_data;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use rayon::prelude::*;

// We make the hypothesis that mappings form a disjoint set of mapping.
fn map(value: i64, almanac: &Almanac) -> i64 {
    almanac.mappings.iter().fold(value, |acc, mapping| {
        mapping
            .iter()
            .fold_while(acc, |acc2, mapping| {
                if acc2 >= mapping.src_start && acc2 < mapping.src_start + mapping.size {
                    Done(acc2 + mapping.dst_start - mapping.src_start)
                } else {
                    Continue(acc2)
                }
            })
            .into_inner()
    })
}

fn solve_part_one(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .map(|seed| map(*seed, almanac))
        .min()
        .unwrap()
}

fn solve_part_two(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .tuples()
        .map(|(start, size)| {
            (*start..(*start + *size))
                .into_par_iter()
                .map(|seed| map(seed, almanac))
                .min()
                .unwrap()
        })
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
    // Should take 3-4 minutes
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
        assert_eq!(
            parse_data(INPUT_EXAMPLE),
            Almanac {
                seeds: vec![79, 14, 55, 13,],
                mappings: vec![
                    vec![Mapping::new(98, 50, 2), Mapping::new(50, 52, 48),],
                    vec![
                        Mapping::new(15, 0, 37),
                        Mapping::new(52, 37, 2),
                        Mapping::new(0, 39, 15),
                    ],
                    vec![
                        Mapping::new(53, 49, 8),
                        Mapping::new(11, 0, 42),
                        Mapping::new(0, 42, 7),
                        Mapping::new(7, 57, 4),
                    ],
                    vec![Mapping::new(18, 88, 7), Mapping::new(25, 18, 70),],
                    vec![
                        Mapping::new(77, 45, 23),
                        Mapping::new(45, 81, 19),
                        Mapping::new(64, 68, 13),
                    ],
                    vec![Mapping::new(69, 0, 1), Mapping::new(0, 1, 69),],
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
