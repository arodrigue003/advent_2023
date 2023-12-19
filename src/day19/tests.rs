use crate::common::{test_part_one_common, test_part_two_common};
use crate::day19::Day19;

static INPUT_EXAMPLE: &str = "";

#[test]
fn test_part_one() {
    test_part_one_common(Day19::default(), INPUT_EXAMPLE, 19114);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day19::default(), INPUT_EXAMPLE, 1);
}
