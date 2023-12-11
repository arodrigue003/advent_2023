use std::fmt::Debug;
use std::path::PathBuf;

use clap::Parser;

use crate::models::AdventSolution;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
pub struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    pub path: PathBuf,
}

pub fn test_part_one_common<S: AdventSolution>(mut solution: S, data: &str, expected_result: i128) {
    solution.parse(data.to_string());
    solution.prepare();
    assert_eq!(solution.solve_part_one(), expected_result);
}

pub fn test_part_two_common<S: AdventSolution>(mut solution: S, data: &str, expected_result: i128) {
    solution.parse(data.to_string());
    solution.prepare();
    assert_eq!(solution.solve_part_two(), expected_result);
}
