use crate::day24::models::{Hail, Hailstone};
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{map, map_res, opt, recognize};
use nom::multi::many1;
use nom::sequence::{pair, tuple};
use nom::{IResult, Parser};
use std::str::FromStr;

fn parse_number(input: &str) -> IResult<&str, i128> {
    map_res(recognize(tuple((opt(tag("-")), digit1))), i128::from_str).parse(input)
}

fn parse_hailstone(input: &str) -> IResult<&str, Hailstone> {
    map(
        tuple((
            parse_number,
            pair(tag(","), space1),
            parse_number,
            pair(tag(","), space1),
            parse_number,
            tuple((space1, tag("@"), space1)),
            parse_number,
            pair(tag(","), space1),
            parse_number,
            pair(tag(","), space1),
            parse_number,
            opt(line_ending),
        )),
        |(x, _, y, _, z, _, vx, _, vy, _, vz, _)| Hailstone::new(x, y, z, vx, vy, vz),
    )
    .parse(input)
}

fn parse_hail(input: &str) -> IResult<&str, Hail> {
    map(many1(parse_hailstone), |hailstones| Hail { hailstones }).parse(input)
}

pub fn parse_input(input: String) -> Hail {
    let (res, games) = parse_hail(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    games
}
