use crate::day18::models::{DigPlan, Direction};
use itertools::assert_equal;
use std::collections::{HashSet, VecDeque};
use std::fmt::{Display, Formatter};

impl Direction {
    /// Return a tuple:
    ///  * The first element is equal to 1 if it was a left turn, 0 otherwise.
    ///  * The second element is equal to 1 if it was a right turn, 0 otherwise.
    pub fn direction(current: Direction, next: Direction) -> (usize, usize) {
        match (current, next) {
            (Direction::Up, Direction::Up)
            | (Direction::Right, Direction::Right)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Left) => (0, 0),
            (Direction::Up, Direction::Right)
            | (Direction::Right, Direction::Down)
            | (Direction::Down, Direction::Left)
            | (Direction::Left, Direction::Up) => (0, 1),
            (Direction::Up, Direction::Left)
            | (Direction::Right, Direction::Up)
            | (Direction::Down, Direction::Right)
            | (Direction::Left, Direction::Down) => (1, 0),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Tile {
    Empty,
    Border,
    Inner,
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".",
                Tile::Border => "#",
                Tile::Inner => "+",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Lagoon {
    border: Vec<(i32, i32)>,
    width: usize,
    height: usize,
    grid: Vec<Vec<Tile>>,
    flood_start: (usize, usize),
}

impl Lagoon {
    pub fn new(border: Vec<(i32, i32)>, is_turning_right: bool, start_direction: Direction) -> Self {
        // Check that the start of the border is equal to the end
        // TODO: we might also want to check that the border does not use the same tile more than once.
        // assert_eq!(border[0], border[border.len() - 1], "The border must form a loop");

        // Determinate position limits
        let min_line = border.iter().map(|point| point.0).min().unwrap();
        let max_line = border.iter().map(|point| point.0).max().unwrap();
        let min_column = border.iter().map(|point| point.1).min().unwrap();
        let max_column = border.iter().map(|point| point.1).max().unwrap();

        // Determinate grid size
        let width = (max_column - min_column + 1) as usize;
        let height = (max_line - min_line + 1) as usize;

        // Create a grid with the desired size
        let mut grid = vec![vec![Tile::Empty; width]; height];

        // Fill it with the border
        for border_element in &border {
            grid[(border_element.0 - min_line) as usize][(border_element.1 - min_column) as usize] = Tile::Border;
        }

        // Determinate the start position
        let first_border_elem = ((border[0].0 - min_line) as usize, (border[0].1 - min_column) as usize);
        let start = match (start_direction, is_turning_right) {
            (Direction::Up, true) => (first_border_elem.0, first_border_elem.1 + 1),
            (Direction::Up, false) => (first_border_elem.0, first_border_elem.1 - 1),
            (Direction::Right, true) => (first_border_elem.0 + 1, first_border_elem.1),
            (Direction::Right, false) => (first_border_elem.0 - 1, first_border_elem.1),
            (Direction::Down, true) => (first_border_elem.0, first_border_elem.1 - 1),
            (Direction::Down, false) => (first_border_elem.0, first_border_elem.1 + 1),
            (Direction::Left, true) => (first_border_elem.0 - 1, first_border_elem.1),
            (Direction::Left, false) => (first_border_elem.0 + 1, first_border_elem.1),
        };

        assert_ne!(grid[start.0][start.1], Tile::Border, "Flood start detection failed!!");

        // Create the struct
        Self {
            border,
            width,
            height,
            grid,
            flood_start: start,
        }
    }

    /// Flood the area and return the total number of flooded cell + the number of border cell
    pub fn flood(&mut self) -> usize {
        // Count how many cells we flooded
        let mut flood_count = 0;

        // To treat queue
        let mut queue = VecDeque::new();
        queue.push_back(self.flood_start);

        while let Some(to_treat) = queue.pop_front() {
            // If we pushed a tile that is not empty, ignore it
            if self.grid[to_treat.0][to_treat.1] != Tile::Empty {
                continue;
            }

            // Set the tile
            self.grid[to_treat.0][to_treat.1] = Tile::Inner;

            // Add one to the count
            flood_count += 1;

            // Add every neighbor to the queue.
            // Since we created a grid such as the border vec is adjacent to the border of the grid, we know that the
            // coordinates of neighbors of a tile that is inside the loop cannot go beyond the grid limit.
            queue.push_back((to_treat.0 + 1, to_treat.1));
            queue.push_back((to_treat.0 - 1, to_treat.1));
            queue.push_back((to_treat.0, to_treat.1 + 1));
            queue.push_back((to_treat.0, to_treat.1 - 1));
        }

        flood_count + self.border.len()
    }
}

impl Display for Lagoon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // Display the data
        for line in &self.grid {
            for tile in line {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub fn solve_part_one(dig_plan: &DigPlan) -> usize {
    // Initial element
    let mut last_position = (0, 0);
    let mut last_direction = dig_plan.instructions[0].direction;

    // Count the number of left and right turn
    let mut left_turns = 0;
    let mut right_turns = 0;

    let mut border = vec![];

    for instruction in &dig_plan.instructions {
        // Update the turn count
        let turn = Direction::direction(last_direction, instruction.direction);
        left_turns += turn.0;
        right_turns += turn.1;
        last_direction = instruction.direction.clone();

        for _ in 0..instruction.distance {
            match instruction.direction {
                Direction::Up => last_position = (last_position.0 - 1, last_position.1),
                Direction::Right => last_position = (last_position.0, last_position.1 + 1),
                Direction::Down => last_position = (last_position.0 + 1, last_position.1),
                Direction::Left => last_position = (last_position.0, last_position.1 - 1),
            }
            border.push(last_position.clone());
        }
    }

    let mut lagoon = Lagoon::new(border, right_turns > left_turns, dig_plan.instructions[0].direction);

    lagoon.flood()
}

pub fn solve_part_two(dig_plan: &DigPlan) -> u32 {
    0
}
