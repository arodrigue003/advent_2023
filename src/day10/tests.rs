use crate::common::{test_part_one_common, test_part_two_common};
use crate::day10::Day10;

static INPUT_EXAMPLE_1: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

static INPUT_EXAMPLE_2: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

static INPUT_EXAMPLE_3: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

static INPUT_EXAMPLE_4: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

#[test]
fn test_part_one() {
    test_part_one_common(Day10::default(), INPUT_EXAMPLE_1, 8);
}

#[test]
fn test_part_two() {
    test_part_two_common(Day10::default(), INPUT_EXAMPLE_1, 1);
    test_part_two_common(Day10::default(), INPUT_EXAMPLE_2, 4);
    test_part_two_common(Day10::default(), INPUT_EXAMPLE_3, 8);
    test_part_two_common(Day10::default(), INPUT_EXAMPLE_4, 10);
}
