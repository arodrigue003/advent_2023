mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day13::logic::{solve_part_one, solve_part_one_2, solve_part_two, solve_part_two_2};
use crate::day13::models::{Grid, Grid2};
use crate::day13::parser::{parse_input, parse_input_2};
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day13 {
    parsed_data: Option<Vec<Grid>>,
}

impl AdventSolution for Day13 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input_2(_data));
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one_2(self.parsed_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two_2(self.parsed_data.as_ref().unwrap()) as i128
    }
}
