use crate::common::{test_part_one_common, test_part_two_common};
use crate::day09::Day09;

static INPUT_EXAMPLE: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

#[test]
fn test_part_one() {
    test_part_one_common(Day09::default(), INPUT_EXAMPLE, 114);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day09::default(), INPUT_EXAMPLE, 2);
}
