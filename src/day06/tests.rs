use crate::common::{test_part_one_common, test_part_two_common};
use crate::day06::Day06;

static INPUT_EXAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn test_part_one() {
    test_part_one_common(Day06::default(), INPUT_EXAMPLE, 288);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day06::default(), INPUT_EXAMPLE, 71503);
}
