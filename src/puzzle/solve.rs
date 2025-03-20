use crate::prelude::*;

impl SudokuBoard {
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
