use crate::models::SensorReport;

pub fn parse_data(data: &str) -> SensorReport {
    SensorReport {
        values_history: data
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|value| value.parse().unwrap())
                    .collect()
            })
            .collect(),
    }
}
