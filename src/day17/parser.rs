use crate::day17::models::Map;

static ZERO_VALUE: u32 = '0' as u32;

pub fn parse_input(input: String) -> Map {
    Map::new(
        input
            .lines()
            .map(|line| line.chars().map(|char| char as u32 - ZERO_VALUE).collect())
            .collect(),
    )
}
