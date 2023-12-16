use crate::day02::models::Game;

pub fn solve_part_one(data: &[Game]) -> u32 {
    data.iter()
        .filter(|game| {
            game.subsets
                .iter()
                .all(|subset| subset.red <= 12 && subset.green <= 13 && subset.blue <= 14)
        })
        .map(|game| game.index)
        .sum()
}

pub fn solve_part_two(data: &[Game]) -> u32 {
    data.iter()
        .map(|game| {
            game.subsets.iter().map(|subset| subset.red).max().unwrap()
                * game.subsets.iter().map(|subset| subset.green).max().unwrap()
                * game.subsets.iter().map(|subset| subset.blue).max().unwrap()
        })
        .sum()
}
