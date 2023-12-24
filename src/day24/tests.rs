use crate::common::{test_part_one_common, test_part_two_common};
use crate::day24::Day24;

static INPUT_EXAMPLE: &str = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
";

#[test]
fn test_part_one() {
    test_part_one_common(Day24::new(7, 27), INPUT_EXAMPLE, 2);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day24::default(), INPUT_EXAMPLE, 47);
}
