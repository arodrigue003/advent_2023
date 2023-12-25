use crate::day25::models::Connection;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::{terminated, tuple};
use nom::{IResult, Parser};

fn parse_connection(input: &str) -> IResult<&str, Connection> {
    map(
        terminated(
            tuple((alpha1, tag(": "), separated_list1(space1, alpha1))),
            opt(line_ending),
        ),
        |(name, _, others): (&str, _, Vec<&str>)| Connection {
            name: name.to_string(),
            others: others.into_iter().map(|other| other.to_string()).collect(),
        },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Vec<Connection> {
    let (res, games) = many1(parse_connection).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    games
}
