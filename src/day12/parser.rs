use crate::day12::models::{ConditionRecord, SpringStatus};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::num::ParseIntError;
use std::str::FromStr;

fn parse_condition_record(input: &str) -> IResult<&str, ConditionRecord> {
    map_res(
        tuple((
            many1(alt((tag("#"), tag("."), tag("?")))),
            space1,
            separated_list1(tag(","), digit1),
            line_ending,
        )),
        |(spring_status, _, spring_groups, _)| {
            Ok::<_, ParseIntError>(ConditionRecord {
                spring_status: spring_status
                    .into_iter()
                    .map(|status| match status {
                        "." => SpringStatus::Operational,
                        "#" => SpringStatus::Damaged,
                        "?" => SpringStatus::Unknown,
                        _ => unreachable!(),
                    })
                    .collect(),
                spring_groups: spring_groups
                    .into_iter()
                    .map(|group| usize::from_str(group))
                    .collect::<Result<Vec<_>, _>>()?,
            })
        },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Vec<ConditionRecord> {
    let (res, conditions_records) = many1(parse_condition_record).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    conditions_records
}
