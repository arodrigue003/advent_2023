use crate::day24::models::{Hail, Hailstone};
use itertools::Itertools;
use num_prime::nt_funcs::factorize64;
use std::collections::HashSet;

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

// Find every possible value for two given hailstone that have the same velocity of B
// for the equations x0 + t0 * v = A + t0 * B, x1 + t1 * v = A + t1 * B
fn get_possibles_b(x0: i128, x1: i128, v: i128) -> HashSet<i128> {
    // Find the prime factors of abs(x0-x1)
    let prime_factors = factorize64(x0.abs_diff(x1) as u64);

    // Compute every divisor of the value
    let mut factors = vec![1];
    for (prime_factor, count) in prime_factors.into_iter() {
        // Update factors vec by choosing how many times we multiply by factor
        factors = factors
            .into_iter()
            .flat_map(|factor| (0..=count as u32).map(move |i| factor * prime_factor.pow(i)))
            .collect();
    }

    // Compute every possible b
    factors
        .into_iter()
        .flat_map(|factor| [v - factor as i128, v + factor as i128])
        .collect()
}

fn get_b_for_x(hail: &Hail) -> HashSet<i128> {
    let mut possible_res = HashSet::new();
    for (a, b) in hail.hailstones.iter().tuple_windows() {
        if a.v.x == b.v.x {
            let possibles_b = get_possibles_b(a.p0.x, b.p0.x, a.v.x);
            if possible_res.len() == 0 {
                // We didn't have possible values yet, set the set to this result
                possible_res = possibles_b;
            } else {
                // We had possible values, compute the intersection
                possible_res = possible_res.intersection(&possibles_b).cloned().collect();

                // The result must not be empty
                if possible_res.len() == 0 {
                    panic!("The problem does not have a solution")
                }

                // If the result only have one solution, we got b
                if possible_res.len() == 1 {
                    return possible_res;
                }
            }
        }
    }

    possible_res
}

fn get_b_for_y(hail: &Hail) -> HashSet<i128> {
    let mut possible_res = HashSet::new();
    for (a, b) in hail.hailstones.iter().tuple_windows() {
        if a.v.y == b.v.y {
            let possibles_b = get_possibles_b(a.p0.y, b.p0.y, a.v.y);
            if possible_res.len() == 0 {
                // We didn't have possible values yet, set the set to this result
                possible_res = possibles_b;
            } else {
                // We had possible values, compute the intersection
                possible_res = possible_res.intersection(&possibles_b).cloned().collect();

                // The result must not be empty
                if possible_res.len() == 0 {
                    panic!("The problem does not have a solution")
                }

                // If the result only have one solution, we got b
                if possible_res.len() == 1 {
                    return possible_res;
                }
            }
        }
    }

    possible_res
}

fn get_b_for_z(hail: &Hail) -> HashSet<i128> {
    let mut possible_res = HashSet::new();
    for (a, b) in hail.hailstones.iter().tuple_windows() {
        if a.v.z == b.v.z {
            let possibles_b = get_possibles_b(a.p0.z, b.p0.z, a.v.z);
            if possible_res.len() == 0 {
                // We didn't have possible values yet, set the set to this result
                possible_res = possibles_b;
            } else {
                // We had possible values, compute the intersection
                possible_res = possible_res.intersection(&possibles_b).cloned().collect();

                // The result must not be empty
                if possible_res.len() == 0 {
                    panic!("The problem does not have a solution")
                }

                // If the result only have one solution, we got b
                if possible_res.len() == 1 {
                    return possible_res;
                }
            }
        }
    }

    possible_res
}

pub fn solve_part_two(hail: &Hail) -> i128 {
    // Clone hail to be able to modify it
    let mut hail = hail.clone();

    // Sort it by vx
    hail.hailstones.sort_by_cached_key(|hailstone| hailstone.v.x);
    let b_x = get_b_for_x(&hail);
    let b_x = if b_x.len() == 1 {
        b_x.into_iter().next().unwrap()
    } else if b_x.len() == 0 {
        unreachable!()
    } else {
        // example
        unreachable!()
    };

    // Sort it by vy
    hail.hailstones.sort_by_cached_key(|hailstone| hailstone.v.y);
    let b_y = get_b_for_y(&hail);
    let b_y = if b_y.len() == 1 {
        b_y.into_iter().next().unwrap()
    } else if b_y.len() == 0 {
        unreachable!()
    } else {
        // example
        assert!(b_y.contains(&1));
        1
    };

    // Sort it by vz
    hail.hailstones.sort_by_cached_key(|hailstone| hailstone.v.z);
    let b_z = get_b_for_z(&hail);
    let b_z = if b_z.len() == 1 {
        b_z.into_iter().next().unwrap()
    } else if b_z.len() == 0 {
        unreachable!()
    } else {
        // example
        assert!(b_z.contains(&2));
        2
    };

    for hailstone in &hail.hailstones {
        if b_x == hailstone.v.x {
            let a_x = hailstone.p0.x;

            // Compute t for the first hailstone different from this one
            let target = hail.hailstones.iter().find(|h| h.v.x != b_x).unwrap();

            // Compute t for this hailstone
            let t = (a_x - target.p0.x) / (target.v.x - b_x);

            // compute a_y and a_z
            let a_y = target.p0.y + t * target.v.y - t * b_y;
            let a_z = target.p0.z + t * target.v.z - t * b_z;

            return a_x + a_y + a_z;
        }
    }
    for hailstone in &hail.hailstones {
        if b_y == hailstone.v.y {
            let a_y = hailstone.p0.y;

            // Compute t for the first hailstone different from this one
            let target = hail.hailstones.iter().find(|h| h.v.y != b_y).unwrap();

            // Compute t for this hailstone
            let t = (a_y - target.p0.y) / (target.v.y - b_y);

            // compute a_x and a_z
            let a_x = target.p0.x + t * target.v.x - t * b_x;
            let a_z = target.p0.z + t * target.v.z - t * b_z;

            return a_x + a_y + a_z;
        }
    }
    for hailstone in &hail.hailstones {
        if b_z == hailstone.v.z {
            let a_z = hailstone.p0.z;

            // Compute t for the first hailstone different from this one
            let target = hail.hailstones.iter().find(|h| h.v.z != b_z).unwrap();

            // Compute t for this hailstone
            let t = (a_z - target.p0.z) / (target.v.z - b_z);

            // compute a_x and a_z
            let a_x = target.p0.x + t * target.v.x - t * b_x;
            let a_y = target.p0.y + t * target.v.y - t * b_y;

            return a_x + a_y + a_z;
        }
    }

    unreachable!();
}
