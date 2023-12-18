use crate::day18::models::{Color, DigPlan, Direction, Instruction};
use nom::bytes::complete::{tag, take};
use nom::character::complete::{digit1, line_ending, one_of, space1};
use nom::combinator::{map, map_res, opt};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::num::ParseIntError;
use std::str::FromStr;

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            _ => unreachable!(),
        }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    map_res(
        tuple((
            one_of("URDL"),
            space1,
            digit1,
            space1,
            tag("(#"),
            take(2usize),
            take(2usize),
            take(2usize),
            tag(")"),
            opt(line_ending),
        )),
        |(direction, _, distance, _, _, red, green, blue, _, _)| {
            Ok::<_, ParseIntError>(Instruction {
                direction: Direction::from(direction),
                distance: i32::from_str(distance)?,
                color: Color {
                    red: u8::from_str_radix(red, 16)?,
                    green: u8::from_str_radix(green, 16)?,
                    blue: u8::from_str_radix(blue, 16)?,
                },
            })
        },
    )
    .parse(input)
}

fn parse_dig_plan(input: &str) -> IResult<&str, DigPlan> {
    map(many1(parse_instruction), |instructions| DigPlan { instructions }).parse(input)
}

pub fn parse_input(input: String) -> DigPlan {
    let (res, dig_plan) = parse_dig_plan(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    dig_plan
}
