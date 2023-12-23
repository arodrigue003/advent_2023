use std::collections::VecDeque;

use bucket_queue::{BucketQueue, FirstInFirstOutQueue};
use ndarray::Array2;

use crate::day21::models::Map;

fn count_accessible_tiles(map: &Map, start: (usize, usize), steps: usize, parity: usize) -> usize {
    if steps == 0 {
        return 0;
    }

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

        visited[[line, column]] = true;
    }

    // The result is equal to the number of distance that has the same remainder as steps by 2 as steps.
    scores
        .iter()
        .filter(|score| **score != usize::MAX && **score % 2 == parity)
        .count()
}

pub fn solve_part_one(map: &Map) -> usize {
    count_accessible_tiles(map, map.start, 64, 0)
}

#[allow(dead_code)]
fn solve_part_two_bruteforce(map: &Map, start: (usize, usize), steps: usize) -> usize {
    // Compute how many maps we will need for the bruteforce
    let map_count = 2 * (steps / map.width) + 1;

    // Build a map that is a concatenation of every map
    let mut grid = vec![vec![false; map.width * map_count]; map.width * map_count];

    for map_line in 0..map_count {
        for map_column in 0..map_count {
            for line in 0..map.height {
                for column in 0..map.width {
                    grid[map.height * map_line + line][map.width * map_column + column] = map.grid[line][column];
                }
            }
        }
    }

    let start = (
        map_count / 2 * map.height + start.0,
        map_count / 2 * map.width + start.1,
    );
    let new_map = Map::new(grid, start);
    // println!("{new_map}");

    count_accessible_tiles(&new_map, start, steps, steps % 2)
}

pub fn solve_part_two(map: &Map) -> usize {
    let steps = 26501365;
    let parity = steps % 2;

    // Verify assertions
    if map.width != map.height {
        panic!("Map must be a square!")
    }
    if map.width % 2 == 0 {
        panic!("Map width must be odd!")
    }
    if map.start.0 != map.height / 2 || map.start.0 != map.width / 2 {
        panic!("Start must be at the center!")
    }
    if (0..map.width).any(|column| map.grid[0][column]) {
        panic!("First line must be empty!")
    }
    if (0..map.width).any(|column| map.grid[map.height / 2][column]) {
        panic!("Middle line must be empty!")
    }
    if (0..map.width).any(|column| map.grid[map.height - 1][column]) {
        panic!("Last line must be empty!")
    }
    if (0..map.height).any(|line| map.grid[line][0]) {
        panic!("First column must be empty!")
    }
    if (0..map.height).any(|line| map.grid[line][map.width / 2]) {
        panic!("Middle column must be empty!")
    }
    if (0..map.height).any(|line| map.grid[line][map.width - 1]) {
        panic!("Last column must be empty!")
    }

    // Compute the number of full square that we will get on the main line / column
    let main_line_length = steps / map.width;

    // Compute the number of positions for a full map
    let full_map_positions_count_base_par = count_accessible_tiles(map, map.start, map.width, parity);
    let full_map_positions_count_other_par = count_accessible_tiles(map, map.start, map.width, (parity + 1) % 2);

    // Compute how many steps will be left on the left map
    let offset_at_line_end = steps - main_line_length * map.width + map.width / 2;

    // Compute the number of position for the left, top, right and bottom map
    let left_map_position_count = count_accessible_tiles(
        map,
        (map.start.0, map.width - 1),
        offset_at_line_end,
        (parity + main_line_length + 1) % 2,
    ) + count_accessible_tiles(
        map,
        (map.start.0, map.width - 1),
        offset_at_line_end.saturating_sub(map.width),
        (parity + main_line_length) % 2,
    );
    let top_map_position_count = count_accessible_tiles(
        map,
        (map.height - 1, map.start.1),
        offset_at_line_end,
        (parity + main_line_length + 1) % 2,
    ) + count_accessible_tiles(
        map,
        (map.height - 1, map.start.1),
        offset_at_line_end.saturating_sub(map.width),
        (parity + main_line_length) % 2,
    );
    let right_map_position_count = count_accessible_tiles(
        map,
        (map.start.0, 0),
        offset_at_line_end,
        (parity + main_line_length + 1) % 2,
    ) + count_accessible_tiles(
        map,
        (map.start.0, 0),
        offset_at_line_end.saturating_sub(map.width),
        (parity + main_line_length) % 2,
    );
    let bottom_map_position_count = count_accessible_tiles(
        map,
        (0, map.start.1),
        offset_at_line_end,
        (parity + main_line_length + 1) % 2,
    ) + count_accessible_tiles(
        map,
        (0, map.start.1),
        offset_at_line_end.saturating_sub(map.width),
        (parity + main_line_length) % 2,
    );

    // Compute how many will be left on the diagonal corners
    let offset_at_diagonal_end = steps - (main_line_length - 1) * map.width - 1;

    // Compute diagonal res
    let upper_left_map_position_count_1 = count_accessible_tiles(
        map,
        (map.height - 1, map.height - 1),
        offset_at_diagonal_end,
        (parity + main_line_length) % 2,
    );
    let upper_left_map_position_count_2 = count_accessible_tiles(
        map,
        (map.height - 1, map.height - 1),
        offset_at_diagonal_end.saturating_sub(map.width),
        (parity + main_line_length + 1) % 2,
    );
    let upper_right_map_position_count_1 = count_accessible_tiles(
        map,
        (map.height - 1, 0),
        offset_at_diagonal_end,
        (parity + main_line_length) % 2,
    );
    let upper_right_map_position_count_2 = count_accessible_tiles(
        map,
        (map.height - 1, 0),
        offset_at_diagonal_end.saturating_sub(map.width),
        (parity + main_line_length + 1) % 2,
    );
    let lower_right_map_position_count_1 =
        count_accessible_tiles(map, (0, 0), offset_at_diagonal_end, (parity + main_line_length) % 2);
    let lower_right_map_position_count_2 = count_accessible_tiles(
        map,
        (0, 0),
        offset_at_diagonal_end.saturating_sub(map.width),
        (parity + main_line_length + 1) % 2,
    );
    let lower_left_map_position_count_1 = count_accessible_tiles(
        map,
        (0, map.height - 1),
        offset_at_diagonal_end,
        (parity + main_line_length) % 2,
    );
    let lower_left_map_position_count_2 = count_accessible_tiles(
        map,
        (0, map.height - 1),
        offset_at_diagonal_end.saturating_sub(map.width),
        (parity + main_line_length + 1) % 2,
    );

    // Compute how many full grid we have
    let full_grid_count_base_par = (main_line_length - (main_line_length - 1) % 2).pow(2);
    let full_grid_count_other_par = (main_line_length - (main_line_length) % 2).pow(2);
    let inner_diagonal_count = main_line_length - 1;
    let out_diagonal_count = main_line_length;

    // Sum it up

    full_map_positions_count_base_par * full_grid_count_base_par
        + full_map_positions_count_other_par * full_grid_count_other_par
        + left_map_position_count
        + right_map_position_count
        + top_map_position_count
        + bottom_map_position_count
        + inner_diagonal_count
            * (upper_left_map_position_count_1
                + upper_right_map_position_count_1
                + lower_left_map_position_count_1
                + lower_right_map_position_count_1)
        + out_diagonal_count
            * (upper_left_map_position_count_2
                + upper_right_map_position_count_2
                + lower_left_map_position_count_2
                + lower_right_map_position_count_2)
}
