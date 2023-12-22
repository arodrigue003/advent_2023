mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day22::logic::{prepare_data, solve_part_one, solve_part_two};
use crate::day22::models::FallingBricks;
use crate::day22::parser::parse_input;
use crate::models::AdventSolution;
use petgraph::Graph;

#[derive(Default)]
pub struct Day22 {
    parsed_data: Option<FallingBricks>,
    prepared_data: Option<Graph<usize, ()>>,
}

impl AdventSolution for Day22 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        let (bricks, graph) = prepare_data(self.parsed_data.clone().unwrap());
        self.parsed_data = Some(bricks);
        self.prepared_data = Some(graph)
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(self.prepared_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(self.prepared_data.as_ref().unwrap()) as i128
    }
}
