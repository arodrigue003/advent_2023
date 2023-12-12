#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConditionRecord {
    pub spring_status: Vec<SpringStatus>,
    pub spring_groups: Vec<usize>,
}
