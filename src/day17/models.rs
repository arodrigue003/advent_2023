use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

use colored::Colorize;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Bot = 2,
    Left = 3,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Map {
    pub grid: Vec<Vec<u32>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(grid: Vec<Vec<u32>>) -> Self {
        let width = grid[0].len();
        let height = grid.len();

        Self { grid, width, height }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for block in line {
                write!(f, "{}", block)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    #[allow(dead_code)]
    pub fn display_path(&self, path: &[(usize, usize, usize, usize)]) {
        // Convert the path to a hashset
        let path_set: HashSet<_> = path.iter().map(|(line, column, _, _)| (*line, *column)).collect();

        for line in 0..self.height {
            for column in 0..self.width {
                if path_set.contains(&(line, column)) {
                    print!("{}", self.grid[line][column].to_string().green());
                } else {
                    print!("{}", self.grid[line][column]);
                }
            }
            println!();
        }
    }

    #[allow(dead_code)]
    pub fn display_predecessor(&self, path: &[(usize, usize)], predecessor: &HashMap<(usize, usize), (usize, usize)>) {
        // Convert the path to a hashset
        let path_set: HashSet<_> = path.iter().cloned().collect();

        for line in 0..self.height {
            for column in 0..self.width {
                let pred = predecessor.get(&(line, column));

                if let Some(pred) = pred {
                    let to_display = if pred.0 + 1 == line {
                        "v"
                    } else if pred.1 + 1 == column {
                        ">"
                    } else if pred.0 == line + 1 {
                        "^"
                    } else if pred.1 == column + 1 {
                        "<"
                    } else {
                        unreachable!()
                    };
                    if path_set.contains(&(line, column)) {
                        print!("{}", to_display.green());
                    } else {
                        print!("{}", to_display);
                    }
                } else {
                    print!("{}", self.grid[line][column]);
                }
            }
            println!();
        }
    }
}
