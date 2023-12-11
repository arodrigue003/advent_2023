use crate::common::{test_part_one_common, test_part_two_common};
use crate::day03::Day03;

static INPUT_EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

#[test]
fn test_part_one() {
    test_part_one_common(Day03::default(), INPUT_EXAMPLE, 4361);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day03::default(), INPUT_EXAMPLE, 467835);
}
