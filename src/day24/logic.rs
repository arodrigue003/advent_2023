use crate::day24::models::{Hail, Hailstone};

fn compute_intersection(h1: &Hailstone, h2: &Hailstone) -> Option<(i128, i128, i128)> {
    let left_num = h1.p0.y * h1.v.x - h1.p0.x * h1.v.y;
    let right_num = h2.p0.y * h2.v.x - h2.p0.x * h2.v.y;
    let x_numerator = h2.v.x * left_num - h1.v.x * right_num;
    let y_numerator = h2.v.y * left_num - h1.v.y * right_num;
    let denominator = h1.v.x * h2.v.y - h1.v.y * h2.v.x;
    if denominator == 0 {
        None
    } else if denominator > 0 {
        Some((x_numerator, y_numerator, denominator))
    } else {
        Some((-x_numerator, -y_numerator, -denominator))
    }
}

fn is_inside(x_num: i128, y_num: i128, den: i128, lower_bound: i128, upper_bound: i128) -> bool {
    x_num >= lower_bound * den && x_num <= upper_bound * den && y_num >= lower_bound * den && y_num <= upper_bound * den
}

fn is_in_the_future(h: &Hailstone, x_num: i128, den: i128) -> bool {
    if h.v.x > 0 {
        x_num > h.p0.x * den
    } else {
        x_num < h.p0.x * den
    }
}

fn does_intersect_inside(h1: &Hailstone, h2: &Hailstone, lower_bound: i128, upper_bound: i128) -> bool {
    if let Some((x_num, y_num, den)) = compute_intersection(h1, h2) {
        is_inside(x_num, y_num, den, lower_bound, upper_bound)
            && is_in_the_future(h1, x_num, den)
            && is_in_the_future(h2, x_num, den)
    } else {
        false
    }
}

pub fn solve_part_one(hail: &Hail, lower_bound: i128, upper_bound: i128) -> usize {
    (0..hail.hailstones.len())
        .flat_map(|i| {
            (i + 1..hail.hailstones.len()).filter(move |j| {
                does_intersect_inside(&hail.hailstones[i], &hail.hailstones[*j], lower_bound, upper_bound)
            })
        })
        .count()
}

pub fn solve_part_two(hail: &Hail) -> u32 {
    0
}
