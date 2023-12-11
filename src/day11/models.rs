use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Tile {
    Empty,
    Galaxy,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Empty => write!(f, "."),
            Tile::Galaxy => write!(f, "#"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SkyMap {
    pub grid: Vec<Vec<Tile>>,
}

impl Display for SkyMap {
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Coordinates {
    pub line: usize,
    pub column: usize,
}

impl Coordinates {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.line, self.column)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SparseSkyMap {
    pub galaxies: Vec<Coordinates>,

    // Theses values store the distance as follow: the number of empty lines between the line i
    // and the line j is equal to empty_lines_count[i][j]
    pub empty_lines_count: Vec<Vec<usize>>,
    pub empty_columns_count: Vec<Vec<usize>>,
}
