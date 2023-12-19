use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Action {
    Goto(String),
    Accepted,
    Rejected,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Part {
    pub x: i32,
    pub m: i32,
    pub a: i32,
    pub s: i32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Rule {
    pub value: i32,
    pub action: Action,
    pub test: fn(&Part, i32) -> bool,
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
