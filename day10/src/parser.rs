use crate::models::{Grid, Point, Tile};

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::BottomLeft,
            'J' => Self::BottomRight,
            '7' => Self::TopRight,
            'F' => Self::TopLeft,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

pub fn parse_data(data: &str) -> Grid {
    // Parse tiles
    let tiles: Vec<Vec<_>> = data
        .lines()
        .map(|line| line.chars().map(From::from).collect())
        .collect();

    // Compute size
    let width = tiles[0].len();
    let height = tiles.len();

    // Add a border to simplify further computation
    let tiles: Vec<Vec<_>> = std::iter::once(vec![Tile::Ground; width + 2])
        .chain(tiles.into_iter().map(|line| {
            std::iter::once(Tile::Ground)
                .chain(line)
                .chain(std::iter::once(Tile::Ground))
                .collect()
        }))
        .chain(std::iter::once(vec![Tile::Ground; width + 2]))
        .collect();

    // Extract starting position
    let start = tiles
        .iter()
        .enumerate()
        .filter_map(|(i_line, line)| {
            line.iter()
                .position(|tile| *tile == Tile::Start)
                .map(|i_column| (i_line, i_column))
        })
        .next()
        .unwrap();

    // Create the grid object
    Grid {
        tiles,
        width: width + 2,
        height: height + 2,
        start: Point::new(start.0, start.1),
    }
}
