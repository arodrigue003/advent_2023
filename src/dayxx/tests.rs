use crate::common::{test_part_one_common, test_part_two_common};
use crate::dayxx::DayXX;

static INPUT_EXAMPLE: &str = "";

#[test]
fn test_part_one() {
    test_part_one_common(DayXX::new(), INPUT_EXAMPLE, 0);
}

#[test]
fn test_part_two() {
    test_part_two_common(DayXX::new(), INPUT_EXAMPLE, 0);
}
