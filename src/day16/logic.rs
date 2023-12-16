use crate::day16::models::{Contraption, Direction, Tile, TileContent};
use std::cmp::max;

impl Tile {
    pub fn get_output_direction(&self, direction: Direction) -> Direction {
        match (&self.content, direction) {
            (TileContent::Empty, direction) => direction,
            (TileContent::Mirror, Direction::Right) => Direction::Up,
            (TileContent::Mirror, Direction::Bot) => Direction::Left,
            (TileContent::Mirror, Direction::Left) => Direction::Bot,
            (TileContent::Mirror, Direction::Up) => Direction::Right,
            (TileContent::AntiMirror, Direction::Right) => Direction::Bot,
            (TileContent::AntiMirror, Direction::Bot) => Direction::Right,
            (TileContent::AntiMirror, Direction::Left) => Direction::Up,
            (TileContent::AntiMirror, Direction::Up) => Direction::Left,
            (TileContent::HorizontalSplitter, Direction::Right) => Direction::Right,
            (TileContent::HorizontalSplitter, Direction::Left) => Direction::Left,
            (TileContent::HorizontalSplitter, Direction::Up) => Direction::Left | Direction::Right,
            (TileContent::HorizontalSplitter, Direction::Bot) => Direction::Left | Direction::Right,
            (TileContent::VerticalSplitter, Direction::Right) => Direction::Up | Direction::Bot,
            (TileContent::VerticalSplitter, Direction::Left) => Direction::Up | Direction::Bot,
            (TileContent::VerticalSplitter, Direction::Up) => Direction::Up,
            (TileContent::VerticalSplitter, Direction::Bot) => Direction::Bot,
            _ => unreachable!(),
        }
    }
}

fn make_beam_progress(contraption: &mut Contraption, line: usize, column: usize, direction: Direction) {
    // If the contraption already has a beam in the current tile, return early
    if contraption.grid[line][column].contains_beam(direction) {
        return;
    }

    // Update the current tile to add the beam
    contraption.grid[line][column].insert_beam(direction);

    // Get output directions and make beam progress further
    for direction in contraption.grid[line][column].get_output_direction(direction) {
        match direction {
            Direction::Up => {
                if line > 0 {
                    make_beam_progress(contraption, line - 1, column, Direction::Up)
                }
            }
            Direction::Right => {
                if column + 1 < contraption.width {
                    make_beam_progress(contraption, line, column + 1, Direction::Right)
                }
            }
            Direction::Bot => {
                if line + 1 < contraption.height {
                    make_beam_progress(contraption, line + 1, column, Direction::Bot)
                }
            }
            Direction::Left => {
                if column > 0 {
                    make_beam_progress(contraption, line, column - 1, Direction::Left)
                }
            }
            _ => unreachable!(),
        }
    }
}

fn simulate_and_get_energized_tile_count(
    contraption: &Contraption,
    line: usize,
    column: usize,
    direction: Direction,
) -> u32 {
    // We need to modify the contraption
    let mut contraption = contraption.clone();

    // Make beam progress
    make_beam_progress(&mut contraption, line, column, direction);

    // Compute the number of energized tiles
    contraption
        .grid
        .iter()
        .flat_map(|line| line.iter().map(|tile| tile.is_energized() as u32))
        .sum()
}

pub fn solve_part_one(contraption: &Contraption) -> u32 {
    simulate_and_get_energized_tile_count(contraption, 0, 0, Direction::Right)
}

pub fn solve_part_two(contraption: &Contraption) -> u32 {
    (0..contraption.width)
        .map(|starting_column| {
            max(
                simulate_and_get_energized_tile_count(contraption, 0, starting_column, Direction::Bot),
                simulate_and_get_energized_tile_count(
                    contraption,
                    contraption.height - 1,
                    starting_column,
                    Direction::Up,
                ),
            )
        })
        .chain((0..contraption.height).map(|starting_line| {
            max(
                simulate_and_get_energized_tile_count(contraption, starting_line, 0, Direction::Right),
                simulate_and_get_energized_tile_count(
                    contraption,
                    starting_line,
                    contraption.width - 1,
                    Direction::Left,
                ),
            )
        }))
        .max()
        .unwrap()
}
