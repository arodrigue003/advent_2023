use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    pub index: usize,
    pub winning: HashSet<u8>,
    pub draw: HashSet<u8>,
}
