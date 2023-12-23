use crate::day23::models::{Map, Tile};

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '^' => Self::Top,
            '>' => Self::Right,
            'v' => Self::Bottom,
            '<' => Self::Left,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

pub fn parse_input(input: String) -> Map {
    Map::new(
        input
            .lines()
            .map(|line| line.chars().map(From::from).collect())
            .collect(),
    )
}
