use crate::common::{test_part_one_common, test_part_two_common};
use crate::day21::Day21;

static INPUT_EXAMPLE: &str = "...........
......##.#.
.###..#..#.
..#.#...#..
....#.#....
.....S.....
.##......#.
.......##..
.##.#.####.
.##...#.##.
...........";

#[test]
fn test_part_one() {
    test_part_one_common(Day21::default(), INPUT_EXAMPLE, 47);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day21::default(), INPUT_EXAMPLE, 528192865877841);
}
