use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd)]
pub struct Mapping {
    pub src_start: i64,
    pub dst_start: i64,
    pub size: i64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}

impl Display for Mapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}..{})->({}..{})",
            self.src_start,
            self.src_start + self.size,
            self.dst_start,
            self.dst_start + self.size
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Almanac {
    pub seeds: Vec<i64>,
    // mappings is sorted
    pub mappings: Vec<Vec<Mapping>>,
}
