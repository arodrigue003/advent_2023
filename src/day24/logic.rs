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

#[derive(Debug, Eq, PartialEq, Clone)]
enum ASolution {
    Simple(i128),
    Period((i128, i128)),
}

fn get_a_for_x(hail: &Hail, b_x: i128) -> ASolution {
    // If b_x is one of the hail velocity, a is forced
    for hailstone in &hail.hailstones {
        if b_x == hailstone.v.x {
            return ASolution::Simple(hailstone.p0.x);
        }
    }

    let periods: Vec<_> = hail
        .hailstones
        .iter()
        .map(|hailstone| solve_periodicity(hailstone.p0.x, hailstone.v.x - b_x, 0, 1).unwrap())
        .collect();

    // Merge them into one
    ASolution::Period(
        periods
            .iter()
            .fold((0, 1), |(acc_offset, acc_period), (offset, period)| {
                solve_periodicity(acc_offset, acc_period, *offset, *period).unwrap()
            }),
    )
}

fn get_a_for_y(hail: &Hail, b_y: i128) -> ASolution {
    // If b_x is one of the hail velocity, a is forced
    for hailstone in &hail.hailstones {
        if b_y == hailstone.v.y {
            return ASolution::Simple(hailstone.p0.y);
        }
    }

    let periods: Vec<_> = hail
        .hailstones
        .iter()
        .map(|hailstone| solve_periodicity(hailstone.p0.y, hailstone.v.y - b_y, 0, 1).unwrap())
        .collect();

    // Merge them into one
    ASolution::Period(
        periods
            .iter()
            .fold((0, 1), |(acc_offset, acc_period), (offset, period)| {
                solve_periodicity(acc_offset, acc_period, *offset, *period).unwrap()
            }),
    )
}

fn get_a_for_z(hail: &Hail, b_z: i128) -> ASolution {
    // If b_x is one of the hail velocity, a is forced
    for hailstone in &hail.hailstones {
        if b_z == hailstone.v.z {
            return ASolution::Simple(hailstone.p0.z);
        }
    }

    let periods: Vec<_> = hail
        .hailstones
        .iter()
        .map(|hailstone| solve_periodicity(hailstone.p0.z, hailstone.v.z - b_z, 0, 1).unwrap())
        .collect();

    // Merge them into one
    ASolution::Period(
        periods
            .iter()
            .fold((0, 1), |(acc_offset, acc_period), (offset, period)| {
                solve_periodicity(acc_offset, acc_period, *offset, *period).unwrap()
            }),
    )
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

/// Implementation of the extended euclidean algorithm
/// This is taken from https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
/// return the tuple (left_coef, right_coef, gcd)
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    loop {
        if r == 0 {
            break;
        }
        let quotient = old_r / r;

        let prov = r;
        r = old_r - quotient * r;
        old_r = prov;

        let prov = s;
        s = old_s - quotient * s;
        old_s = prov;

        let prov = t;
        t = old_t - quotient * t;
        old_t = prov
    }

    if old_r < 0 {
        (-old_s, -old_t, -old_r)
    } else {
        (old_s, old_t, old_r)
    }
}

