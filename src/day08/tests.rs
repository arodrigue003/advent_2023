use crate::common::{test_part_one_common, test_part_two_common};
use crate::day08::Day08;

static INPUT_EXAMPLE_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

static INPUT_EXAMPLE_2: &str = "LR

AAA = (11B, 11B)
11B = (11C, 11C)
11C = (11D, 11D)
11D = (11E, 11E)
11E = (ZZZ, ZZZ)
ZZZ = (11B, 11B)
22A = (22B, 22B)
22B = (22C, 22C)
22C = (22D, 22D)
22D = (22Z, 22Z)
22Z = (22A, 22B)
33A = (33B, 33B)
33B = (33C, 33C)
33C = (33Z, 33Z)
33Z = (33B, 33B)
";

#[test]
fn test_part_one() {
    test_part_one_common(Day08::new(), INPUT_EXAMPLE_1, 6);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day08::new(), INPUT_EXAMPLE_2, 45);
}
