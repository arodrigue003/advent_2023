use crate::day05::models::{Almanac, Mapping, Range};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn map_range_inner(range: Range, mappings: &[Mapping]) -> Vec<Range> {
    let mut res = vec![];
    let mut current = range.start;

    for mapping in mappings {
        // The mapping started before and is still ongoing
        if mapping.src_start <= current && current < mapping.src_start + mapping.size {
            let end = range.end.min(mapping.src_start + mapping.size);
            res.push(Range {
                start: current + mapping.dst_start - mapping.src_start,
                end: end + mapping.dst_start - mapping.src_start,
            });
            // println!("1 {:?}", res[res.len() - 1]);
            current = end;
        }

        // The mapping is after the end of the range, return early
        if range.end < mapping.src_start {
            break;
        }

        // If mapping is between current and range.end, transform the important part
        if current < mapping.src_start {
            // here we have current < mapping.scr_start <= range.end

            // First add the unmaped part to the res
            res.push(Range {
                start: current,
                end: mapping.src_start,
            });
            // println!("3 {:?}", res[res.len() - 1]);

            // Compute the right side of the mapping
            current = range.end.min(mapping.src_start + mapping.size);

            // Add the mapped range to the result
            res.push(Range {
                start: mapping.dst_start,
                end: current + mapping.dst_start - mapping.src_start,
            });
            // println!("4 {:?}", res[res.len() - 1]);
        }
    }

    // Add whats remaining if required
    if current != range.end {
        res.push(Range {
            start: current,
            end: range.end,
        });
        // println!("2 {:?}", res[res.len() - 1]);
    }

    res
}

fn map_range(range: Range, almanac: &Almanac) -> Vec<Range> {
    almanac.mappings.iter().fold(vec![range], |acc, mapping| {
        acc.into_iter()
            .flat_map(|range_inner| map_range_inner(range_inner, mapping))
            .collect()
    })
}

// We make the hypothesis that mappings form a disjoint set of mapping.
fn map_value(value: i64, almanac: &Almanac) -> i64 {
    almanac.mappings.iter().fold(value, |acc, mapping| {
        mapping
            .iter()
            .fold_while(acc, |acc2, mapping| {
                if acc2 >= mapping.src_start && acc2 < mapping.src_start + mapping.size {
                    Done(acc2 + mapping.dst_start - mapping.src_start)
                } else {
                    Continue(acc2)
                }
            })
            .into_inner()
    })
}

pub fn solve_part_one(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .map(|seed| map_value(*seed, almanac))
        .min()
        .unwrap()
}

pub fn solve_part_two(almanac: &Almanac) -> i64 {
    almanac
        .seeds
        .iter()
        .tuples()
        .flat_map(|(start, size)| {
            map_range(
                Range {
                    start: *start,
                    end: *start + *size,
                },
                almanac,
            )
        })
        .map(|range| range.start)
        .min()
        .unwrap()
}
