use crate::models::{Direction, Grid, PipeLoop, Point, Tile};
use std::collections::HashSet;

impl Tile {
    /// Return the output direction considering we entered the tile from the given direction
    pub fn get_output_direction(&self, direction: Direction) -> Option<Direction> {
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
    pub fn is_pointing(&self, direction: Direction, start_tile: Tile) -> bool {
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

    /// Return true if a tile at the left of the current one is connected to us or not
    pub fn is_connected(&self, left: Tile, start_tile: Tile) -> bool {
        let current_tile = if self == &Tile::Start {
            start_tile
        } else {
            *self
        };

        match current_tile {
            Tile::Ground => false,
            Tile::Vertical => false,
            Tile::Horizontal => left.is_pointing(Direction::East, start_tile),
            Tile::TopLeft => false,
            Tile::TopRight => left.is_pointing(Direction::East, start_tile),
            Tile::BottomLeft => false,
            Tile::BottomRight => left.is_pointing(Direction::East, start_tile),
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

impl Grid {
    pub fn get_loop(&self) -> PipeLoop {
        // Determinate the loop direction by starting every starting direction
        for starting_direction in [
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ] {
            let mut size = 0;
            let mut direction = starting_direction;
            let mut position = self.start.clone();
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
                if position == self.start {
                    return PipeLoop {
                        start_tile: Tile::from((starting_direction, direction)),
                        loop_elements,
                        size,
                    };
                }

                // Get the output direction
                direction = match self.tiles[position.line][position.column]
                    .get_output_direction(direction)
                {
                    None => break,
                    Some(direction) => direction,
                };
            }
        }

        unreachable!()
    }
}
