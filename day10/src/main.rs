use std::fs;

use advent_2023_common::get_args;
use day10::models::{Direction, Grid, Point};
use day10::parser::parse_data;

fn solve_part_one(data: &Grid) -> usize {
    data.get_loop().size / 2
}

fn solve_part_two(data: &Grid) -> i64 {
    let pipe_loop = data.get_loop();

    let mut inner_count = 0;
    for (i_line, line) in data.tiles.iter().enumerate() {
        let mut pointing_north = 0;
        let mut pointing_south = 0;

        for (i_column, tile) in line.iter().enumerate().skip(1) {
            let position = Point::new(i_line, i_column);

            if pipe_loop.loop_elements.contains(&position) {
                if tile.is_pointing(Direction::North, pipe_loop.start_tile) {
                    pointing_north += 1;
                }
                if tile.is_pointing(Direction::South, pipe_loop.start_tile) {
                    pointing_south += 1;
                }
            } else {
                if pointing_north % 2 == 1 && pointing_south % 2 == 1 {
                    inner_count += 1;
                }
            }
        }
    }

    inner_count
}

fn main() {
    let args = get_args();
    let data: String = fs::read_to_string(&args.path).unwrap();
    let parsed_data = parse_data(&data);

    if args.verbose {
        parsed_data.display_loop(&parsed_data.get_loop());
        println!("Starting position: {}", parsed_data.start);
    }

    println!("Part one solution: {}", solve_part_one(&parsed_data));
    println!("Part two solution: {}", solve_part_two(&parsed_data));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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
        assert_eq!(solve_part_one(&parse_data(INPUT_EXAMPLE_1)), 8);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE_1)), 1);
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE_2)), 4);
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE_3)), 8);
        assert_eq!(solve_part_two(&parse_data(INPUT_EXAMPLE_4)), 10);
    }
}
