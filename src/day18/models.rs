use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
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
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Instruction {
    pub direction: Direction,
    pub distance: i32,
    pub color: Color,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} ({})", self.direction, self.distance, self.color)
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
