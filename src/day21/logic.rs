use crate::day21::models::Map;
use bucket_queue::{BucketQueue, FirstInFirstOutQueue};
use ndarray::Array2;
use std::collections::VecDeque;

fn count_accessible_tiles(map: &Map, start: (usize, usize), steps: usize) -> usize {
    // Use the Dijkstra in order to determinate the distance we can travel in steps steps.

    // Create structures to support the algorithm
    let mut visited = Array2::<bool>::default((map.height, map.width));
    let mut scores = Array2::<usize>::from_elem((map.height, map.height), usize::MAX);
    let mut visit_next = BucketQueue::<VecDeque<_>>::new();

    // Add the start node
    visit_next.enqueue((0, start.0, start.1, steps), 0);

    // Main loop
    while let Some((score, line, column, remaining_steps)) = visit_next.dequeue_min() {
        // If the node was already visited, we don't have to do it again
        // If we don't have any more steps to use, skip it
        if visited[[line, column]] || remaining_steps == 0 {
            continue;
        }

        // Up
        if line > 0 && !map.grid[line - 1][column] && score + 1 < scores[[line - 1, column]] {
            scores[[line - 1, column]] = score + 1;
            visit_next.enqueue((score + 1, line - 1, column, remaining_steps - 1), score + 1);
        }
        // Left
        if column > 0 && !map.grid[line][column - 1] && score + 1 < scores[[line, column - 1]] {
            scores[[line, column - 1]] = score + 1;
            visit_next.enqueue((score + 1, line, column - 1, remaining_steps - 1), score + 1);
        }
        // Down
        if line < map.height - 1 && !map.grid[line + 1][column] && score + 1 < scores[[line + 1, column]] {
            scores[[line + 1, column]] = score + 1;
            visit_next.enqueue((score + 1, line + 1, column, remaining_steps - 1), score + 1);
        }
        // Right
        if column < map.width - 1 && !map.grid[line][column + 1] && score + 1 < scores[[line, column + 1]] {
            scores[[line, column + 1]] = score + 1;
            visit_next.enqueue((score + 1, line, column + 1, remaining_steps - 1), score + 1);
        }
    }

    // The result is equal to the number of distance that has the same remainder as steps by 2 as steps.
    let remainder = steps % 2;
    scores
        .iter()
        .filter(|score| **score != usize::MAX && **score % 2 == remainder)
        .count()
}

pub fn solve_part_one(map: &Map) -> usize {
    count_accessible_tiles(map, map.start, 64)
}

pub fn solve_part_two(map: &Map) -> u32 {
    0
}
