use crate::common::{test_part_one_common, test_part_two_common};
use crate::day13::logic::find_mirrored_lines_part_one;
use crate::day13::Day13;

static INPUT_EXAMPLE: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

#[test]
fn test_part_one() {
    test_part_one_common(Day13::default(), INPUT_EXAMPLE, 405);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day13::default(), INPUT_EXAMPLE, 1);
}
