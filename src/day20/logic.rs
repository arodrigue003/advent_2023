use crate::day20::models::{Broadcaster, CableManagement, Conjunction, FlipFlop, Module, ModuleType, Untyped};
use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use std::collections::VecDeque;

impl Module for Untyped {
    fn get_pulses(&mut self, _input_offset: u16, _is_high: bool) -> Option<(&Vec<(usize, u16)>, bool)> {
        None
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Untyped
    }

    fn get_neighbors(&self) -> Vec<usize> {
        vec![]
    }
}

impl Module for Broadcaster {
    fn get_pulses(&mut self, _input_offset: u16, is_high: bool) -> Option<(&Vec<(usize, u16)>, bool)> {
        // Broadcaster always broadcast to every output
        Some((&self.output, is_high))
    }

    fn get_name(&self) -> &str {
        "broadcaster"
    }

    fn is_broadcaster(&self) -> bool {
        true
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Broadcaster
    }

    fn get_neighbors(&self) -> Vec<usize> {
        self.output.iter().map(|(neighbor, _)| *neighbor).collect()
    }
}

impl Module for FlipFlop {
    fn get_pulses(&mut self, _input_offset: u16, is_high: bool) -> Option<(&Vec<(usize, u16)>, bool)> {
        // Update the value if necessary
        if is_high {
            None
        } else {
            self.memory = !self.memory;
            Some((&self.output, self.memory))
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::FlipFlop
    }

    fn get_neighbors(&self) -> Vec<usize> {
        self.output.iter().map(|(neighbor, _)| *neighbor).collect()
    }
}

impl Module for Conjunction {
    fn get_pulses(&mut self, input_offset: u16, is_high: bool) -> Option<(&Vec<(usize, u16)>, bool)> {
        // Set the value for the associated input_offset
        self.value = self.value & !(1 << input_offset) | ((is_high as u16) << input_offset);

        Some((&self.output, self.value != self.mask))
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> ModuleType {
        ModuleType::Conjunction
    }

    fn get_neighbors(&self) -> Vec<usize> {
        self.output.iter().map(|(neighbor, _)| *neighbor).collect()
    }
}

pub fn solve_part_one(cable_management: &CableManagement) -> usize {
    // Clone the cable management to be able to modify it
    let mut cable_management = cable_management.clone();

    // Get broadcaster position
    let broadcaster_position = cable_management
        .modules
        .iter()
        .position(|module| module.is_broadcaster())
        .unwrap();

    // Create the queue
    let mut queue = VecDeque::with_capacity(1000);

    // Store high and low count
    let mut high_count = 0;
    let mut low_count = 0;

    // Perform 1_000 click
    for _ in 0..1_000 {
        // Add the button press to low_count
        low_count += 1;

        queue.push_back((broadcaster_position, 0, false));

        while let Some((position, input_offset, is_high)) = queue.pop_front() {
            // Get the new elements
            if let Some((outputs, output_is_high)) =
                cable_management.modules[position].get_pulses(input_offset, is_high)
            {
                if output_is_high {
                    high_count += outputs.len();
                } else {
                    low_count += outputs.len();
                }

                for output in outputs {
                    queue.push_back((output.0, output.1, output_is_high))
                }
            }
        }
    }

    low_count * high_count
}

pub fn solve_part_two(cable_management_ref: &CableManagement) -> u32 {
    // Create a graph with petgraph
    let mut graph: Graph<(ModuleType, String), ()> = Graph::new();

    let nodes: Vec<_> = cable_management_ref
        .modules
        .iter()
        .map(|module| graph.add_node((module.get_type(), module.get_name().to_string())))
        .collect();

    for (position, module) in cable_management_ref.modules.iter().enumerate() {
        for neighbor in module.get_neighbors() {
            graph.add_edge(nodes[position], nodes[neighbor], ());
        }
    }

    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    //
    // // Clone the cable management to be able to modify it
    // let mut cable_management = cable_management_ref.clone();
    //
    // // Get broadcaster position
    // let broadcaster_position = cable_management
    //     .modules
    //     .iter()
    //     .position(|module| module.is_broadcaster())
    //     .unwrap();
    //
    // // Get bt position
    // let bt_position = cable_management
    //     .modules
    //     .iter()
    //     .position(|module| module.get_name() == "rx")
    //     .unwrap();
    //
    // // Create the queue
    // let mut queue = VecDeque::with_capacity(1000);
    //
    // // Store high and low count
    // let mut count = 0;
    //
    // // Perform 1_000 click
    // loop {
    //     queue.push_back((broadcaster_position, 0, false));
    //
    //     // While the signal is running
    //     while let Some((position, input_offset, is_high)) = queue.pop_front() {
    //         // Get the new elements
    //         if let Some((outputs, output_is_high)) =
    //             cable_management.modules[position].get_pulses(input_offset, is_high)
    //         {
    //             // For each output of the module
    //             for (destination, input_offset) in outputs {
    //                 queue.push_back((*destination, *input_offset, output_is_high));
    //
    //                 // If the output was sent to bt, display it
    //                 if *destination == bt_position {
    //                     println!(
    //                         "Action: {} -{}-> {} (step: {})",
    //                         cable_management_ref.modules[position].get_name(),
    //                         is_high,
    //                         cable_management_ref.modules[*destination].get_name(),
    //                         count
    //                     )
    //                 }
    //             }
    //         }
    //     }
    //
    //     count += 1;
    // }

    0
}