/// Find the smallest integer solutions of `a + bx = c + dy`
/// If a solution is found return the offset and periodicity (e, f) that allows us to generate
/// every value where this equality has integer solution using the formula `en + f` where n is
/// a positive integer.
///
/// * In order to solve the equation we transform it to a linear diophantine equation.
///
/// * We transform the equation to an equation with the form `ex - fy = g`.
///
/// * We can now find two values `x0` and `y0` that satisfy this equation if `g` is a multiple of
/// `gcd(e, -f)`. To do that we use the extended gcd algorithm and we get two coeff verifying:
/// `left_coeff*e - right_coeff*f = gcd(e, -f)`.
///
/// * We now can compute `x0` and `y0` to have a solution for `ex - fy = g` by the operation `g / gcd`
/// This work because `g` is a multiple of the `gcd`, as well as `e` and `-f`.
/// We know have `x0 = left_coef * g / gcd` and `y0 = right_coef * g / gcd` satisfying
/// `a + bx = c + dy`
///
/// * Now we know that every solution has the form `x0 - fn / gcd` and `y0 - en / gcn` for every
/// integer n. We want to find the maximal value of n where x and y are positive. This will allow
/// us to find the minimal offset at witch both side of the initial equation meats up.
/// To do that we have to find the maximal value of n for which `n <= x0 * gcd / f` and  
/// `n <= y0 * gcd / e`. We note it `max_n`.
///
/// * Finally, we can determinate the offset by evaluating `a + bx = a + b(x0 - fn / gcd)` for
/// `n = max_n`.
/// * The periodicity is equal to the coefficient in front of `x`, that is equal to
/// `b * f / gcd)`.
fn solve_periodicity(a: i128, b: i128, c: i128, d: i128) -> Option<(i128, i128)> {
    // Transform it to the regular form of a linear diophantine equation (ex - fy = g)
    let e = b;
    let f = d;
    let g = c - a;

    // Compute the extended gcd of e and -f
    let (left_coef, right_coef, gcd) = extended_gcd(e, -f);

    if g % gcd != 0 {
        // println!("no solution because of the gcd");
        return None;
    }

    // Find a first solution thanks to the bezout coefficients
    // Here we know that x0 * e - y0 * f = g
    let x0 = left_coef * g / gcd;
    let y0 = right_coef * g / gcd;

    // Now we know that every solution has the form x0 - fn / gcd and y0 - en / gcn for every
    // integer n.
    // We now need to find the smallest positive solution to theses equations
    // to do that we need to find the first integer value for which of theses solutions is
    // positive
    let max_n = (x0 * gcd).div_euclid(f).min((y0 * gcd).div_euclid(e));

    // Offset is the application of the minimal solution we found for a + bx
    let offset = a + b * (x0 - f * max_n / gcd);

    // periodicity is equal to b * f / gcd (b times n coef)
    let periodicity = b * f / gcd;

    // println!(
    //     "a={}, b={}, c={}, d={}, e={}, f={}, g={}, left_coef={}, right_coef={}, gcd={}, \
    //     x0={}, y0={}, max_n={}, offset={}, periodicity={}, check1={}, check2={}",
    //     a,
    //     b,
    //     c,
    //     d,
    //     e,
    //     f,
    //     g,
    //     left_coef,
    //     right_coef,
    //     gcd,
    //     x0,
    //     y0,
    //     max_n,
    //     offset,
    //     periodicity,
    //     left_coef * e - right_coef * f,
    //     x0 * e - y0 * f
    // );

    Some((offset, periodicity))
}

pub fn solve_part_two(hail: &Hail) -> usize {
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
    let a_x = get_a_for_x(&hail, b_x);

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
    let a_y = get_a_for_y(&hail, b_y);

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
    let a_z = get_a_for_z(&hail, b_z);

    println!("Velocity: ({}, {}, {})", b_x, b_y, b_z);
    println!("Start: ({:?}, {:?}, {:?})", a_x, a_y, a_z);

    // println!(
    //     "{:?}",
    //     factorize64((339598398806870i64 - 325248475515080i64).abs() as u64)
    // );

    // Find the prime factors of the value
    // let res = factorize64(18u64.abs_diff(12u64));
    // let res = factorize64((339598398806870i64 - 325248475515080i64).abs() as u64);
    //
    // // Compute every divisor of the value
    // let mut factors = vec![1];
    // for (prime_factor, count) in res.into_iter() {
    //     // Update factors vec by choosing how many times we multiply by factor
    //     factors = factors
    //         .into_iter()
    //         .flat_map(|factor| (0..=count as u32).map(move |i| factor * prime_factor.pow(i)))
    //         .collect();
    // }
    // let factors: HashSet<_> = factors.into_iter().collect();
    //
    // // println!("{:?}", factors);
    //
    // println!("{:?}", get_possibles_b(18, 12, -1));
    // println!("{:?}", get_possibles_b(339598398806870, 325248475515080, -95));
    //
    // factors.len()

    0
}
