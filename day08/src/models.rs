#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub name: String,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct NavigationMap {
    pub instructions: Vec<Direction>,
    pub nodes: Vec<Node>,
}
