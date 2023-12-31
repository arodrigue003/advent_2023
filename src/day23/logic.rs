use crate::day23::models::{Map, Tile};
use crate::day23::PreparedDataType;
use ndarray::Array2;
use petgraph::algo::bellman_ford;
use petgraph::graph::NodeIndex;
use petgraph::prelude::EdgeRef;
use petgraph::{Graph, Undirected};
use std::collections::{HashMap, VecDeque};

fn is_intersection(map: &Map, line: usize, column: usize) -> bool {
    // The tile is an interception if it has more than two non wall neighbor
    let mut neighbor_count = 0;
    if line > 0 && map.grid[line - 1][column] != Tile::Wall {
        neighbor_count += 1;
    }
    if column > 0 && map.grid[line][column - 1] != Tile::Wall {
        neighbor_count += 1;
    }
    if line < map.height - 1 && map.grid[line + 1][column] != Tile::Wall {
        neighbor_count += 1;
    }
    if column < map.width - 1 && map.grid[line][column + 1] != Tile::Wall {
        neighbor_count += 1;
    }
    neighbor_count > 2
}

pub fn prepare_data(map: &Map) -> (Graph<(usize, usize), f64>, NodeIndex, NodeIndex) {
    // Create the graph
    let mut graph = Graph::new();

    // Find start and end tiles
    let start_column = map.grid[0].iter().position(|tile| tile == &Tile::Empty).unwrap();
    let end_column = map.grid[map.height - 1]
        .iter()
        .position(|tile| tile == &Tile::Empty)
        .unwrap();

    // Transform them in nodes
    let start_node = graph.add_node((0, start_column));
    let end_node = graph.add_node((map.height - 1, end_column));

    // Store graph nodes in a hashmap
    let mut nodes: HashMap<_, _> = HashMap::new();

    // Add the start and end nodes in the map
    nodes.insert((0, start_column), start_node);
    nodes.insert((map.height - 1, end_column), end_node);

    // Keep track of visited nodes
    let mut visited = Array2::<bool>::default((map.height, map.width));

    // Create a queue for visit
    let mut queue = VecDeque::new();

    // Add the start to the queue
    queue.push_back((0, start_column, start_node, 0));

    while let Some((line, column, mut path_start_node, mut distance)) = queue.pop_front() {
        if is_intersection(map, line, column) {
            // Create the graph node or get it from the map
            let node = nodes
                .entry((line, column))
                .or_insert_with(|| graph.add_node((line, column)));

            // Add the edge between the path_start_node and the new node
            // We add the minus distance because we want to use a shortest path algorithm on -G
            graph.add_edge(path_start_node, *node, (-distance) as f64);

            // Reset the path_start_node and the distance for after
            path_start_node = *node;
            distance = 0;
        }

        // If we are at the end, always add the distance and continue
        if line == map.height - 1 && column == end_column {
            graph.add_edge(path_start_node, nodes[&(line, column)], (-distance) as f64);
            continue;
        }

        // If we already visited the node, don't do anything more
        if visited[[line, column]] {
            continue;
        }

        // Up
        if line > 0
            && (map.grid[line][column] == Tile::Empty || map.grid[line][column] == Tile::Top)
            && (map.grid[line - 1][column] == Tile::Empty || map.grid[line - 1][column] == Tile::Top)
        {
            queue.push_back((line - 1, column, path_start_node, distance + 1));
        }
        // Left
        if column > 0
            && (map.grid[line][column] == Tile::Empty || map.grid[line][column] == Tile::Left)
            && (map.grid[line][column - 1] == Tile::Empty || map.grid[line][column - 1] == Tile::Left)
        {
            queue.push_back((line, column - 1, path_start_node, distance + 1));
        }
        // Bot
        if line < map.height - 1
            && (map.grid[line][column] == Tile::Empty || map.grid[line][column] == Tile::Bottom)
            && (map.grid[line + 1][column] == Tile::Empty || map.grid[line + 1][column] == Tile::Bottom)
        {
            queue.push_back((line + 1, column, path_start_node, distance + 1));
        }
        // Right
        if column < map.width - 1
            && (map.grid[line][column] == Tile::Empty || map.grid[line][column] == Tile::Right)
            && (map.grid[line][column + 1] == Tile::Empty || map.grid[line][column + 1] == Tile::Right)
        {
            queue.push_back((line, column + 1, path_start_node, distance + 1));
        }

        visited[[line, column]] = true;
    }
    (graph, start_node, end_node)
}

pub fn solve_part_one((graph, start_node, end_node): &PreparedDataType) -> i64 {
    // Use Bellman Ford to get the lowest distance in -G, return -distance.
    let res = bellman_ford(graph, *start_node).unwrap();
    -(res.distances[end_node.index()] as i64)
}

fn get_all_paths(
    graph: &Graph<(usize, usize), usize, Undirected>,
    start: NodeIndex,
    end: NodeIndex,
    visited: &mut Vec<bool>,
    max_len: &mut usize,
    current_len: usize,
) {
    visited[start.index()] = true;
    if start == end {
        if current_len > *max_len {
            *max_len = current_len;
        }
    } else {
        for edge in graph.edges(start) {
            let target = edge.target();
            if !visited[target.index()] {
                get_all_paths(graph, target, end, visited, max_len, current_len + edge.weight());
            }
        }
    }

    visited[start.index()] = false;
}

pub fn solve_part_two((graph, start_node, end_node): &PreparedDataType) -> usize {
    // First, we need to convert the DAG of -G to an undirected graph of G
    let mut undirected_graph = Graph::new_undirected();

    // Construct a hashmap of every nodes in the first graph
    let nodes: HashMap<_, _> = graph
        .node_indices()
        .map(|node| {
            let (line, column) = graph.node_weight(node).unwrap();
            ((*line, *column), undirected_graph.add_node((*line, *column)))
        })
        .collect();

    // Add the edges
    for edge in graph.edge_references() {
        let source = nodes[&graph.node_weight(edge.source()).unwrap()];
        let target = nodes[&graph.node_weight(edge.target()).unwrap()];

        undirected_graph.add_edge(source, target, (-edge.weight()) as usize);
    }

    // Get the new start node and end node from the nodes map
    let start_node = nodes[&graph.node_weight(*start_node).unwrap()];
    let end_node = nodes[&graph.node_weight(*end_node).unwrap()];

    // Store visited nodes
    let mut visited = vec![false; undirected_graph.node_count()];
    let mut max_len = 0;
    get_all_paths(&undirected_graph, start_node, end_node, &mut visited, &mut max_len, 0);

    max_len
}
