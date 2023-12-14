use crate::day13::models::Grid;

pub fn find_mirrored_lines(lines: &[u64], width: usize) -> Option<usize> {
    let mut positions: u64 = (1 << width) - 1;

    for line_int in lines {
        if positions == 0 {
            return None;
        }

        let reversed = line_int.reverse_bits();

        positions = (1..width).fold(0, |acc, position| {
            let pattern_width = std::cmp::min(position, width - position);

            let res = if positions & (1 << (width - position - 1)) > 0 {
                let right = line_int << 64 - width + position >> 64 - pattern_width;
                let left = reversed << width - position >> 64 - pattern_width;
                (left == right) as u64
            } else {
                0
            };

            (acc << 1) + res
        });
    }

    if positions == 0 {
        None
    } else {
        Some(width - positions.ilog2() as usize - 1)
    }
}

pub fn find_mirrored_lines_with_replacement(lines: &[u64], width: usize) -> Option<usize> {
    let part_one_result = find_mirrored_lines(lines, width);

    let height = lines.len();

    for i_line in 0..height {
        for i_column in 0..width {
            let mut positions: u64 = (1 << width) - 1;

            for i_line_inner in 0..height {
                if positions == 0 {
                    break;
                }

                let mut line_int = lines[i_line_inner];

                // If a replacement was selected, do it on the line.
                if i_line_inner == i_line {
                    line_int ^= 1 << (width - i_column - 1);
                }

                let reversed = line_int.reverse_bits();

                positions = (1..width).fold(0, |acc, position| {
                    let pattern_width = std::cmp::min(position, width - position);

                    let res = if positions & (1 << (width - position - 1)) > 0 {
                        let right = line_int << 64 - width + position >> 64 - pattern_width;
                        let left = reversed << width - position >> 64 - pattern_width;
                        (left == right) as u64
                    } else {
                        0
                    };

                    (acc << 1) + res
                });
            }

            // clear the bit of the part one result
            if let Some(part_one_result) = part_one_result {
                // println!("{:#066b}", positions);
                positions &= !(1 << (width - part_one_result - 1));
                // println!("{:#066b}", positions);
            }

            if positions != 0 {
                return Some(width - positions.ilog2() as usize - 1);
            }
        }
    }

    None
}

pub fn solve_part_one(data: &[Grid]) -> usize {
    data.iter()
        .filter_map(|grid| find_mirrored_lines(&grid.lines, grid.width))
        .sum::<usize>()
        + 100
            * data
                .iter()
                .filter_map(|grid| find_mirrored_lines(&grid.columns, grid.height))
                .sum::<usize>()
}

pub fn solve_part_two(data: &[Grid]) -> usize {
    data.iter()
        .filter_map(|grid| find_mirrored_lines_with_replacement(&grid.lines, grid.width))
        .sum::<usize>()
        + 100
            * data
                .iter()
                .filter_map(|grid| find_mirrored_lines_with_replacement(&grid.columns, grid.height))
                .sum::<usize>()
}
