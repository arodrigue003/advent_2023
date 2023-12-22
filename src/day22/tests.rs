use crate::common::{test_part_one_common, test_part_two_common};
use crate::day22::Day22;

static INPUT_EXAMPLE_1: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

static INPUT_EXAMPLE_2: &str = "0,0,1~0,1,1
1,1,1~1,1,1
0,0,2~0,0,2
0,1,2~1,1,2";

static INPUT_EXAMPLE_3: &str = "0,0,1~1,0,1
0,1,1~0,1,2
0,0,5~0,0,5
0,0,4~0,1,4";

#[test]
fn test_part_one() {
    test_part_one_common(Day22::default(), INPUT_EXAMPLE_1, 5);
    test_part_one_common(Day22::default(), INPUT_EXAMPLE_2, 3);
    test_part_one_common(Day22::default(), INPUT_EXAMPLE_3, 2);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day22::default(), INPUT_EXAMPLE_1, 7);
}
