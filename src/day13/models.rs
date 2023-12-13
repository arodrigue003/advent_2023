#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    pub lines: Vec<u64>,
    pub columns: Vec<u64>,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Tile {
    Ash,
    Rock,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Grid2 {
    pub grid: Vec<Vec<Tile>>,
}
