use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Action {
    Goto(String),
    Accepted,
    Rejected,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PartValue {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Test {
    Lower,
    Greater,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Part {
    pub x: i64,
    pub m: i64,
    pub a: i64,
    pub s: i64,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rule {
    // Test
    pub part_value: PartValue,
    pub test: Test,
    pub value: i64,

    // Action
    pub action: Action,

    // Helper func
    pub test_func: fn(&Part, i64) -> bool,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Workflow {
    pub name: String,
    pub rules: Vec<Rule>,
    pub default_action: Action,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct System {
    pub workflows: HashMap<String, Workflow>,
    pub parts: Vec<Part>,
}
