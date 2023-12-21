use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Map {
    pub grid: Vec<Vec<bool>>, // true if their is a rock, false if clear
    pub start: (usize, usize),
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(grid: Vec<Vec<bool>>, start: (usize, usize)) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        Self {
            grid,
            start,
            width,
            height,
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i_line, line) in self.grid.iter().enumerate() {
            for (i_column, tile) in line.iter().enumerate() {
                if self.start.0 == i_line && self.start.1 == i_column {
                    write!(f, "S")?;
                } else if *tile {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        write!(f, "Start!: {:?}", &self.start)
    }
}
