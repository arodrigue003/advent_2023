use std::num::ParseIntError;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::combinator::{map, map_res, opt};
use nom::multi::separated_list1;
use nom::sequence::{terminated, tuple};
use nom::{IResult, Parser};

use crate::day15::models::{Action, Step};

fn parse_remove(input: &str) -> IResult<&str, Action> {
    map(tag("-"), |_| Action::Remove).parse(input)
}

fn parse_add(input: &str) -> IResult<&str, Action> {
    map_res(tuple((tag("="), digit1)), |(_, value)| {
        Ok::<_, ParseIntError>(Action::Add(usize::from_str(value)?))
    })
    .parse(input)
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    alt((parse_remove, parse_add)).parse(input)
}

fn parse_step(input: &str) -> IResult<&str, Step> {
    map(tuple((alpha1, parse_action)), |(lens, action)| Step {
        label: lens.to_string(),
        action,
    })
    .parse(input)
}

pub fn parse_input(input: String) -> Vec<Step> {
    let (res, games) = terminated(separated_list1(tag(","), parse_step), opt(line_ending))
        .parse(&input)
        .unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    games
}
