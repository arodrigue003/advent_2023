mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day18::logic::flood_dig_plan;
use crate::day18::models::DigPlan;
use crate::day18::parser::parse_input;
use crate::models::AdventSolution;

#[derive(Default)]
pub struct Day18 {
    part_01: Option<DigPlan>,
    part_02: Option<DigPlan>,
}

impl AdventSolution for Day18 {
    fn parse(&mut self, _data: String) {
        let dig_plans = parse_input(_data);
        self.part_01 = Some(dig_plans.0);
        self.part_02 = Some(dig_plans.1);
    }

    fn solve_part_one(&self) -> i128 {
        flood_dig_plan(self.part_01.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        flood_dig_plan(self.part_02.as_ref().unwrap()) as i128
    }
}
