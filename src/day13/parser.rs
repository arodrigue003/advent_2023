use crate::day13::models::{Grid, Grid2, Tile};

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
        //
        // for column in &columns {
        //     println!("{:#066b}", column);
        // }
        // println!("{}", columns.len());
        // println!("{}", height);
        // println!("{}", width);

        Self {
            lines,
            columns,
            width,
            height,
        }
    }
}

pub fn parse_input_2(input: String) -> Vec<Grid> {
    let mut grids: Vec<Grid> = vec![];

    let mut last_grid = vec![];
    let mut width: usize = 0;

    for line in input.lines() {
        if line == "" {
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

pub fn parse_input(input: String) -> Vec<Grid2> {
    let mut grids: Vec<Grid2> = vec![];

    let mut last_grid: Vec<Vec<Tile>> = vec![];

    for line in input.lines() {
        if line == "" {
            grids.push(Grid2 { grid: last_grid });
            last_grid = vec![];
        } else {
            last_grid.push(
                line.chars()
                    .map(|char| match char {
                        '.' => Tile::Ash,
                        '#' => Tile::Rock,
                        _ => unimplemented!(),
                    })
                    .collect(),
            );
        }
    }
    grids.push(Grid2 { grid: last_grid });

    println!("{:?}", &grids);

    grids
}
