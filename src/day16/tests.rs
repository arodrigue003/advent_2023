use crate::common::{test_part_one_common, test_part_two_common};
use crate::day16::Day16;

static INPUT_EXAMPLE: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

#[test]
fn test_part_one() {
    test_part_one_common(Day16::default(), INPUT_EXAMPLE, 46);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day16::default(), INPUT_EXAMPLE, 51);
}
