use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Bot,
    Left,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum TileContent {
    Empty,
    Mirror,
    AntiMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Display for TileContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TileContent::Empty => ".",
                TileContent::Mirror => "/",
                TileContent::AntiMirror => r"\",
                TileContent::HorizontalSplitter => "-",
                TileContent::VerticalSplitter => "|",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tile {
    pub content: TileContent,
    pub beams: [bool; 4],
}

impl Tile {
    pub fn new(content: TileContent) -> Self {
        Self {
            content,
            beams: [false; 4],
        }
    }

    pub fn contains_beam(&self, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.beams[0],
            Direction::Right => self.beams[1],
            Direction::Bot => self.beams[2],
            Direction::Left => self.beams[3],
        }
    }

    pub fn insert_beam(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.beams[0] = true,
            Direction::Right => self.beams[1] = true,
            Direction::Bot => self.beams[2] = true,
            Direction::Left => self.beams[3] = true,
        }
    }

    pub fn is_energized(&self) -> bool {
        self.beams.iter().any(|value| *value)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Contraption {
    pub grid: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Contraption {
    pub fn new(grid: Vec<Vec<Tile>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();
        Self { grid, width, height }
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
