use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Action {
    Remove,
    Add(usize),
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Remove => write!(f, "-"),
            Action::Add(value) => write!(f, "={}", value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Step {
    pub label: String,
    pub action: Action,
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.label, self.action)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BoxLens {
    pub label: String,
    pub focal: usize,
}

impl Display for BoxLens {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.label, self.focal)
    }
}
