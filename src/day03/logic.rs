use crate::day03::models::{Schematic, SchematicCell};
use std::collections::HashMap;

pub fn solve_part_one(data: &Schematic) -> u32 {
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

pub fn solve_part_two(data: &Schematic) -> u32 {
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
