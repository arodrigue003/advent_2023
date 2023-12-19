use crate::day19::models::{Action, Part, Rule, System, Workflow};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending, one_of};
use nom::combinator::{map, map_res, opt};
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::num::ParseIntError;
use std::str::FromStr;

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    map_res(
        tuple((one_of("xmas"), one_of("<>"), digit1, tag(":"), alpha1)),
        |(variable, test, value, _, action)| {
            let value = i32::from_str(value)?;
            let action = match action {
                "A" => Action::Accepted,
                "R" => Action::Rejected,
                s => Action::Goto(s.to_string()),
            };
            let test = match (variable, test) {
                ('x', '<') => |part: &Part, value: i32| part.x < value,
                ('x', '>') => |part: &Part, value: i32| part.x > value,
                ('m', '<') => |part: &Part, value: i32| part.m < value,
                ('m', '>') => |part: &Part, value: i32| part.m > value,
                ('a', '<') => |part: &Part, value: i32| part.a < value,
                ('a', '>') => |part: &Part, value: i32| part.a > value,
                ('s', '<') => |part: &Part, value: i32| part.s < value,
                ('s', '>') => |part: &Part, value: i32| part.s > value,
                _ => unreachable!(),
            };

            Ok::<_, ParseIntError>(Rule { value, action, test })
        },
    )
    .parse(input)
}

fn parse_workflow(input: &str) -> IResult<&str, (String, Workflow)> {
    map(
        tuple((
            alpha1,
            tag("{"),
            separated_list0(tag(","), parse_rule),
            tag(","),
            alpha1,
            tag("}"),
            opt(line_ending),
        )),
        |(name, _, rules, _, default_action, _, _)| {
            let default_action = match default_action {
                "A" => Action::Accepted,
                "R" => Action::Rejected,
                s => Action::Goto(s.to_string()),
            };

            (
                name.to_string(),
                Workflow {
                    name: name.to_string(),
                    rules,
                    default_action,
                },
            )
        },
    )
    .parse(input)
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    map_res(
        tuple((
            tag("{x="),
            digit1,
            tag(",m="),
            digit1,
            tag(",a="),
            digit1,
            tag(",s="),
            digit1,
            tag("}"),
            opt(line_ending),
        )),
        |(_, x, _, m, _, a, _, s, _, _)| {
            Ok::<_, ParseIntError>(Part {
                x: i32::from_str(x)?,
                m: i32::from_str(m)?,
                a: i32::from_str(a)?,
                s: i32::from_str(s)?,
            })
        },
    )
    .parse(input)
}

fn parse_system(input: &str) -> IResult<&str, System> {
    map(
        tuple((many1(parse_workflow), many1(line_ending), many1(parse_part))),
        |(workflows, _, parts)| System {
            workflows: workflows.into_iter().collect(),
            parts,
        },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> System {
    let (res, system) = parse_system(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    system
}
