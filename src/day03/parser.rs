use crate::day03::models::{EnginePart, Schematic, SchematicCell};
use itertools::Itertools;

static ZERO_VALUE: u8 = b'0';

pub fn parse_input(input: String) -> Schematic {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|cell| match cell {
                    '.' => SchematicCell::Empty,
                    '0'..='9' => SchematicCell::Part(cell as u8 - ZERO_VALUE),
                    _ => SchematicCell::Symbol(cell),
                })
                .collect()
        })
        .collect();

    // Compute grid dimension
    let width = grid[0].len();
    let height = grid.len();

    // Add edges around the grid
    let grid: Vec<Vec<_>> = std::iter::once(vec![SchematicCell::Empty; width + 2])
        .chain(grid.into_iter().map(|line| {
            std::iter::once(SchematicCell::Empty)
                .chain(line)
                .chain(std::iter::once(SchematicCell::Empty))
                .collect()
        }))
        .chain(std::iter::once(vec![SchematicCell::Empty; width + 2]))
        .collect();

    // Extract engine part
    let engine_parts: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(i_line, line)| {
            line.iter()
                .enumerate()
                .group_by(|(_, cell)| cell.is_part())
                .into_iter()
                .filter(|(key, _)| *key)
                .map(|(_, group)| {
                    group.into_iter().fold(
                        (usize::MAX, 0, 0),
                        |(start, _end, value), (i_col, cell)| {
                            let start = if start == usize::MAX { i_col } else { start };
                            match cell {
                                SchematicCell::Part(x) => (start, i_col, value * 10 + (*x as u32)),
                                _ => unreachable!(),
                            }
                        },
                    )
                })
                .map(|(start, end, value)| EnginePart {
                    value,
                    col_start: start,
                    col_end: end,
                    line: i_line,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    Schematic {
        grid,
        width: width + 2,
        height: height + 2,
        engine_parts,
    }
}
