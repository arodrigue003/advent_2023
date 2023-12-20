use crate::day20::models::{Broadcaster, CableManagement, Conjunction, FlipFlop, Module, ModuleType, Untyped};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, alpha1, line_ending};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::tuple;
use nom::{IResult, Parser};
use std::collections::HashMap;

impl From<&str> for ModuleType {
    fn from(value: &str) -> Self {
        match value {
            "broadcaster" => Self::Broadcaster,
            "%" => Self::FlipFlop,
            "&" => Self::Conjunction,
            _ => unreachable!(),
        }
    }
}

fn parse_module(input: &str) -> IResult<&str, (ModuleType, &str, Vec<&str>)> {
    map(
        tuple((
            alt((tag("broadcaster"), tag("%"), tag("&"))),
            alpha0,
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
            opt(line_ending),
        )),
        |(module_type, name, _, output, _)| (ModuleType::from(module_type), name, output),
    )
    .parse(input)
}

fn build_modules<'a>(mut modules_data: Vec<(ModuleType, &'a str, Vec<&'a str>)>) -> Vec<Box<dyn Module>> {
    // First build a hashmap of module names in order to be able to get them quickly after.
    let mut modules_positions: HashMap<_, _> = modules_data
        .iter()
        .enumerate()
        .map(|(position, (_, name, _))| (*name, position))
        .collect();

    // Then build a hashmap of modules input count in order to determinate how many input each module has.
    // We also get the list of untyped output
    let mut modules_input_count: HashMap<&str, u16> = HashMap::new();
    let mut untyped_outputs: Vec<&str> = vec![];
    for (_, _, output) in &modules_data {
        for output in output {
            // Update the input count
            *modules_input_count.entry(*output).or_default() += 1;

            // Add it to the untyped output list if necessary
            if modules_positions.get(*output).is_none() {
                untyped_outputs.push(*output);
            }
        }
    }

    // Add back the untyped output to the list
    for untyped_output in untyped_outputs {
        modules_data.push((ModuleType::Untyped, untyped_output, vec![]));
        modules_positions.insert(untyped_output, modules_data.len() - 1);
    }

    // Build a hashmap to store the progression in each module input phase
    let mut modules_input_offset: HashMap<&str, u16> = HashMap::new();

    // Build the modules
    let mut modules: Vec<Box<dyn Module>> = vec![];
    for (module_type, name, output) in modules_data {
        // Convert the output to a tuple of module position, offset
        let output: Vec<_> = output
            .into_iter()
            .map(|output_name| {
                let output_position = modules_positions[output_name];
                let entry = modules_input_offset.entry(output_name).or_default();
                let output_offset = *entry;
                *entry += 1;
                (output_position, output_offset)
            })
            .collect();

        modules.push(match module_type {
            ModuleType::Broadcaster => Box::new(Broadcaster::new(output)),
            ModuleType::FlipFlop => Box::new(FlipFlop::new(name.to_string(), output)),
            ModuleType::Conjunction => Box::new(Conjunction::new(name.to_string(), output, modules_input_count[name])),
            ModuleType::Untyped => Box::new(Untyped::new(name.to_string())),
        });
    }

    modules
}

fn parse_cable_management(input: &str) -> IResult<&str, CableManagement> {
    map(many1(parse_module), |modules| CableManagement {
        modules: build_modules(modules),
    })
    .parse(input)
}

pub fn parse_input(input: String) -> CableManagement {
    let (res, cable_management) = parse_cable_management(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    cable_management
}
