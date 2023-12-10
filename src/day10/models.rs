use colored::Colorize;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Tile {
    Ground,
    Start,
    Vertical,
    Horizontal,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Ground => ".",
                Tile::Start => "S",
                Tile::Vertical => "│",
                Tile::Horizontal => "─",
                Tile::TopLeft => "┌",
                Tile::TopRight => "┐",
                Tile::BottomLeft => "└",
                Tile::BottomRight => "┘",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
    pub start: Point,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid:")?;
        for line in &self.tiles {
            for cell in line {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Start: {}", self.start)
    }
}

impl Grid {
    #[allow(dead_code)]
    pub fn display_loop(&self, pipe_loop: &PipeLoop) {
        for (i_line, line) in self.tiles.iter().enumerate() {
            for (i_column, tile) in line.iter().enumerate() {
                let position = Point::new(i_line, i_column);
                if position == self.start {
                    print!("{}", pipe_loop.start_tile.to_string().red().bold());
                } else if pipe_loop.loop_elements.contains(&position) {
                    print!("{}", tile.to_string().green())
                } else {
                    print!("{}", tile);
                }
            }
            println!()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point {
    pub line: usize,
    pub column: usize,
}

impl Point {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PipeLoop {
    pub start_tile: Tile,
    pub loop_elements: HashSet<Point>,
    pub size: usize,
}
