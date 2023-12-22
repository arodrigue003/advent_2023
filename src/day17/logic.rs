use std::collections::VecDeque;

use bucket_queue::{BucketQueue, FirstInFirstOutQueue};
use ndarray::{Array4, Axis};

use crate::day17::models::{Direction, Map};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct VisitNext {
    score: usize,
    line: usize,
    column: usize,
    direction: Direction,
    direction_steps: usize,
}

impl VisitNext {
    fn new(score: usize, line: usize, column: usize, direction: Direction, direction_steps: usize) -> Self {
        Self {
            score,
            line,
            column,
            direction,
            direction_steps,
        }
    }
}

/// Find the shortest path from start to finish.
/// The algorithm is a modified version of a Dijkstra algorithm.
fn find_shortest_path(map: &Map, min_distance: usize, max_distance: usize) -> usize {
    // Create structure to support the algorithm
    let mut visited = Array4::<bool>::default((map.height, map.width, 4, max_distance + 1));
    let mut scores = Array4::<usize>::from_elem((map.height, map.width, 4, max_distance + 1), usize::MAX);
    let mut visit_next = BucketQueue::<VecDeque<_>>::new();
    // let mut predecessor = HashMap::new();

    // Add the start node
    scores[[0, 0, 0, 0]] = 0;
    scores[[0, 0, 1, 0]] = 0;
    scores[[0, 0, 2, 0]] = 0;
    scores[[0, 0, 3, 0]] = 0;
    visit_next.enqueue(VisitNext::new(0, 0, 0, Direction::Right, 0), 0);
    visit_next.enqueue(VisitNext::new(0, 0, 0, Direction::Bot, 0), 0);

    while let Some(visit) = visit_next.dequeue_min() {
        let line = visit.line;
        let column = visit.column;

        // If the node was already visited, we don't have to do it again
        if visited[[line, column, visit.direction as usize, visit.direction_steps]] {
            continue;
        }

        // Up
        if line > 0 {
            visit_neighbor(
                map,
                min_distance,
                max_distance,
                &mut visited,
                &mut scores,
                &mut visit_next,
                &visit,
                line - 1,
                column,
                Direction::Up,
                Direction::Bot,
            );
        }

        // Left
        if column > 0 {
            visit_neighbor(
                map,
                min_distance,
                max_distance,
                &mut visited,
                &mut scores,
                &mut visit_next,
                &visit,
                line,
                column - 1,
                Direction::Left,
                Direction::Right,
            );
        }

        // Bot
        if line < map.height - 1 {
            visit_neighbor(
                map,
                min_distance,
                max_distance,
                &mut visited,
                &mut scores,
                &mut visit_next,
                &visit,
                line + 1,
                column,
                Direction::Bot,
                Direction::Up,
            );
        }

        // Right
        if column < map.width - 1 {
            visit_neighbor(
                map,
                min_distance,
                max_distance,
                &mut visited,
                &mut scores,
                &mut visit_next,
                &visit,
                line,
                column + 1,
                Direction::Right,
                Direction::Left,
            );
        }

        visited[[line, column, visit.direction as usize, visit.direction_steps]] = true
    }

    // // Get the winning path
    // let winning = scores
    //     .index_axis(Axis(0), map.height - 1)
    //     .index_axis(Axis(0), map.width - 1)
    //     .indexed_iter()
    //     .filter_map(|((direction, direction_steps), score)| {
    //         if direction_steps >= min_distance {
    //             Some((*score, direction, direction_steps))
    //         } else {
    //             None
    //         }
    //     })
    //     .min()
    //     .unwrap();
    //
    // // Reconstruct the path
    // let mut path = vec![];
    // let mut node = (map.height - 1, map.width - 1, winning.1, winning.2);
    // loop {
    //     // Add the node to the path
    //     path.push(node);
    //
    //     // If we are at the start, break
    //     if node.0 == 0 && node.1 == 0 {
    //         break;
    //     }
    //
    //     // Get the predecessor
    //     node = predecessor[&node];
    // }
    // path.reverse();
    //
    // map.display_path(&path);
    //
    // winning.0

    scores
        .index_axis(Axis(0), map.height - 1)
        .index_axis(Axis(0), map.width - 1)
        .indexed_iter()
        .filter_map(|((_, direction_steps), score)| {
            if direction_steps >= min_distance {
                Some(*score)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
fn visit_neighbor(
    map: &Map,
    min_distance: usize,
    max_distance: usize,
    visited: &mut Array4<bool>,
    scores: &mut Array4<usize>,
    visit_next: &mut BucketQueue<VecDeque<VisitNext>>,
    visit: &VisitNext,
    new_line: usize,
    new_colum: usize,
    new_direction: Direction,
    forbidden_direction: Direction,
) {
    if visit.direction != forbidden_direction
        && ((visit.direction != new_direction && visit.direction_steps >= min_distance)
            || (visit.direction == new_direction && visit.direction_steps < max_distance))
    {
        let direction_steps = if visit.direction == new_direction {
            visit.direction_steps + 1
        } else {
            1
        };

        if !visited[[new_line, new_colum, new_direction as usize, direction_steps]] {
            let next_score = visit.score + map.grid[new_line][new_colum] as usize;
            if next_score < scores[[new_line, new_colum, new_direction as usize, direction_steps]] {
                scores[[new_line, new_colum, new_direction as usize, direction_steps]] = next_score;
                visit_next.enqueue(
                    VisitNext::new(next_score, new_line, new_colum, new_direction, direction_steps),
                    next_score,
                );
                // predecessor.insert(
                //     (new_line, new_colum, new_direction as usize, direction_steps),
                //     (line, new_colum, visit.direction as usize, visit.direction_steps),
                // );
            }
        }
    }
}

pub fn solve_part_one(map: &Map) -> usize {
    find_shortest_path(map, 1, 3)
}

pub fn solve_part_two(map: &Map) -> usize {
    find_shortest_path(map, 4, 10)
}
