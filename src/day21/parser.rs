use crate::day21::models::Map;

pub fn parse_input(input: String) -> Map {
    let mut start = (usize::MAX, usize::MAX);

    let grid = input
        .lines()
        .enumerate()
        .map(|(i_line, line)| {
            line.chars()
                .enumerate()
                .map(|(i_column, tile)| match tile {
                    '.' => false,
                    '#' => true,
                    'S' => {
                        start = (i_line, i_column);
                        false
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    Map::new(grid, start)
}
