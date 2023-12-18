use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "U",
                Direction::Right => "R",
                Direction::Down => "D",
                Direction::Left => "L",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    pub direction: Direction,
    pub distance: i64,
    // pub color: Color,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.direction, self.distance)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DigPlan {
    pub instructions: Vec<Instruction>,
}

impl Display for DigPlan {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Instructions:")?;

        for instruction in &self.instructions {
            writeln!(f, "{}", instruction)?;
        }

        Ok(())
    }
}
