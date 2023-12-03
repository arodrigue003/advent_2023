use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SchematicCell {
    Empty,
    Symbol(char),
    Part(u8),
}

impl Display for SchematicCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SchematicCell::Empty => write!(f, "."),
            SchematicCell::Symbol(s) => write!(f, "{}", s),
            SchematicCell::Part(p) => write!(f, "{}", p),
        }
    }
}

impl SchematicCell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    pub fn is_part(&self) -> bool {
        matches!(self, Self::Part(_))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EnginePart {
    pub value: u32,
    pub col_start: usize,
    pub col_end: usize,
    pub line: usize,
}

impl Display for EnginePart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:<4}: ({:<3},{:<3}) -> ({:<3}, {:<3})",
            self.value, self.line, self.col_start, self.line, self.col_end
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Schematic {
    pub grid: Vec<Vec<SchematicCell>>,
    pub width: usize,
    pub height: usize,
    pub engine_parts: Vec<EnginePart>,
}

impl Display for Schematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid:")?;
        for line in &self.grid {
            for cell in line {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Engine parts:")?;
        for part in &self.engine_parts {
            writeln!(f, " * {}", &part)?;
        }

        Ok(())
    }
}
