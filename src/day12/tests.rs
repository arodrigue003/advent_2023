use crate::common::{test_part_one_common, test_part_two_common};
use crate::day12::Day12;

static INPUT_EXAMPLE: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

#[test]
fn test_part_one() {
    test_part_one_common(Day12::default(), INPUT_EXAMPLE, 21);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day12::default(), INPUT_EXAMPLE, 525152);
}
