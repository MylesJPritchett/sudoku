use crate::prelude::*;
use std::fs;

pub fn read_sudoku_puzzles(filename: &str) -> Result<Vec<Puzzle>> {
    let content = fs::read_to_string(filename).map_err(|e| Error::IO(e))?;

    let mut puzzles = Vec::new();
    let mut lines = content.lines();

    while let Some(line) = lines.next() {
        if line.starts_with("Grid") {
            let mut grid = [[0u8; 9]; 9];
            let name = line.to_string();

            for i in 0..9 {
                if let Some(row) = lines.next() {
                    let row_digits: Vec<u8> = row
                        .chars()
                        .map(|c| c.to_digit(10).unwrap_or(0) as u8)
                        .collect();

                    if row_digits.len() != 9 {
                        return Err(Error::Generic(format!(
                            "Row {} in puzzle {} doesn't have exactly 9 digits",
                            i + 1,
                            name
                        )));
                    }

                    grid[i].copy_from_slice(&row_digits);
                } else {
                    return Err(Error::Generic(format!(
                        "Not enough rows for puzzle {}",
                        name
                    )));
                }
            }

            // Create a SudokuBoard from the grid
            let board = SudokuBoard { grid };

            // Create a Puzzle with default variation and difficulty
            // These could be inferred or parsed from the name if needed
            let puzzle = Puzzle {
                name,
                variation: Variation::Standard, // Default
                difficulty: Difficulty::Medium, // Default
                board,
            };

            puzzles.push(puzzle);
        }
    }

    if puzzles.is_empty() {
        return Err(Error::Generic("No puzzles found in the file".to_string()));
    }

    Ok(puzzles)
}
