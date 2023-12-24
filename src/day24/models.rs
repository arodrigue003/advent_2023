#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: i128,
    pub y: i128,
    pub z: i128,
}

impl Point {
    pub fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Hailstone {
    pub p0: Point,
    pub v: Point,
}

impl Hailstone {
    pub fn new(x: i128, y: i128, z: i128, vx: i128, vy: i128, vz: i128) -> Self {
        Self {
            p0: Point::new(x, y, z),
            v: Point::new(vx, vy, vz),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Hail {
    pub hailstones: Vec<Hailstone>,
}
