use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{map, map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom::Parser;

use crate::day06::models::{Race, Races};

fn parse_times(input: &str) -> IResult<&str, Vec<i64>> {
    map_res(
        delimited(
            tuple((tag("Time:"), space1)),
            separated_list1(space1, digit1),
            opt(line_ending),
        ),
        |times| times.into_iter().map(i64::from_str).collect(),
    )
    .parse(input)
}

fn parse_distances(input: &str) -> IResult<&str, Vec<i64>> {
    map_res(
        delimited(
            tuple((tag("Distance:"), space1)),
            separated_list1(space1, digit1),
            opt(line_ending),
        ),
        |times| times.into_iter().map(i64::from_str).collect(),
    )
    .parse(input)
}

fn parse_races(input: &str) -> IResult<&str, Races> {
    map(tuple((parse_times, parse_distances)), |(times, distances)| Races {
        races: times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect(),
    })
    .parse(input)
}

pub fn parse_input(input: String) -> Races {
    let (res, almanac) = parse_races(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    almanac
}
