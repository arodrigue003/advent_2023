use std::num::ParseIntError;
use std::str::FromStr;

use crate::day05::models::{Almanac, Mapping};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{map, map_res};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom::Parser;

fn parse_seeds(input: &str) -> IResult<&str, Vec<i64>> {
    map_res(
        delimited(
            tag("seeds: "),
            separated_list1(space1, digit1),
            many1(line_ending),
        ),
        |seeds| seeds.into_iter().map(i64::from_str).collect(),
    )
    .parse(input)
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    map_res(
        tuple((digit1, space1, digit1, space1, digit1, line_ending)),
        |(dst_start, _, src_start, _, size, _)| {
            Ok::<_, ParseIntError>(Mapping {
                src_start: i64::from_str(src_start)?,
                dst_start: i64::from_str(dst_start)?,
                size: i64::from_str(size)?,
            })
        },
    )
    .parse(input)
}

fn parse_mapping_group(input: &str) -> IResult<&str, Vec<Mapping>> {
    map(
        delimited(
            tuple((take_until("map:"), tag("map:"), line_ending)),
            many1(parse_mapping),
            many0(line_ending),
        ),
        |mut mapping_group| {
            mapping_group.sort();
            mapping_group
        },
    )
    .parse(input)
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    map(
        tuple((parse_seeds, many1(parse_mapping_group))),
        |(seeds, mappings)| Almanac { seeds, mappings },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Almanac {
    let (res, almanac) = parse_almanac(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    almanac
}
