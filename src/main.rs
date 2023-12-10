use advent_2023::day01::Day01;
use advent_2023::day02::Day02;
use advent_2023::day03::Day03;
use advent_2023::day04::Day04;
use advent_2023::day05::Day05;
use advent_2023::day06::Day06;
use advent_2023::day07::Day07;
use advent_2023::day08::Day08;
use advent_2023::day09::Day09;
use advent_2023::day10::Day10;
use advent_2023::models::AdventSolution;
use clap::{Args, Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Eq, PartialEq, Clone)]
enum Commands {
    /// Run every solution
    All,

    /// Run a specific day
    Day(DayArgs),
}

#[derive(Args, Debug, Eq, PartialEq, Clone)]
struct DayArgs {
    /// Day
    pub day: usize,

    /// File to parse
    pub path: PathBuf,
}

fn run_all(solutions: &mut Vec<Box<dyn AdventSolution>>) {
    for (i, solution) in solutions.iter_mut().enumerate() {
        let input = fs::read_to_string(format!("inputs/day{:0>2}", i + 1)).unwrap();

        run_day(i, solution, input);
    }
}

fn run_day(day: usize, solution: &mut Box<dyn AdventSolution>, input: String) {
    // Parse the data
    let now = Instant::now();
    solution.parse(input);
    let parse_time = now.elapsed().as_micros();

    // Prepare the parsed_data
    let now = Instant::now();
    solution.prepare();
    let prep_time = now.elapsed().as_micros();

    // Solve part one
    let now = Instant::now();
    let part_01_sol = solution.solve_part_one();
    let part_01_time = now.elapsed().as_micros();

    // Solve part two
    let now = Instant::now();
    let part_02_sol = solution.solve_part_two();
    let part_02_time = now.elapsed().as_micros();

    // Display the result
    println!(
        "Day {:0>2}, results: {:>14}, {:>14}, \
            parse_time: {:>10} us, prep_time: {:>10} us, \
            part_01_time: {:>10} us, part_02_time: {:>10} us",
        day + 1,
        part_01_sol,
        part_02_sol,
        parse_time,
        prep_time,
        part_01_time,
        part_02_time
    );
}

fn main() {
    let mut solutions: Vec<Box<dyn AdventSolution>> = vec![
        Box::new(Day01::new()),
        Box::new(Day02::new()),
        Box::new(Day03::new()),
        Box::new(Day04::new()),
        Box::new(Day05::new()),
        Box::new(Day06::new()),
        Box::new(Day07::new()),
        Box::new(Day08::new()),
        Box::new(Day09::new()),
        Box::new(Day10::new()),
    ];

    let arguments = Cli::parse();

    match arguments.command {
        Commands::All => {
            run_all(&mut solutions);
        }
        Commands::Day(day_args) => {
            let input = fs::read_to_string(day_args.path).unwrap();
            run_day(day_args.day, &mut solutions[day_args.day - 1], input);
        }
    }
}
