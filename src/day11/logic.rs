use crate::day11::models::{Coordinates, SkyMap, SparseSkyMap, Tile};

/// We want to store the position of every star in a vec.
/// We want to store the number of empty lines and empty column between them after that.
pub fn prepare(data: &SkyMap) -> SparseSkyMap {
    // Get data size
    let width = data.grid[0].len();
    let height = data.grid.len();

    let galaxies: Vec<_> = data
        .grid
        .iter()
        .enumerate()
        .flat_map(|(i_line, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, tile)| tile == &&Tile::Galaxy)
                .map(move |(i_column, _)| Coordinates::new(i_line, i_column))
        })
        .collect();

    // Precompute the list of empty lines
    let empty_lines: Vec<_> = data
        .grid
        .iter()
        .map(|line| line.iter().all(|tile| tile == &Tile::Empty))
        .map(|empty| empty as usize)
        .collect();

    // Now build a vec to store distances for lines.
    let mut empty_lines_count: Vec<Vec<_>> = (0..height)
        .map(|i_line_1| {
            (0..height)
                .scan(0, |acc, i_line_2| {
                    if empty_lines[i_line_1] == 0 && i_line_2 >= i_line_1 {
                        *acc += empty_lines[i_line_2];
                    }
                    Some(*acc)
                })
                .collect()
        })
        .collect();

    // Transpose the data in order to update the other part of the array
    for i_line_1 in 0..height {
        for i_line_2 in i_line_1..height {
            empty_lines_count[i_line_2][i_line_1] = empty_lines_count[i_line_1][i_line_2];
        }
    }

    // Do the same for the columns
    let empty_columns: Vec<_> = (0..width)
        .map(|i_column| {
            (0..height)
                .map(|i_line| &data.grid[i_line][i_column])
                .all(|tile| tile == &Tile::Empty)
        })
        .map(|empty| empty as usize)
        .collect();

    let mut empty_columns_count: Vec<Vec<_>> = (0..width)
        .map(|i_column_1| {
            (0..width)
                .scan(0, |acc, i_column_2| {
                    if empty_columns[i_column_1] == 0 && i_column_2 >= i_column_1 {
                        *acc += empty_columns[i_column_2];
                    }
                    Some(*acc)
                })
                .collect()
        })
        .collect();

    for i_column_1 in 0..width {
        for i_column_2 in i_column_1..width {
            empty_columns_count[i_column_2][i_column_1] = empty_columns_count[i_column_1][i_column_2];
        }
    }

    SparseSkyMap {
        galaxies,
        empty_lines_count,
        empty_columns_count,
    }
}

pub fn solve_part_one(prepared_data: &SparseSkyMap) -> usize {
    // For every pair of star compute the distance between them
    let mut total_distance = 0;
    for i_star_1 in 0..prepared_data.galaxies.len() {
        for i_star_2 in i_star_1 + 1..prepared_data.galaxies.len() {
            let star_1 = &prepared_data.galaxies[i_star_1];
            let star_2 = &prepared_data.galaxies[i_star_2];
            total_distance += star_2.column.abs_diff(star_1.column)
                + prepared_data.empty_columns_count[star_1.column][star_2.column]
                + star_2.line.abs_diff(star_1.line)
                + prepared_data.empty_lines_count[star_1.line][star_2.line];
        }
    }

    total_distance
}

pub fn solve_part_two(prepared_data: &SparseSkyMap) -> usize {
    // For every pair of star compute the distance between them
    let mut total_distance = 0;
    for i_star_1 in 0..prepared_data.galaxies.len() {
        for i_star_2 in i_star_1 + 1..prepared_data.galaxies.len() {
            let star_1 = &prepared_data.galaxies[i_star_1];
            let star_2 = &prepared_data.galaxies[i_star_2];
            total_distance += star_2.column.abs_diff(star_1.column)
                + prepared_data.empty_columns_count[star_1.column][star_2.column] * 999_999
                + star_2.line.abs_diff(star_1.line)
                + prepared_data.empty_lines_count[star_1.line][star_2.line] * 999_999;
        }
    }

    total_distance
}
