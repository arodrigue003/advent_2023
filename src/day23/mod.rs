mod logic;
mod models;
mod parser;

#[cfg(test)]
mod tests;

use crate::day23::logic::{prepare_data, solve_part_one, solve_part_two};
use crate::day23::models::Map;
use crate::day23::parser::parse_input;
use crate::models::AdventSolution;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[derive(Default)]
pub struct Day23 {
    parsed_data: Option<Map>,
    prepared_data: Option<(Graph<(usize, usize), f64>, NodeIndex, NodeIndex)>,
}

impl AdventSolution for Day23 {
    fn parse(&mut self, _data: String) {
        self.parsed_data = Some(parse_input(_data));
    }

    fn prepare(&mut self) {
        self.prepared_data = Some(prepare_data(self.parsed_data.as_ref().unwrap()))
    }

    fn solve_part_one(&self) -> i128 {
        solve_part_one(self.prepared_data.as_ref().unwrap()) as i128
    }

    fn solve_part_two(&self) -> i128 {
        solve_part_two(self.prepared_data.as_ref().unwrap()) as i128
    }
}
