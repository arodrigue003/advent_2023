use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Tile {
    Empty,
    Top,
    Right,
    Bottom,
    Left,
    Wall,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".",
                Tile::Top => "^",
                Tile::Right => ">",
                Tile::Bottom => "v",
                Tile::Left => "<",
                Tile::Wall => "#",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Map {
    pub grid: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(grid: Vec<Vec<Tile>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        Self { grid, width, height }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for tile in line {
                write!(f, "{tile}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
