use crate::day04::models::Game;

pub fn solve_part_one(data: &[Game]) -> u32 {
    data.iter()
        .map(|game| game.draw.intersection(&game.winning).count())
        .filter(|correct| *correct > 0)
        .map(|correct| 1 << (correct - 1))
        .sum()
}

pub fn solve_part_two(data: &[Game]) -> u32 {
    let game_count = data.len();
    let mut copies = vec![1; game_count];

    for (i, game) in data.iter().enumerate() {
        let correct = game.draw.intersection(&game.winning).count();

        for j in i + 1..game_count.min(i + 1 + correct) {
            copies[j] += copies[i];
        }
    }

    copies.iter().sum()
}
