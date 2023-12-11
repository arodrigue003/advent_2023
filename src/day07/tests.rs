use crate::common::{test_part_one_common, test_part_two_common};
use crate::day07::Day07;

static INPUT_EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

#[test]
fn test_part_one() {
    test_part_one_common(Day07::default(), INPUT_EXAMPLE, 6440);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day07::default(), INPUT_EXAMPLE, 5905);
}
