#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Connection {
    pub name: String,
    pub others: Vec<String>,
}
