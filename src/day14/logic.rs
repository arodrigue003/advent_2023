use crate::day14::models::{Platform, Tile};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

static TARGET: usize = 1_000_000_000;

impl Platform {
    fn compute_load(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(i_line, line)| {
                (self.height - i_line) * line.iter().filter(|tile| **tile == Tile::Round).count()
            })
            .sum()
    }

    fn tilt_north(&mut self) -> &mut Self {
        // Iterate column by column, from the left to the right
        for i_column in 0..self.width {
            for i_line in 0..self.height {
                // We need to move the tile
                if self.grid[i_line][i_column] == Tile::Round {
                    let mut last_free = i_line;

                    // Move it in the column until we find an obstacle
                    for i_line_2 in (0..i_line).rev() {
                        if self.grid[i_line_2][i_column] == Tile::Empty {
                            // No obstacle, update the last_free pointer
                            last_free = i_line_2;
                        } else {
                            // Found an obstacle, break the loop
                            break;
                        }
                    }

                    // Perform the swap
                    self.grid[i_line][i_column] = Tile::Empty;
                    self.grid[last_free][i_column] = Tile::Round;
                }
            }
        }

        self
    }

    fn tilt_west(&mut self) -> &mut Self {
        // Iterate line by line, from top to bottom
        for i_line in 0..self.height {
            for i_column in 0..self.width {
                // We need to move the tile
                if self.grid[i_line][i_column] == Tile::Round {
                    let mut last_free = i_column;

                    // Move it in the line until we find an obstacle
                    for i_column_2 in (0..i_column).rev() {
                        if self.grid[i_line][i_column_2] == Tile::Empty {
                            // No obstacle, update the last_free pointer
                            last_free = i_column_2;
                        } else {
                            // Found an obstacle, break the loop
                            break;
                        }
                    }

                    // Perform the swap
                    self.grid[i_line][i_column] = Tile::Empty;
                    self.grid[i_line][last_free] = Tile::Round;
                }
            }
        }

        self
    }

    fn tilt_south(&mut self) -> &mut Self {
        // Iterate column by column in reverse, from the left to the right
        for i_column in 0..self.width {
            for i_line in (0..self.height).rev() {
                // We need to move the tile
                if self.grid[i_line][i_column] == Tile::Round {
                    let mut last_free = i_line;

                    // Move it in the column until we find an obstacle
                    for i_line_2 in i_line + 1..self.height {
                        if self.grid[i_line_2][i_column] == Tile::Empty {
                            // No obstacle, update the last_free pointer
                            last_free = i_line_2;
                        } else {
                            // Found an obstacle, break the loop
                            break;
                        }
                    }

                    // Perform the swap
                    self.grid[i_line][i_column] = Tile::Empty;
                    self.grid[last_free][i_column] = Tile::Round;
                }
            }
        }

        self
    }

    fn tilt_east(&mut self) -> &mut Self {
        // Iterate line by line, from top to bottom
        for i_line in 0..self.height {
            for i_column in (0..self.width).rev() {
                // We need to move the tile
                if self.grid[i_line][i_column] == Tile::Round {
                    let mut last_free = i_column;

                    // Move it in the line until we find an obstacle
                    for i_column_2 in i_column + 1..self.width {
                        if self.grid[i_line][i_column_2] == Tile::Empty {
                            // No obstacle, update the last_free pointer
                            last_free = i_column_2;
                        } else {
                            // Found an obstacle, break the loop
                            break;
                        }
                    }

                    // Perform the swap
                    self.grid[i_line][i_column] = Tile::Empty;
                    self.grid[i_line][last_free] = Tile::Round;
                }
            }
        }

        self
    }
}

pub fn solve_part_one(platform: &Platform) -> usize {
    platform.clone().tilt_north().compute_load()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn solve_part_two(platform: &Platform) -> usize {
    // Copy the data for the process to work
    let mut platform = platform.clone();

    // Store grid hash to increase loop speed
    let mut grid_results: HashMap<u64, usize> = HashMap::new();

    // Do cycles
    let mut cycle_count = 0;
    loop {
        // Compute platform hash
        let platform_hash = calculate_hash(&platform);

        // Check if we looped somehow
        if let Some(last_cycle_count) = grid_results.get(&platform_hash) {
            // Determinate how many steps to skip
            let cycle_skip = cycle_count - last_cycle_count;
            let skip_count = (TARGET - cycle_count) / cycle_skip;

            // Perform the skip
            cycle_count += skip_count * cycle_skip;

            // Break the loop in case we reached the end
            if cycle_count == TARGET {
                break;
            };
        }

        // Add the hash to the collection
        grid_results.insert(platform_hash, cycle_count);

        // Tilt the platform
        platform.tilt_north().tilt_west().tilt_south().tilt_east();

        // increase the cycle count
        cycle_count += 1;

        // termination check
        if cycle_count == TARGET {
            break;
        }
    }

    platform.compute_load()
}
