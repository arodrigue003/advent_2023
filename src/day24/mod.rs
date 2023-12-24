mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day24::logic::{solve_part_one, solve_part_two};
use crate::day24::models::Hail;
use crate::day24::parser::parse_input;
use crate::models::AdventSolution;

pub struct Day24 {
    lower_bound: i128,
    upper_bound: i128,
    parsed_data: Option<Hail>,
}

impl Default for Day24 {
    fn default() -> Self {
        Self {
            lower_bound: 200_000_000_000_000,
            upper_bound: 400_000_000_000_000,
            parsed_data: None,
        }
    }
}

impl Day24 {
    pub fn new(lower_bound: i128, upper_bound: i128) -> Self {
        Self {
            lower_bound,
            upper_bound,
            parsed_data: None,
        }
    }
}

impl AdventSolution for Day24 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(self.parsed_data.as_ref().unwrap(), self.lower_bound, self.upper_bound) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(self.parsed_data.as_ref().unwrap()) as i128
    }
}
