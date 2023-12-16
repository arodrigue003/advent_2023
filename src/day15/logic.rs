use crate::day15::models::{Action, BoxLens, Step};

fn compute_string_hash(data: &str) -> usize {
    data.chars().fold(0, |acc, char| (acc + char as usize) * 17 % 256)
}

pub fn solve_part_one(steps: &[Step]) -> usize {
    steps.iter().map(|step| compute_string_hash(&step.to_string())).sum()
}

pub fn solve_part_two(steps: &[Step]) -> usize {
    let mut boxes: Vec<Vec<BoxLens>> = vec![vec![]; 256];

    for step in steps {
        let hash = compute_string_hash(&step.label);
        let lens_position = boxes[hash].iter().position(|lens| lens.label == step.label);

        match step.action {
            Action::Remove => {
                if let Some(lens_position) = lens_position {
                    boxes[hash].remove(lens_position);
                }
            }
            Action::Add(focal) => {
                if let Some(lens_position) = lens_position {
                    boxes[hash][lens_position].focal = focal;
                } else {
                    boxes[hash].push(BoxLens {
                        label: step.label.to_string(),
                        focal,
                    })
                }
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .flat_map(|(box_number, lens_library)| {
            lens_library
                .iter()
                .enumerate()
                .map(move |(slot_number, box_lens)| (1 + box_number) * (1 + slot_number) * box_lens.focal)
        })
        .sum()
}
