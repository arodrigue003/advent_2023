use crate::common::{test_part_one_common, test_part_two_common};
use crate::day01::Day01;

static INPUT_EXAMPLE_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

static INPUT_EXAMPLE_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

#[test]
fn test_part_one() {
    test_part_one_common(Day01::default(), INPUT_EXAMPLE_1, 142);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day01::default(), INPUT_EXAMPLE_2, 281);
}
