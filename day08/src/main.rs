use gcd::Gcd;
use std::collections::HashMap;
use std::fs;

use advent_2023_common::get_args;
use day08::models::{Direction, NavigationMap};
use day08::parser::parse_data;

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

    (old_s, old_t, old_r)
}

/// Find integer solutions of a + bx = c + dy
/// If a solution is found return the offset and periodicity (e, f) that allows us to generate
/// every value where this equality has integer solution using the formula en + f where n is
/// a positive integer.
///
/// In order to solve the equation we transform it to a linear diophantine equation.
/// We transform the equation to an equation with the form ex - fy = g
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
    let max_n = (x0 * gcd.abs())
        .div_euclid(f)
        .min((y0 * gcd.abs()).div_euclid(e));

    // Offset is the application of the minimal solution we found for a + bx
    let offset = a + b * (x0 - f * max_n / gcd.abs());

    // periodicity is equal to b * f / gcd (b times n coef)
    let periodicity = b * f / gcd.abs();

    // println!(
    //     "a={}, b={}, c={}, d={}, e={}, f={}, g={}, left_coef={}, right_coef={}, gcd={}, gcd2={}, x0={}, y0={}, max_n={}, offset={}, periodicity={}, check={}",
    //     a,b,c,d,e,f,g, left_coef, right_coef, gcd, (e.abs() as u128).gcd(f.abs()as u128), x0, y0, max_n, offset, periodicity, x0 * e - y0 * f
    // );

    Some((offset, periodicity))
}

fn solve_part_one(data: &NavigationMap) -> u32 {
    // Detect the end position
    let end = data
        .nodes
        .iter()
        .position(|node| node.name.as_str() == "ZZZ")
        .unwrap();

    // At the beginning, the current position is at the start
    let mut current = data
        .nodes
        .iter()
        .position(|node| node.name.as_str() == "AAA")
        .unwrap();

    // Iterate over the instruction until we reach the end
    let mut steps = 0;
    for direction in data.instructions.iter().cycle() {
        current = match direction {
            Direction::Left => data.nodes[current].left,
            Direction::Right => data.nodes[current].right,
        };

        steps += 1;

        if current == end {
            break;
        }
    }

    steps
}

/// Compute every moment where the current value will be at the end.
/// Solution have the form vec((offset, periodicity)).
/// Every combination will be found at steps offset + periodicity * n where n is a positive integer
fn compute_periodicities(current: &usize, data: &&NavigationMap) -> Vec<(i128, i128)> {
    let mut current = *current;
    let mut to_check: HashMap<_, _> = HashMap::new();
    let mut current_pos = 0;
    let mut current_step: i128 = 0;
    for direction in data.instructions.iter().cycle() {
        // Make progress
        current = match direction {
            Direction::Left => data.nodes[current].left,
            Direction::Right => data.nodes[current].right,
        };

        // Update positions
        current_pos = (current_pos + 1) % data.instructions.len();
        current_step += 1;

        if data.nodes[current].name.ends_with("Z") {
            let new_check = (&data.nodes[current].name, current_pos);
            if let Some(last_step) = to_check.get(&new_check) {
                let periodicity = current_step - last_step;
                return to_check
                    .values()
                    .filter(|step| *step >= last_step)
                    .map(|step| (*step, periodicity))
                    .collect();
            }

            to_check.insert(new_check, current_step);
        }
    }

    unreachable!()
}

fn solve_part_two(data: &NavigationMap) -> i128 {
    // At the beginning, the current position is every nodes that ends with the letter a
    let mut currents: Vec<_> = data
        .nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node.name.ends_with("A"))
        .map(|(position, _)| position)
        .collect();

    // We need to search for a pattern
    let periodicities: Vec<_> = currents
        .iter()
        .map(|current| compute_periodicities(current, &data))
        .collect();

    periodicities
        .iter()
        .fold(vec![(0, 1)], |acc, periods| {
            acc.into_iter()
                .flat_map(|(acc_offset, acc_period)| {
                    periods.iter().filter_map(move |(offset, period)| {
                        solve_periodicity(acc_offset, acc_period, *offset, *period)
                    })
                })
                .collect()
        })
        .into_iter()
        .map(|(offset, _)| offset)
        .min()
        .unwrap()
}

fn main() {
    let args = get_args();
    let data: String = fs::read_to_string(&args.path).unwrap();
    let parsed_data = parse_data(&data);

    if args.verbose {
        println!("{:#?}", &parsed_data);
    }

    println!("Part one solution: {}", solve_part_one(&parsed_data));
    println!("Part two solution: {}", solve_part_two(&parsed_data));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    static INPUT_EXAMPLE_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    static INPUT_EXAMPLE_2: &str = "LR

AAA = (11B, 11B)
11B = (11C, 11C)
11C = (11D, 11D)
11D = (11E, 11E)
11E = (ZZZ, ZZZ)
ZZZ = (11B, 11B)
22A = (22B, 22B)
22B = (22C, 22C)
22C = (22D, 22D)
22D = (22Z, 22Z)
22Z = (22A, 22B)
33A = (33B, 33B)
33B = (33C, 33C)
33C = (33Z, 33Z)
33Z = (33B, 33B)
";

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one(&parse_data(INPUT_EXAMPLE_1)), 6);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE_2)), 45);
    }
}
