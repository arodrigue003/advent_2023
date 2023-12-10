#[derive(Debug, PartialEq, Clone)]
pub struct Race {
    pub time: i64,
    pub distance: i64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Races {
    pub races: Vec<Race>,
}
