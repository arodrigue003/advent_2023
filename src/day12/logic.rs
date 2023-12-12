use crate::day12::models::{ConditionRecord, SpringStatus};

fn get_combination_count(
    memory: &mut Vec<Vec<usize>>,
    condition_record: &ConditionRecord,
    status_pos: usize,
    group_pos: usize,
) -> usize {
    if memory[status_pos][group_pos] != usize::MAX {
        return memory[status_pos][group_pos];
    }

    let res = if status_pos >= condition_record.spring_status.len() {
        // End condition
        if group_pos >= condition_record.spring_groups.len() {
            // We were able to reach the end of both our input
            1
        } else {
            // We reached the end of the shapes but we were not able to get an through the end
            // of the groups
            0
        }
    } else {
        // We didn't reach the end of the status yet
        if group_pos >= condition_record.spring_groups.len() {
            // We already exhausted our group options, either we have a case we can ignore or
            // the combination is invalid
            if condition_record.spring_status[status_pos] == SpringStatus::Damaged {
                0
            } else {
                get_combination_count(memory, condition_record, status_pos + 1, group_pos)
            }
        } else {
            // We try to get further in the computation depending on the tile status
            let mut res = 0;
            let group_len = condition_record.spring_groups[group_pos];

            if condition_record.spring_status[status_pos] != SpringStatus::Damaged {
                // We can try to skip this tile
                res += get_combination_count(memory, condition_record, status_pos + 1, group_pos)
            }

            if (status_pos + group_len) <= condition_record.spring_status.len()
                && condition_record.spring_status[status_pos..status_pos + group_len]
                    .iter()
                    .all(|tile| tile != &SpringStatus::Operational)
                && (status_pos + group_len >= condition_record.spring_status.len()
                    || condition_record.spring_status[status_pos + group_len]
                        != SpringStatus::Damaged)
            {
                // We can move a shape forward
                res += get_combination_count(
                    memory,
                    condition_record,
                    status_pos + group_len + 1,
                    group_pos + 1,
                );
            }

            res
        }
    };

    memory[status_pos][group_pos] = res;

    res
}

pub fn solve_part_one(conditions_records: &[ConditionRecord]) -> usize {
    conditions_records
        .iter()
        .map(|record| {
            let mut memory = vec![
                vec![usize::MAX; record.spring_groups.len() + 2];
                record.spring_status.len() + 2
            ];

            get_combination_count(&mut memory, record, 0, 0)
        })
        .sum()
}

pub fn solve_part_two(conditions_records: &[ConditionRecord]) -> usize {
    // Clone condition records
    let mut conditions_records = conditions_records.to_vec();
    for condition_record in &mut conditions_records {
        // Expand the status
        condition_record.spring_status = condition_record
            .spring_status
            .clone()
            .into_iter()
            .chain(std::iter::once(SpringStatus::Unknown))
            .chain(condition_record.spring_status.clone())
            .chain(std::iter::once(SpringStatus::Unknown))
            .chain(condition_record.spring_status.clone())
            .chain(std::iter::once(SpringStatus::Unknown))
            .chain(condition_record.spring_status.clone())
            .chain(std::iter::once(SpringStatus::Unknown))
            .chain(condition_record.spring_status.clone())
            .collect();

        // Expend the groups
        condition_record.spring_groups = condition_record
            .spring_groups
            .clone()
            .into_iter()
            .chain(condition_record.spring_groups.clone())
            .chain(condition_record.spring_groups.clone())
            .chain(condition_record.spring_groups.clone())
            .chain(condition_record.spring_groups.clone())
            .collect();
    }

    conditions_records
        .iter()
        .map(|record| {
            let mut memory = vec![
                vec![usize::MAX; record.spring_groups.len() + 2];
                record.spring_status.len() + 2
            ];

            get_combination_count(&mut memory, record, 0, 0)
        })
        .sum()
}
