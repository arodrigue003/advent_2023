use crate::day19::models::{Action, Part, Rule, System, Workflow};
use std::collections::HashMap;

impl Rule {
    fn get_action(&self, part: &Part) -> Option<&Action> {
        if (self.test)(part, self.value) {
            Some(&self.action)
        } else {
            None
        }
    }
}

impl Workflow {
    fn get_action(&self, part: &Part) -> &Action {
        for rule in &self.rules {
            if let Some(action) = rule.get_action(part) {
                return action;
            }
        }

        return &self.default_action;
    }
}

fn is_accepted(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    // Get the first workflow
    let mut workflow_name: &str = "in";

    loop {
        let workflow = &workflows[workflow_name];
        let action = workflow.get_action(part);

        match action {
            Action::Goto(target) => workflow_name = target.as_str(),
            Action::Accepted => return true,
            Action::Rejected => return false,
        }
    }
}

pub fn solve_part_one(system: &System) -> i32 {
    system
        .parts
        .iter()
        .filter(|part| is_accepted(part, &system.workflows))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

pub fn solve_part_two(system: &System) -> u32 {
    0
}
