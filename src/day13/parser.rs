use crate::day13::models::Grid;

impl Grid {
    pub fn new(lines: Vec<u64>, width: usize) -> Self {
        // Get height
        let height = lines.len();

        // transpose the matrix
        let columns: Vec<_> = (0..width)
            .map(|i_column| {
                (0..height).fold(0, |acc, i_line| {
                    (acc << 1) + ((lines[i_line] & (1 << (width - i_column - 1)) > 0) as u64)
                })
            })
            .collect();

        Self {
            lines,
            columns,
            width,
            height,
        }
    }
}

pub fn parse_input(input: String) -> Vec<Grid> {
    let mut grids: Vec<Grid> = vec![];

    let mut last_grid = vec![];
    let mut width: usize = 0;

    for line in input.lines() {
        if line.is_empty() {
            grids.push(Grid::new(last_grid, width));
            last_grid = vec![];
            width = 0;
        } else {
            width = line.len();
            last_grid.push(line.chars().fold(0, |acc, char| {
                (acc << 1)
                    + match char {
                        '.' => 0,
                        '#' => 1,
                        _ => unimplemented!(),
                    }
            }));
        }
    }
    grids.push(Grid::new(last_grid, width));

    grids
}
