use crate::common::{test_part_one_common, test_part_two_common};
use crate::day17::Day17;

static INPUT_EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

#[test]
fn test_part_one() {
    test_part_one_common(Day17::default(), INPUT_EXAMPLE, 102);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day17::default(), INPUT_EXAMPLE, 94);
}
