#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    pub lines: Vec<u64>,
    pub columns: Vec<u64>,
    pub width: usize,
    pub height: usize,
}
