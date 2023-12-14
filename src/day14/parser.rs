use crate::day14::models::{Platform, Tile};

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'O' => Self::Round,
            '#' => Self::Square,
            _ => unreachable!(),
        }
    }
}

impl Platform {
    pub fn new(grid: Vec<Vec<Tile>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Self {
            grid,
            width,
            height,
        }
    }
}

pub fn parse_input(input: String) -> Platform {
    Platform::new(
        input
            .lines()
            .map(|line| line.chars().map(From::from).collect())
            .collect(),
    )
}
