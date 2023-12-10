use crate::day10::models::{Direction, Grid, PipeLoop, Point, Tile};
use std::collections::HashSet;

impl Tile {
    /// Return the output direction considering we entered the tile from the given direction
    fn get_output_direction(&self, direction: Direction) -> Option<Direction> {
        match (self, direction) {
            (Tile::Vertical, direction) => Some(direction),
            (Tile::Horizontal, direction) => Some(direction),
            (Tile::TopLeft, Direction::North) => Some(Direction::East),
            (Tile::TopLeft, Direction::West) => Some(Direction::South),
            (Tile::TopRight, Direction::North) => Some(Direction::West),
            (Tile::TopRight, Direction::East) => Some(Direction::South),
            (Tile::BottomRight, Direction::South) => Some(Direction::West),
            (Tile::BottomRight, Direction::East) => Some(Direction::North),
            (Tile::BottomLeft, Direction::South) => Some(Direction::East),
            (Tile::BottomLeft, Direction::West) => Some(Direction::North),
            (_, _) => None,
        }
    }

    /// Return true if the tile is pointing out to that direction.
    fn is_pointing(&self, direction: Direction, start_tile: Tile) -> bool {
        let current_tile = if self == &Tile::Start {
            start_tile
        } else {
            *self
        };

        match current_tile {
            Tile::Ground => false,
            Tile::Vertical => direction == Direction::North || direction == Direction::South,
            Tile::Horizontal => direction == Direction::East || direction == Direction::West,
            Tile::TopLeft => direction == Direction::South || direction == Direction::East,
            Tile::TopRight => direction == Direction::South || direction == Direction::West,
            Tile::BottomLeft => direction == Direction::North || direction == Direction::East,
            Tile::BottomRight => direction == Direction::North || direction == Direction::West,
            Tile::Start => unreachable!(),
        }
    }
}

impl From<(Direction, Direction)> for Tile {
    fn from(value: (Direction, Direction)) -> Self {
        match value {
            (Direction::North, Direction::North) => Self::Vertical,
            (Direction::South, Direction::South) => Self::Vertical,
            (Direction::East, Direction::East) => Self::Horizontal,
            (Direction::West, Direction::West) => Self::Horizontal,
            (Direction::South, Direction::West) => Self::TopLeft,
            (Direction::East, Direction::North) => Self::TopLeft,
            (Direction::South, Direction::East) => Self::TopRight,
            (Direction::West, Direction::North) => Self::TopRight,
            (Direction::North, Direction::East) => Self::BottomRight,
            (Direction::West, Direction::South) => Self::BottomRight,
            (Direction::North, Direction::West) => Self::BottomLeft,
            (Direction::East, Direction::South) => Self::BottomLeft,
            (_, _) => unreachable!(),
        }
    }
}

pub fn get_loop(grid: &Grid) -> PipeLoop {
    // Determinate the loop direction by starting every starting direction
    for starting_direction in [
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::North,
    ] {
        let mut size = 0;
        let mut direction = starting_direction;
        let mut position = grid.start.clone();
        let mut loop_elements = HashSet::new();
        loop {
            // Advance position depending on the direction
            position = match direction {
                Direction::North => Point::new(position.line - 1, position.column),
                Direction::East => Point::new(position.line, position.column + 1),
                Direction::South => Point::new(position.line + 1, position.column),
                Direction::West => Point::new(position.line, position.column - 1),
            };

            // Add the tile in the loop elements
            loop_elements.insert(position.clone());

            size += 1;

            // If we reached the start again, success
            if position == grid.start {
                return PipeLoop {
                    start_tile: Tile::from((starting_direction, direction)),
                    loop_elements,
                    size,
                };
            }

            // Get the output direction
            direction =
                match grid.tiles[position.line][position.column].get_output_direction(direction) {
                    None => break,
                    Some(direction) => direction,
                };
        }
    }

    unreachable!()
}

pub fn solve_part_one(pipe_loop: &PipeLoop) -> usize {
    pipe_loop.size / 2
}

pub fn solve_part_two(data: &Grid, pipe_loop: &PipeLoop) -> i64 {
    let mut inner_count = 0;
    for (i_line, line) in data.tiles.iter().enumerate() {
        let mut pointing_north = 0;
        let mut pointing_south = 0;

        for (i_column, tile) in line.iter().enumerate().skip(1) {
            let position = Point::new(i_line, i_column);

            if pipe_loop.loop_elements.contains(&position) {
                if tile.is_pointing(Direction::North, pipe_loop.start_tile) {
                    pointing_north += 1;
                }
                if tile.is_pointing(Direction::South, pipe_loop.start_tile) {
                    pointing_south += 1;
                }
            } else {
                if pointing_north % 2 == 1 && pointing_south % 2 == 1 {
                    inner_count += 1;
                }
            }
        }
    }

    inner_count
}
