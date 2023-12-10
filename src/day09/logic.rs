use crate::day09::models::SensorReport;
use itertools::Itertools;

fn generate_next_line(line: &[i64]) -> Vec<i64> {
    line.iter()
        .tuple_windows()
        .map(|(left, right)| *right - *left)
        .collect()
}

pub fn solve_part_one(data: &SensorReport) -> i64 {
    let mut global_res = 0;
    for value_history in &data.values_history {
        let mut value_expansion = vec![value_history.clone()];

        loop {
            if value_expansion[value_expansion.len() - 1]
                .iter()
                .all(|value| *value == 0)
            {
                break;
            }
            value_expansion.push(generate_next_line(
                &value_expansion[value_expansion.len() - 1],
            ))
        }

        let mut res = 0;
        for i in (0..value_expansion.len()).rev() {
            res += value_expansion[i][value_expansion[i].len() - 1]
        }
        global_res += res;
    }
    global_res
}

pub fn solve_part_two(data: &SensorReport) -> i64 {
    let mut global_res = 0;
    for value_history in &data.values_history {
        let mut value_expansion = vec![value_history.clone()];

        loop {
            if value_expansion[value_expansion.len() - 1]
                .iter()
                .all(|value| *value == 0)
            {
                break;
            }
            value_expansion.push(generate_next_line(
                &value_expansion[value_expansion.len() - 1],
            ))
        }

        let mut res = 0;
        for i in (0..value_expansion.len()).rev() {
            res = value_expansion[i][0] - res;
        }
        global_res += res;
    }
    global_res
}
