use crate::day22::models::{Brick, Direction, FallingBricks, Point};
use ndarray::Array2;

use petgraph::{Graph, Incoming, Outgoing};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn prepare_data(falling_bricks: &FallingBricks) -> Graph<usize, ()> {
    // Clone the data
    let mut falling_bricks = falling_bricks.clone();

    // Determinate the grid size
    let ends: Vec<_> = falling_bricks.bricks.iter().map(Brick::end).collect();
    let max_x = ends.iter().map(Point::x).max().unwrap();
    let max_y = ends.iter().map(Point::y).max().unwrap();

    // Create a graph that will contains the connection between bricks
    let mut graph = Graph::new();

    // Add the bricks to the graph as nodes
    let nodes: Vec<_> = (0..falling_bricks.bricks.len())
        .map(|brick_id| graph.add_node(brick_id))
        .collect();

    // Create an array that will store the max height of every column
    let mut height_map = Array2::<(usize, usize)>::from_elem((max_x + 1, max_y + 1), (0, usize::MAX));

    // Create a queue to store and retrieve bricks from bot to top
    let mut queue = BinaryHeap::new();

    // Add the bricks to the queue
    for (brick_id, brick) in falling_bricks.bricks.iter_mut().enumerate() {
        queue.push(Reverse((brick.start_offset.z, brick_id, brick)));
    }

    // While we still have bricks to treat
    while let Some(Reverse((_, brick_id, falling_brick))) = queue.pop() {
        // retrieve the brick coordinates
        let coordinates = falling_brick.coordinates();

        // Check the lowest z position that is available for the brick
        let max_z = coordinates
            .0
            .iter()
            .map(|point| height_map[[point.x, point.y]].0)
            .max()
            .unwrap();

        // Update the height_map with this value
        // We add the graph connection at the same time
        if falling_brick.direction == Direction::Z {
            // We know that their is something below this point because we chose it like that.
            if max_z > 0 {
                graph.add_edge(
                    nodes[brick_id],
                    nodes[height_map[[falling_brick.start_offset.x, falling_brick.start_offset.y]].1],
                    (),
                );
            }
            height_map[[coordinates.0[0].x, coordinates.0[0].y]] = (max_z + falling_brick.end.z + 1, brick_id);
        } else {
            for coordinate in &coordinates.0 {
                if max_z > 0 {
                    let (other_z, other_brick_id) = height_map[[coordinate.x, coordinate.y]];
                    if other_z == max_z
                        && other_brick_id != usize::MAX
                        && !graph.contains_edge(nodes[brick_id], nodes[other_brick_id])
                    {
                        graph.add_edge(nodes[brick_id], nodes[other_brick_id], ());
                    }
                }
                height_map[[coordinate.x, coordinate.y]] = (max_z + 1, brick_id);
            }
        }

        // Move the brick
        falling_brick.fall(falling_brick.start_offset.z - max_z);
    }

    graph
}

pub fn solve_part_one(graph: &Graph<usize, ()>) -> usize {
    // Iterate over nodes to determinate the safe ones
    graph
        .node_indices()
        .filter(|node| {
            graph
                .neighbors_directed(*node, Incoming)
                .all(|neighbor| graph.neighbors_directed(neighbor, Outgoing).count() > 1)
        })
        .count()
}

pub fn solve_part_two(_falling_bricks: &FallingBricks) -> u32 {
    0
}
