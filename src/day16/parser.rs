use crate::day16::models::{Contraption, Tile, TileContent};

impl From<char> for TileContent {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '/' => Self::Mirror,
            '\\' => Self::AntiMirror,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self::new(From::from(value))
    }
}

pub fn parse_input(input: String) -> Contraption {
    Contraption::new(
        input
            .lines()
            .map(|line| line.chars().map(From::from).collect())
            .collect(),
    )
}
