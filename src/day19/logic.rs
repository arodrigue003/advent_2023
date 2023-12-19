use crate::day19::models::{Action, Part, PartValue, Rule, System, Test, Workflow};
use std::collections::{HashMap, VecDeque};

impl Rule {
    fn get_action(&self, part: &Part) -> Option<&Action> {
        if (self.test_func)(part, self.value) {
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

        &self.default_action
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

pub fn solve_part_one(system: &System) -> i64 {
    system
        .parts
        .iter()
        .filter(|part| is_accepted(part, &system.workflows))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn count(&self) -> i64 {
        (self.x.end - self.x.start)
            * (self.m.end - self.m.start)
            * (self.a.end - self.a.start)
            * (self.s.end - self.s.start)
    }

    fn clone_with_modified_range(&self, part_value: &PartValue, new_range: &Range) -> Self {
        match part_value {
            PartValue::X => Self {
                x: new_range.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            PartValue::M => Self {
                x: self.x.clone(),
                m: new_range.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            PartValue::A => Self {
                x: self.x.clone(),
                m: self.m.clone(),
                a: new_range.clone(),
                s: self.s.clone(),
            },
            PartValue::S => Self {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: new_range.clone(),
            },
        }
    }
}

pub fn solve_part_two(system: &System) -> i64 {
    // Create structures
    let mut res = 0;
    let mut queue: VecDeque<(&str, PartRange)> = VecDeque::new();

    // Add the initial PartRange
    queue.push_back((
        "in",
        PartRange {
            x: Range::new(1, 4001),
            m: Range::new(1, 4001),
            a: Range::new(1, 4001),
            s: Range::new(1, 4001),
        },
    ));

    // While we still have item in the queue
    while let Some((workflow_name, part_range)) = queue.pop_front() {
        let workflow = &system.workflows[workflow_name];

        let mut part_range = Some(part_range);

        for rule in &workflow.rules {
            if let Some(inner_part_range) = part_range.as_ref() {
                // Get the PartRange rule associated range
                let part_concerned_range = match rule.part_value {
                    PartValue::X => &inner_part_range.x,
                    PartValue::M => &inner_part_range.m,
                    PartValue::A => &inner_part_range.a,
                    PartValue::S => &inner_part_range.s,
                };

                // Split the range accordingly
                let (matched, remaining) = match rule.test {
                    Test::Lower => {
                        (
                            if part_concerned_range.start < rule.value {
                                Some(Range::new(
                                    part_concerned_range.start,
                                    part_concerned_range.end.min(rule.value),
                                ))
                            } else {
                                // The range is outside of the condition and is never matched
                                None
                            },
                            if rule.value < part_concerned_range.end {
                                Some(Range::new(
                                    part_concerned_range.start.max(rule.value),
                                    part_concerned_range.end,
                                ))
                            } else {
                                None
                            },
                        )
                    }
                    Test::Greater => {
                        (
                            if rule.value + 1 < part_concerned_range.end {
                                Some(Range::new(
                                    part_concerned_range.start.max(rule.value + 1),
                                    part_concerned_range.end,
                                ))
                            } else {
                                // The range is outside of the condition and is never matched
                                None
                            },
                            if part_concerned_range.start < rule.value + 1 {
                                Some(Range::new(
                                    part_concerned_range.start,
                                    part_concerned_range.end.min(rule.value + 1),
                                ))
                            } else {
                                None
                            },
                        )
                    }
                };

                // If the matched range is not null
                if let Some(matched) = matched {
                    // Create a PartRange from it
                    let matched_part_range =
                        PartRange::clone_with_modified_range(inner_part_range, &rule.part_value, &matched);

                    // Do the action according to the action
                    match &rule.action {
                        Action::Goto(x) => {
                            // Add the new part range to the queue to run it in another workflow
                            queue.push_back((x.as_str(), matched_part_range));
                        }
                        Action::Accepted => {
                            // Update the result
                            res += matched_part_range.count();
                        }
                        Action::Rejected => {
                            // Eject the part from the system by doing nothing
                        }
                    }
                }

                // Update part_range
                if let Some(remaining) = remaining {
                    let remaining_part_range =
                        PartRange::clone_with_modified_range(inner_part_range, &rule.part_value, &remaining);
                    part_range = Some(remaining_part_range);
                } else {
                    part_range = None;
                }
            } else {
                // We don't have any part_range left to match
                break;
            }
        }

        // If part_range is not null, apply the default option
        if let Some(inner_part_range) = part_range {
            match &workflow.default_action {
                Action::Goto(x) => queue.push_back((x.as_str(), inner_part_range)),
                Action::Accepted => res += inner_part_range.count(),
                Action::Rejected => {}
            }
        }
    }

    res
}
