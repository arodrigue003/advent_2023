use std::num::ParseIntError;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending};
use nom::combinator::{map, map_res, opt};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::day22::models::{Brick, FallingBricks, Point};

fn parse_brick(input: &str) -> IResult<&str, Brick> {
    map_res(
        tuple((
            digit1,
            tag(","),
            digit1,
            tag(","),
            digit1,
            tag("~"),
            digit1,
            tag(","),
            digit1,
            tag(","),
            digit1,
            opt(line_ending),
        )),
        |(start_x, _, start_y, _, start_z, _, end_x, _, end_y, _, end_z, _)| {
            Ok::<_, ParseIntError>(Brick::new(
                Point::new(
                    usize::from_str(start_x)?,
                    usize::from_str(start_y)?,
                    usize::from_str(start_z)?,
                ),
                Point::new(
                    usize::from_str(end_x)?,
                    usize::from_str(end_y)?,
                    usize::from_str(end_z)?,
                ),
            ))
        },
    )
    .parse(input)
}

fn parse_falling_bricks(input: &str) -> IResult<&str, FallingBricks> {
    map(many1(parse_brick), |bricks| FallingBricks { bricks }).parse(input)
}

pub fn parse_input(input: String) -> FallingBricks {
    let (res, games) = parse_falling_bricks(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    games
}
