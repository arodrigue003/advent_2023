use std::collections::HashMap;

use crate::day08::models::{Direction, NavigationMap, Node};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, line_ending};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{terminated, tuple};
use nom::IResult;
use nom::Parser;

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    map(
        terminated(alpha1, many1(line_ending)),
        |directions: &str| {
            directions
                .chars()
                .map(|char| match char {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => unreachable!(),
                })
                .collect()
        },
    )
    .parse(input)
}

fn parse_node(input: &str) -> IResult<&str, (&str, &str, &str)> {
    map(
        tuple((
            alphanumeric1,
            tag(" = ("),
            alphanumeric1,
            tag(", "),
            alphanumeric1,
            tag(")"),
            opt(line_ending),
        )),
        |(name, _, left, _, right, _, _): (&str, _, &str, _, &str, _, _)| (name, left, right),
    )
    .parse(input)
}

fn build_nodes(nodes_data: Vec<(&str, &str, &str)>) -> Vec<Node> {
    // First build a hashmap of node names in order to be able to get them quickly after
    let nodes_positions: HashMap<_, _> = nodes_data
        .iter()
        .enumerate()
        .map(|(position, (name, _, _))| (*name, position))
        .collect();

    // Then create the vec of nodes iteratively
    nodes_data
        .into_iter()
        .map(|(name, left, right)| Node {
            name: name.to_string(),
            left: nodes_positions[left],
            right: nodes_positions[right],
        })
        .collect()
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    map(many1(parse_node), build_nodes).parse(input)
}

fn parse_navigation_map(input: &str) -> IResult<&str, NavigationMap> {
    map(
        tuple((parse_directions, parse_nodes)),
        |(instructions, nodes)| NavigationMap {
            instructions,
            nodes,
        },
    )
    .parse(input)
}

pub fn parse_input(input: String) -> NavigationMap {
    let (res, navigation_map) = parse_navigation_map(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    navigation_map
}
