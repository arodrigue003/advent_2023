use crate::day09::models::SensorReport;

pub fn parse_input(input: String) -> SensorReport {
    SensorReport {
        values_history: input
            .lines()
            .map(|line| {
                line.split(' ')
                    .map(|value| value.parse().unwrap())
                    .collect()
            })
            .collect(),
    }
}
