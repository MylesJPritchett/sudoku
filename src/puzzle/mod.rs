use crate::prelude::*;
use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;

pub mod import;
pub mod solve;

#[derive(Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Extreme,
}

#[derive(Clone)]
pub enum Variation {
    Standard,
    Sandwhich,
    Killer,
}

#[derive(Clone)]
pub struct Puzzle {
    name: String,
    variation: Variation,
    difficulty: Difficulty,
    board: SudokuBoard,
}

#[derive(Clone)]
pub struct SudokuBoard {
    grid: [[u8; 9]; 9],
}

impl Puzzle {
    pub fn print(&self) {
        println!("Puzzle: {}", self.name);
        self.board.print();
    }

    pub fn solve(&mut self) {
        println!("Solving with Bruteforce");
        self.board.brute_force();
    }
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        SudokuBoard {
            grid: [[0; 9]; 9], // Initializes an empty board
        }
    }

    fn from(grid: [[u8; 9]; 9]) -> SudokuBoard {
        SudokuBoard { grid }
    }

    pub fn print(&self) {
        println!("┌───────┬───────┬───────┐");
        for (i, row) in self.grid.iter().enumerate() {
            print!("│ ");
            for (j, &cell) in row.iter().enumerate() {
                if cell == 0 {
                    print!("· ");
                } else {
                    print!("{} ", cell);
                }
                if (j + 1) % 3 == 0 && j < 8 {
                    print!("│ ");
                }
            }
            println!("│");
            if (i + 1) % 3 == 0 && i < 8 {
                println!("├───────┼───────┼───────┤");
            }
        }
        println!("└───────┴───────┴───────┘");
    }
}

pub fn pick_random_puzzle(filename: &str) -> Result<Puzzle> {
    let puzzles = read_sudoku_puzzles(filename)?;

    let mut rng = rand::rng();
    puzzles
        .choose(&mut rng)
        .cloned()
        .ok_or_else(|| Error::Generic("Failed to randomly select a puzzle".to_string()))
}

pub fn pick_worst_backtracking(filename: &str) -> Result<Puzzle> {
    let puzzles = read_sudoku_puzzles(filename)?;
    puzzles
        .into_iter()
        .find(|p| p.name == "Worst Case Backtrack")
        .ok_or_else(|| Error::Generic("Worst Case Backtrack puzzle not found".to_string()))
}
