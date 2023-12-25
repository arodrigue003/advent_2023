use crate::day25::models::Connection;
use petgraph::graph::EdgeIndex;
use petgraph::Graph;
use rand::Rng;
use std::collections::HashMap;

pub fn solve_part_one(connections: &[Connection]) -> u32 {
    // Create a graph
    let mut graph = Graph::new_undirected();

    // Store nodes
    let mut nodes: HashMap<_, _> = HashMap::new();

    for connection in connections {
        // get the node from the map
        let start = nodes
            .entry(&connection.name)
            .or_insert_with(|| graph.add_node(1))
            .clone();

        for other in &connection.others {
            let end = nodes.entry(&other).or_insert_with(|| graph.add_node(1));

            graph.add_edge(start, *end, 1);
        }
    }

    let mut rng = rand::thread_rng();

    // Perform the algorithm while the cut is not of size 3
    loop {
        let mut new_graph = graph.clone();

        // Use the Karger's algorithm to find the minimal cut
        while new_graph.node_count() > 2 {
            // Choose two nodes arbitrary
            let edge_index = EdgeIndex::new(rng.gen_range(0..new_graph.edge_count()));

            // Get the edge from the graph
            let (start, end) = new_graph.edge_endpoints(edge_index).unwrap();

            // Connect every edge to end to start instead
            let neighbors: Vec<_> = new_graph.neighbors_undirected(end).into_iter().collect();
            for neighbor in neighbors {
                if start != neighbor {
                    new_graph.add_edge(start, neighbor, 1);
                }
            }

            // Update the weight of start to take into account the removal of end
            new_graph[start] = new_graph[start] + new_graph[end];

            // Pop end
            new_graph.remove_node(end);
        }

        if new_graph.edge_count() == 3 {
            return new_graph.node_weights().fold(1, |acc, weight| acc * *weight);
        }
    }
}

pub fn solve_part_two(_connections: &[Connection]) -> u32 {
    0
}
