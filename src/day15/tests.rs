use crate::common::{test_part_one_common, test_part_two_common};
use crate::day15::Day15;

static INPUT_EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

#[test]
fn test_part_one() {
    test_part_one_common(Day15::default(), INPUT_EXAMPLE, 1320);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day15::default(), INPUT_EXAMPLE, 145);
}
