mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day10::logic::{get_loop, solve_part_one, solve_part_two};
use crate::day10::models::{Grid, PipeLoop};
use crate::day10::parser::parse_input;
use crate::models::AdventSolution;

pub struct Day10 {
    parsed_data: Option<Grid>,
    prepared_data: Option<PipeLoop>,
}

impl Day10 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            parsed_data: None,
            prepared_data: None,
        }
    }
}

impl AdventSolution for Day10 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        self.prepared_data = Some(get_loop(&self.parsed_data.as_ref().unwrap()));
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(&self.prepared_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(
            &self.parsed_data.as_ref().unwrap(),
            &self.prepared_data.as_ref().unwrap(),
        ) as i128
    }
}
