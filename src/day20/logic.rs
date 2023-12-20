use crate::day20::models::{CableManagement, Module, ModuleType};
use std::collections::VecDeque;

impl Module {
    fn handle_broadcast(&self, is_high: bool) -> Vec<(usize, u16, bool)> {
        // Broadcaster always broadcast to every output
        self.output
            .iter()
            .map(|(position, input_offset)| (*position, *input_offset, is_high))
            .collect()
    }

    fn handle_flipflop(&mut self, is_high: bool) -> Vec<(usize, u16, bool)> {
        // Update the value if necessary
        if is_high {
            vec![]
        } else {
            self.memory = !self.memory;
            self.output
                .iter()
                .map(|(position, input_offset)| (*position, *input_offset, self.memory))
                .collect()
        }
    }

    fn handle_conjunction(&mut self, input_offset: u16, is_high: bool) -> Vec<(usize, u16, bool)> {
        // Set the value for the associated input_offset
        self.value = self.value & !(1 << input_offset) | ((is_high as u16) << input_offset);

        self.output
            .iter()
            .map(|(position, input_offset)| (*position, *input_offset, self.value != self.mask))
            .collect()
    }

    // Process the input pulse and return the list of module position to send pulse to
    // with the input offset and the pulse value (true is high, false is low)
    fn get_pulses(&mut self, input_offset: u16, is_high: bool) -> Vec<(usize, u16, bool)> {
        match &self.module_type {
            ModuleType::Broadcaster => self.handle_broadcast(is_high),
            ModuleType::FlipFlop => self.handle_flipflop(is_high),
            ModuleType::Conjunction => self.handle_conjunction(input_offset, is_high),
            ModuleType::Untyped => vec![],
        }
    }
}

pub fn solve_part_one(cable_management: &CableManagement) -> u32 {
    // Clone the cable management to be able to modify it
    let mut cable_management = cable_management.clone();

    // Get broadcaster position
    let broadcaster_position = cable_management
        .modules
        .iter()
        .position(|module| module.module_type == ModuleType::Broadcaster)
        .unwrap();

    // Create the queue
    let mut queue = VecDeque::new();

    // Store high and low count
    let mut high_count = 0;
    let mut low_count = 0;

    // Perform 1_000 click
    for _ in 0..1000 {
        // Add the button press to low_count
        low_count += 1;

        queue.push_back((broadcaster_position, 0, false));

        while let Some((position, input_offset, is_high)) = queue.pop_front() {
            // Get the new elements
            let new_elements = cable_management.modules[position].get_pulses(input_offset, is_high);
            for new_element in new_elements {
                if new_element.2 {
                    high_count += 1;
                } else {
                    low_count += 1;
                }
                // println!(
                //     "action: {} -{}-> {}",
                //     &cable_management.modules[position].name,
                //     &new_element.2,
                //     &cable_management.modules[new_element.0].name
                // );
                queue.push_back(new_element);
            }
        }
        // println!("{}", &cable_management);
    }
    low_count * high_count
}

pub fn solve_part_two(cable_management: &CableManagement) -> u32 {
    0
}
