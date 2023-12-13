use crate::day13::models::{Grid, Grid2, Tile};

pub fn find_mirrored_lines_part_one(lines: &[u64], width: usize) -> Option<usize> {
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

fn find_mirrored_lines(lines: &[u64], width: usize) -> Option<usize> {
    let mut positions: u64 = (1 << width) - 1;

    for line_int in lines {
        if positions == 0 {
            return None;
        }

        let fusion = line_int | (line_int << width);
        let fusion_reversed = fusion.reverse_bits();

        positions = (0..width).fold(0, |acc, position| {
            let res = if positions & (1 << (width - position - 1)) > 0 {
                let right = fusion << (64 - 2 * width + position) >> (64 - width / 2);
                let left = fusion_reversed << (width - position) >> (64 - width / 2);
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

fn find_mirrored_line_2(lines: &[Vec<Tile>]) -> Option<usize> {
    let width = lines[0].len();
    let mut positions = vec![true; width];

    for line in lines {
        if positions.iter().all(|position| *position == false) {
            return None;
        }

        for position in 0..width {
            // for every starting position

            if positions[position] == true {
                // If the position was not ruled out early

                for i in 0..width {
                    // for every point starting from the position

                    if line[(position + i) % width] != line[(position + width - i - 1) % width] {
                        // no match, the status become failed
                        positions[position] = false;
                        break;
                    }
                }
            }
        }
    }

    positions
        .iter()
        .enumerate()
        .filter(|(_, found)| **found == true)
        .map(|(pos, _)| pos)
        .next()
}

fn find_mirrored_column_2(lines: &[Vec<Tile>]) -> Option<usize> {
    let height = lines.len();
    let width = lines[0].len();
    let mut positions = vec![true; height];

    for i_column in 0..width {
        if positions.iter().all(|position| *position == false) {
            return None;
        }

        for position in 0..height {
            // for every starting position

            if positions[position] == true {
                // If the position was not ruled out early

                for i in 0..height {
                    // for every point starting from the position

                    if lines[(position + i) % height][i_column]
                        != lines[(position + height - i - 1) % height][i_column]
                    {
                        // no match, the status become failed
                        positions[position] = false;
                        break;
                    }
                }
            }
        }
    }

    positions
        .iter()
        .enumerate()
        .filter(|(_, found)| **found == true)
        .map(|(pos, _)| pos)
        .next()
}

pub fn solve_part_one(data: &[Grid2]) -> usize {
    data.iter()
        .enumerate()
        .filter_map(|(pos, grid)| {
            let res = find_mirrored_line_2(&grid.grid);
            if res != None {
                println!("{pos}, {:?}", res);
            };
            res
        })
        .sum::<usize>()
        + data
            .iter()
            .enumerate()
            .filter_map(|(pos, grid)| {
                let res = find_mirrored_column_2(&grid.grid);
                if res != None {
                    println!("{pos}, {:?}", res);
                };
                res
            })
            .map(|score| score * 100)
            .sum::<usize>()
}

pub fn solve_part_one_2(data: &[Grid]) -> usize {
    data.iter()
        .filter_map(|grid| find_mirrored_lines_part_one(&grid.lines, grid.width))
        .sum::<usize>()
        + 100
            * data
                .iter()
                .filter_map(|grid| find_mirrored_lines_part_one(&grid.columns, grid.height))
                .sum::<usize>()
}

pub fn solve_part_two(_data: &[Grid2]) -> u32 {
    0
}

pub fn solve_part_two_2(_data: &[Grid]) -> u32 {
    0
}
