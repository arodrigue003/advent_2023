use std::collections::HashMap;
use std::fs;

use advent_2023_common::get_args;
use day03::models::{Schematic, SchematicCell};

fn solve_part_one(data: &Schematic) -> u32 {
    data.engine_parts
        .iter()
        .filter(|engine_part| {
            (engine_part.col_start - 1..=engine_part.col_end + 1).any(|i_col| {
                data.grid[engine_part.line - 1][i_col].is_symbol()
                    || data.grid[engine_part.line + 1][i_col].is_symbol()
            }) || data.grid[engine_part.line][engine_part.col_start - 1].is_symbol()
                || data.grid[engine_part.line][engine_part.col_end + 1].is_symbol()
        })
        .map(|engine_part| engine_part.value)
        .sum()
}

fn solve_part_two(data: &Schematic) -> u32 {
    let mut gear_connexions: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for engine_part in &data.engine_parts {
        // Look at the top end at the bottom of the part for a gear
        for col in engine_part.col_start - 1..=engine_part.col_end + 1 {
            if let SchematicCell::Symbol('*') = data.grid[engine_part.line - 1][col] {
                gear_connexions
                    .entry((engine_part.line - 1, col))
                    .or_default()
                    .push(engine_part.value)
            };
            if let SchematicCell::Symbol('*') = data.grid[engine_part.line + 1][col] {
                gear_connexions
                    .entry((engine_part.line + 1, col))
                    .or_default()
                    .push(engine_part.value)
            }
        }
        // Look at the left
        if let SchematicCell::Symbol('*') = data.grid[engine_part.line][engine_part.col_start - 1] {
            gear_connexions
                .entry((engine_part.line, engine_part.col_start - 1))
                .or_default()
                .push(engine_part.value)
        }
        // Look at the right
        if let SchematicCell::Symbol('*') = data.grid[engine_part.line][engine_part.col_end + 1] {
            gear_connexions
                .entry((engine_part.line, engine_part.col_end + 1))
                .or_default()
                .push(engine_part.value)
        }
    }

    gear_connexions
        .values()
        .filter(|values| values.len() == 2)
        .map(|values| values[0] * values[1])
        .sum()
}

fn main() {
    let args = get_args();
    let data: String = fs::read_to_string(&args.path).unwrap();
    let schematic = Schematic::new(&data);

    if args.verbose {
        println!("{}", &schematic);
    }

    println!("Part one solution: {}", solve_part_one(&schematic));
    println!("Part two solution: {}", solve_part_two(&schematic));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT_EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(&Schematic::new(INPUT_EXAMPLE)), 4361);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&Schematic::new(INPUT_EXAMPLE)), 467835);
    }
}
