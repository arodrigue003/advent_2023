mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day08::logic::{solve_part_one, solve_part_two};
use crate::day08::models::NavigationMap;
use crate::day08::parser::parse_input;
use crate::models::AdventSolution;

pub struct Day08 {
    parsed_data: Option<NavigationMap>,
}

impl Day08 {
    pub fn new() -> Self {
        Self { parsed_data: None }
    }
}

impl AdventSolution for Day08 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(&self.parsed_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(&self.parsed_data.as_ref().unwrap()) as i128
    }
}
