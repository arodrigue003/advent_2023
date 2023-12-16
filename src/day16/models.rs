use std::fmt::{Display, Formatter};

use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub struct Direction: u32 {
        const Up = 0b00000001;
        const Right = 0b00000010;
        const Bot = 0b00000100;
        const Left = 0b00001000;
    }
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
    pub beams: Direction,
}

impl Tile {
    pub fn new(content: TileContent) -> Self {
        Self {
            content,
            beams: Direction::empty(),
        }
    }

    #[inline(always)]
    pub fn contains_beam(&self, direction: Direction) -> bool {
        self.beams.contains(direction)
    }

    #[inline(always)]
    pub fn insert_beam(&mut self, direction: Direction) {
        self.beams |= direction
    }

    #[inline(always)]
    pub fn is_energized(&self) -> bool {
        !self.beams.is_empty()
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Contraption {
    pub grid: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Contraption {
    pub fn new(grid: Vec<Vec<Tile>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        Self {
            grid: grid.into_iter().flat_map(|line| line.into_iter()).collect(),
            width,
            height,
        }
    }

    #[inline(always)]
    pub fn offset(&self, line: usize, column: usize) -> usize {
        line * self.width + column
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in 0..self.height {
            for column in 0..self.width {
                write!(f, "{}", self.grid[self.offset(line, column)])?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
