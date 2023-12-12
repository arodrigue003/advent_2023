use crate::day12::models::{ConditionRecord, SpringStatus};

fn get_combination_count(
    condition_record: &ConditionRecord,
    status_pos: usize,
    group_pos: usize,
) -> usize {
    // println!("{status_pos}, {group_pos}",);

    if status_pos >= condition_record.spring_status.len() {
        // End condition
        if group_pos >= condition_record.spring_groups.len() {
            // We were able to reach the end of both our input
            // println!("exit 1");
            1
        } else {
            // We reached the end of the shapes but we were not able to get an through the end
            // of the groups
            // println!("exit 2");
            0
        }
    } else {
        // We didn't reach the end of the status yet
        if group_pos >= condition_record.spring_groups.len() {
            // We already exhausted our group options, either we have a case we can ignore or
            // the combination is invalid
            if condition_record.spring_status[status_pos] == SpringStatus::Damaged {
                // println!("exit 3");
                0
            } else {
                // println!("exit 4");
                get_combination_count(condition_record, status_pos + 1, group_pos)
            }
        } else {
            // println!("exit 5");
            // We try to get further in the computation depending on the tile status
            let mut res = 0;
            let group_len = condition_record.spring_groups[group_pos];

            if condition_record.spring_status[status_pos] != SpringStatus::Damaged {
                // We can try to skip this tile
                // println!("Sub1");
                res += get_combination_count(condition_record, status_pos + 1, group_pos)
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
                // println!("Sub2");
                res += get_combination_count(
                    condition_record,
                    status_pos + group_len + 1,
                    group_pos + 1,
                );
            }

            res
        }
    }
}

pub fn solve_part_one(conditions_records: &[ConditionRecord]) -> usize {
    conditions_records
        .iter()
        .map(|record| get_combination_count(record, 0, 0))
        .sum()
}

pub fn solve_part_two(_data: &[ConditionRecord]) -> u32 {
    0
}
