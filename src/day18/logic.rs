use crate::day18::models::{DigPlan, Direction};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum LineType {
    Vertical,
    Horizontal,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Line {
    start: (i64, i64),
    end: (i64, i64),
    line_type: LineType,
    /// 0: line start goes up, 1: line start goes down
    start_directions: (bool, bool),
    /// 0: line end goes up, 1: line end goes down
    end_directions: (bool, bool),
}

impl Line {
    pub fn new(
        mut start: (i64, i64),
        mut end: (i64, i64),
        mut start_direction: Direction,
        direction: Direction,
        mut end_direction: Direction,
    ) -> Self {
        // Determinate the type of line
        let line_type = match direction {
            Direction::Up | Direction::Down => LineType::Vertical,
            Direction::Right | Direction::Left => LineType::Horizontal,
        };

        // Be sure that start is before end
        if start.0 > end.0 || start.1 > end.1 {
            std::mem::swap(&mut start, &mut end);
            let temp = start_direction;
            start_direction = end_direction.opposite();
            end_direction = temp.opposite();
        }

        // Compute start end end direction if the line is horizontal
        let start_directions = if line_type == LineType::Horizontal {
            match start_direction {
                Direction::Up => (false, true),
                Direction::Down => (true, false),
                _ => unreachable!(),
            }
        } else {
            (true, false)
        };
        let end_directions = if line_type == LineType::Horizontal {
            match end_direction {
                Direction::Up => (true, false),
                Direction::Down => (false, true),
                _ => unreachable!(),
            }
        } else {
            (false, true)
        };

        Self {
            start,
            end,
            line_type,
            start_directions,
            end_directions,
        }
    }

    fn intersect_line(&self, line: i64) -> bool {
        match self.line_type {
            LineType::Vertical => self.start.0 < line && self.end.0 > line,
            LineType::Horizontal => self.start.0 == line,
        }
    }

    fn len(&self) -> i64 {
        match self.line_type {
            LineType::Vertical => self.end.0 - self.start.0,
            LineType::Horizontal => self.end.1 - self.start.1,
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?}->{:?}) type: {:?}, start_direction: {:?}, end_direction: {:?}",
            self.start, self.end, self.line_type, self.start_directions, self.end_directions
        )
    }
}

struct Lagoon {
    pub lines: Vec<Line>,
}

impl Lagoon {
    fn flood(&self) -> i64 {
        // Get the list of line of interest thanks to the line of every line. Remove duplicates and sort it
        let break_lines: Vec<_> = self.lines.iter().flat_map(|line| [line.start.0, line.end.0]).collect();
        let mut break_lines: Vec<_> = break_lines.into_iter().unique().collect();
        break_lines.sort();

        let mut total_tiles = 0;

        // For each pair of break point, we may have two lines to consider:
        //  * the line of the break point
        //  * the line after it, if we have at least two empty lines.
        // We don't have to consider the final line because we know by construction that no inner tile can be found here
        for (start_line, end_line) in break_lines.iter().copied().tuple_windows() {
            // Handle the start line
            let line_inner_tiles = self.get_line_inner_tile_count(start_line);
            total_tiles += line_inner_tiles;

            // handle every other lines
            if end_line - start_line > 1 {
                let line_inner_tiles = self.get_line_inner_tile_count(start_line + 1);
                total_tiles += line_inner_tiles * (end_line - start_line - 1);
            }
        }

        total_tiles + self.lines.iter().map(Line::len).sum::<i64>()
    }

    fn get_line_inner_tile_count(&self, i_line: i64) -> i64 {
        let mut total_tiles = 0;

        // First, determinate the intersection of the lines with the line
        let mut intersections: Vec<_> = self.lines.iter().filter(|line| line.intersect_line(i_line)).collect();

        // Order them by column
        intersections.sort_by_cached_key(|line| line.start.1);

        // Count how many lines goes up and how many lines goes down
        let mut up_count = 0;
        let mut down_count = 0;

        // Iterate over the intersection
        for (current_line, next_line) in intersections.iter().tuple_windows() {
            up_count += current_line.start_directions.0 as i32 + current_line.end_directions.0 as i32;
            down_count += current_line.start_directions.1 as i32 + current_line.end_directions.1 as i32;

            // If we are inside, add what we can to the result
            if up_count % 2 == 1 && down_count % 2 == 1 {
                total_tiles += next_line.start.1 - current_line.end.1 - 1;
            }
        }

        total_tiles
    }
}

pub fn flood_dig_plan(dig_plan: &DigPlan) -> i64 {
    // Line storage
    let mut lines = vec![];

    // Start element
    let mut start_position = (0, 0);
    let mut start_direction = dig_plan.instructions[dig_plan.instructions.len() - 1].direction;

    // iterate over instruction
    for (current_instruction, next_instruction) in dig_plan.instructions.iter().circular_tuple_windows() {
        // Compute end position
        let end_position = match current_instruction.direction {
            Direction::Up => (start_position.0 - current_instruction.distance, start_position.1),
            Direction::Right => (start_position.0, start_position.1 + current_instruction.distance),
            Direction::Down => (start_position.0 + current_instruction.distance, start_position.1),
            Direction::Left => (start_position.0, start_position.1 - current_instruction.distance),
        };

        // Create the associated line
        lines.push(Line::new(
            start_position,
            end_position,
            start_direction,
            current_instruction.direction,
            next_instruction.direction,
        ));

        // Update start elements
        start_position = end_position;
        start_direction = current_instruction.direction;
    }

    Lagoon { lines }.flood()
}
