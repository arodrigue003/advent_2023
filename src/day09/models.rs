#[derive(Debug, PartialEq, Clone)]
pub struct SensorReport {
    pub values_history: Vec<Vec<i64>>,
}
