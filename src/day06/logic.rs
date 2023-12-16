use crate::day06::models::Races;

// Check if value has an integer square root and returns it.
fn perfect_square_root(value: i64) -> Option<i64> {
    let res = ((value as f64).sqrt() + 0.5) as i64;
    if res * res == value {
        Some(res)
    } else {
        None
    }
}

// Compute the minimal value for which we would go further than the reference.
// This correspond to the lower integer root of the inequality x(t-x)>=d
fn compute_lower(time: i64, distance: i64) -> i64 {
    let delta = time * time - 4 * distance;

    match perfect_square_root(delta) {
        None => ((time as f64 - (delta as f64).sqrt()) / 2f64).ceil() as i64,
        Some(psr) => (time - psr) / 2 + 1,
    }
}

// Compute the maximum value for which we would go further than the reference.
// This correspond to the upper integer root of the inequality x(t-x)>=d
fn compute_higher(time: i64, distance: i64) -> i64 {
    let delta = time * time - 4 * distance;

    match perfect_square_root(delta) {
        None => ((time as f64 + (delta as f64).sqrt()) / 2f64).floor() as i64,
        Some(psr) => (time + psr - 1) / 2,
    }
}

pub fn solve_part_one(data: &Races) -> i64 {
    data.races
        .iter()
        .map(|race| compute_higher(race.time, race.distance) - compute_lower(race.time, race.distance) + 1)
        .product()
}

pub fn solve_part_two(data: &Races) -> i64 {
    let time = data
        .races
        .iter()
        .fold(0, |acc, race| acc * 10i64.pow(race.time.ilog10() + 1) + race.time);
    let distance = data.races.iter().fold(0, |acc, race| {
        acc * 10i64.pow(race.distance.ilog10() + 1) + race.distance
    });

    compute_higher(time, distance) - compute_lower(time, distance) + 1
}
