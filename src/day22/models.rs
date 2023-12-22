use std::fmt::{Display, Formatter};
use std::mem::swap;
use std::ops::{Add, Sub};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum Direction {
    X,
    Y,
    Z,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl Point {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    #[allow(dead_code)]
    pub fn x(&self) -> usize {
        self.x
    }

    #[allow(dead_code)]
    pub fn y(&self) -> usize {
        self.y
    }

    #[allow(dead_code)]
    pub fn z(&self) -> usize {
        self.z
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BrickCoordinates(pub Vec<Point>);

impl Display for BrickCoordinates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, brick) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", brick)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Brick {
    pub start_offset: Point,
    pub offsets: Vec<Point>,
    pub end: Point,
    pub direction: Direction,
}

impl Brick {
    pub fn new(mut start: Point, mut end: Point) -> Self {
        // be sure that start is before the end
        if start > end {
            swap(&mut start, &mut end);
        }

        // normalize the start and end coordinate
        let start_offset = start;
        let end = end - start_offset;

        let (offsets, direction) = if end.x != 0 {
            (
                (0..=end.x).map(|offset| Point::new(offset, 0, 0)).collect(),
                Direction::X,
            )
        } else if end.y != 0 {
            (
                (0..=end.y).map(|offset| Point::new(0, offset, 0)).collect(),
                Direction::Y,
            )
        } else {
            (
                (0..=end.z).map(|offset| Point::new(0, 0, offset)).collect(),
                Direction::Z,
            )
        };

        Self {
            start_offset,
            offsets,
            end,
            direction,
        }
    }

    pub fn end(&self) -> Point {
        self.end + self.start_offset
    }

    pub fn coordinates(&self) -> BrickCoordinates {
        BrickCoordinates(self.offsets.iter().map(|offset| self.start_offset + *offset).collect())
    }

    pub fn fall(&mut self, z: usize) {
        self.start_offset.z -= z;
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} -> {} [{}]",
            self.start_offset,
            self.end + self.start_offset,
            self.coordinates()
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FallingBricks {
    pub bricks: Vec<Brick>,
}

impl Display for FallingBricks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Falling bricks:")?;
        for brick in &self.bricks {
            writeln!(f, "{}", brick)?
        }

        Ok(())
    }
}
