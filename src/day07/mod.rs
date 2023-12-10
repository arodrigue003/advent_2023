mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day07::logic::{solve_part_one, solve_part_two};
use crate::day07::models::Hand;
use crate::day07::parser::parse_input;
use crate::models::AdventSolution;

pub struct Day07 {
    parsed_data: Option<Vec<Hand>>,
}

impl Day07 {
    pub fn new() -> Self {
        Self { parsed_data: None }
    }
}

impl AdventSolution for Day07 {
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
