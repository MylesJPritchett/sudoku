use crate::prelude::*;

use std::time::Instant;
impl SudokuBoard {
    pub fn solve(&mut self) -> bool {
        let start_time = Instant::now();
        loop {
            let progress = self.fill_single_candidates();
            if self.is_solved() {
                let elapsed_time = start_time.elapsed();

                println!(
                    "Solved logically in {:.4} ms",
                    elapsed_time.as_secs_f64() * 1000.0
                );
                return true;
            }
            if !progress {
                break;
            }
        }
        let success = self.brute_force();

        let elapsed_time = start_time.elapsed();
        println!(
            "Solved with brute force in {:.4} ms",
            elapsed_time.as_secs_f64() * 1000.0
        );
        success
    }

    pub fn brute_force(&mut self) -> bool {
        if let Some((row, col)) = self.find_empty() {
            for num in 1..=9 {
                if self.is_valid(row, col, num) {
                    self.grid[row][col] = num;
                    if self.brute_force() {
                        return true;
                    }
                    self.grid[row][col] = 0
                }
            }
            false
        } else {
            true
        }
    }

    pub fn fill_single_candidates(&mut self) -> bool {
        let mut changed = false;
        loop {
            let mut progress = false;
            for row in 0..9 {
                for col in 0..9 {
                    if self.grid[row][col] == 0 {
                        let mut possible_values = vec![];

                        for num in 1..=9 {
                            if self.is_valid(row, col, num) {
                                possible_values.push(num);
                            }
                        }
                        if possible_values.len() == 1 {
                            self.grid[row][col] = possible_values[0];
                            progress = true;
                        }
                    }
                }
            }
            if !progress {
                break;
            }
            changed = true;
        }
        changed
    }

    fn is_solved(&self) -> bool {
        self.grid
            .iter()
            .all(|row| row.iter().all(|&cell| cell != 0))
    }

    fn is_valid(&self, row: usize, col: usize, num: u8) -> bool {
        if self.grid[row].contains(&num) {
            return false;
        }

        if self.grid.iter().any(|r| r[col] == num) {
            return false;
        }

        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in 0..3 {
            for c in 0..3 {
                if self.grid[box_row + r][box_col + c] == num {
                    return false;
                }
            }
        }
        true
    }

    fn find_empty(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }
}
