use std::num::ParseIntError;
use std::str::FromStr;

use nom::bytes::complete::{tag, take};
use nom::character::complete::{digit1, line_ending, one_of, space1};
use nom::combinator::{map, map_res, opt};
use nom::multi::many1;
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::day18::models::{DigPlan, Direction, Instruction};

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

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "3" => Self::Up,
            "0" => Self::Right,
            "1" => Self::Down,
            "2" => Self::Left,
            _ => unreachable!(),
        }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, (Instruction, Instruction)> {
    map_res(
        tuple((
            one_of("URDL"),
            space1,
            digit1,
            space1,
            tag("(#"),
            take(5usize),
            take(1usize),
            tag(")"),
            opt(line_ending),
        )),
        |(direction, _, distance, _, _, distance2, direction2, _, _)| {
            Ok::<_, ParseIntError>((
                Instruction {
                    direction: Direction::from(direction),
                    distance: i64::from_str(distance)?,
                },
                Instruction {
                    direction: Direction::from(direction2),
                    distance: i64::from_str_radix(distance2, 16)?,
                },
            ))
        },
    )
    .parse(input)
}

fn parse_dig_plan(input: &str) -> IResult<&str, (DigPlan, DigPlan)> {
    map(many1(parse_instruction), |instructions| {
        let instruction_sets = instructions.into_iter().unzip();
        (
            DigPlan {
                instructions: instruction_sets.0,
            },
            DigPlan {
                instructions: instruction_sets.1,
            },
        )
    })
    .parse(input)
}

pub fn parse_input(input: String) -> (DigPlan, DigPlan) {
    let (res, dig_plans) = parse_dig_plan(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    dig_plans
}
