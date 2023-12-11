use std::num::ParseIntError;
use std::str::FromStr;

use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{map_res, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;

use crate::day04::models::Game;

fn parse_game(input: &str) -> IResult<&str, Game> {
    map_res(
        tuple((
            tuple((tag("Card"), space1)),
            digit1,
            tuple((tag(":"), space1)),
            separated_list1(space1, digit1),
            tuple((tag(" |"), space1)),
            separated_list1(space1, digit1),
            opt(line_ending),
        )),
        |(_, index, _, wining, _, draw, _)| {
            Ok::<_, ParseIntError>(Game {
                index: usize::from_str(index)?,
                winning: wining
                    .into_iter()
                    .map(u8::from_str)
                    .collect::<Result<_, ParseIntError>>()?,
                draw: draw
                    .into_iter()
                    .map(u8::from_str)
                    .collect::<Result<_, ParseIntError>>()?,
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
