mod logic;

#[cfg(test)]
mod tests;

use crate::day01::logic::{solve_part_one, solve_part_two};
use crate::models::AdventSolution;

pub struct Day01 {
    parsed_data: Option<String>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { parsed_data: None }
    }
}

impl AdventSolution for Day01 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(_data);
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(&self.parsed_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(&self.parsed_data.as_ref().unwrap()) as i128
    }
}
