use crate::day11::models::{SkyMap, Tile};

pub fn parse_input(input: String) -> SkyMap {
    SkyMap {
        grid: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '.' => Tile::Empty,
                        '#' => Tile::Galaxy,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    }
}
