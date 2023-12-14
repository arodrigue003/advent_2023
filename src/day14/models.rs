use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub enum Tile {
    Empty,
    Round,
    Square,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".",
                Tile::Round => "O",
                Tile::Square => "#",
            }
        )
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Platform {
    pub grid: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Display for Platform {
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
