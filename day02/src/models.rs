use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct GameSubset {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl GameSubset {
    pub fn red(value: u32) -> Self {
        Self {
            red: value,
            green: 0,
            blue: 0,
        }
    }

    pub fn green(value: u32) -> Self {
        Self {
            red: 0,
            green: value,
            blue: 0,
        }
    }

    pub fn blue(value: u32) -> Self {
        Self {
            red: 0,
            green: 0,
            blue: value,
        }
    }
}

impl Add for GameSubset {
    type Output = GameSubset;

    fn add(self, rhs: Self) -> Self::Output {
        GameSubset {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sum for GameSubset {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |a, b| a + b)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    pub index: u32,
    pub subsets: Vec<GameSubset>,
}
