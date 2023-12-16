use std::num::ParseIntError;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{map, map_res, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::day02::models::{Game, GameSubset};

fn parse_cube(input: &str) -> IResult<&str, GameSubset> {
    map_res(
        tuple((digit1, space1, alt((tag("blue"), tag("red"), tag("green"))))),
        |(value, _, color)| {
            let value = u32::from_str(value)?;
            Ok::<_, ParseIntError>(match color {
                "red" => GameSubset::red(value),
                "green" => GameSubset::green(value),
                "blue" => GameSubset::blue(value),
                _ => unreachable!(),
            })
        },
    )
    .parse(input)
}

fn parse_subset(input: &str) -> IResult<&str, GameSubset> {
    map(separated_list1(tag(", "), parse_cube), |cubes| cubes.into_iter().sum()).parse(input)
}

fn parse_subsets(input: &str) -> IResult<&str, Vec<GameSubset>> {
    separated_list1(tag("; "), parse_subset).parse(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    map_res(
        tuple((tag("Game "), digit1, tag(": "), parse_subsets, opt(line_ending))),
        |(_, index, _, subsets, _)| {
            Ok::<_, ParseIntError>(Game {
                index: u32::from_str(index)?,
                subsets,
            })
        },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Vec<Game> {
    let (res, games) = many1(parse_game).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    games
}
