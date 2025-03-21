use std::collections::{HashMap, HashSet};

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
    method_counts: HashMap<String, usize>,
    units: Units,
    candidates: Vec<Vec<HashSet<u8>>>,
}

#[derive(Clone)]
pub struct Units {
    row_units: Vec<HashSet<(usize, usize)>>,
    col_units: Vec<HashSet<(usize, usize)>>,
    box_units: Vec<HashSet<(usize, usize)>>,
}

impl Puzzle {
    pub fn print(&self) {
        println!("Puzzle: {}", self.name);
        self.board.print();
    }

    pub fn solve(&mut self) -> bool {
        self.board.solve()
    }
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        let units: Units = Self::compute_units();

        let mut board = Self {
            grid: [[0; 9]; 9], // Initializes an empty board
            method_counts: HashMap::new(),
            units,
            candidates: vec![vec![HashSet::new(); 9]; 9],
        };
        board.compute_candidates();
        board
    }

    fn from(grid: [[u8; 9]; 9]) -> SudokuBoard {
        let units = Self::compute_units();
        let mut board = Self {
            grid, // Initializes an empty board
            method_counts: HashMap::new(),
            units,
            candidates: vec![vec![HashSet::new(); 9]; 9],
        };
        board.compute_candidates();
        board
    }

    pub fn print(&self) {
        println!("Solved with {:?}", self.method_counts);
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

    fn increment_method_count(&mut self, method_name: &str) {
        *self
            .method_counts
            .entry(method_name.to_string())
            .or_insert(0) += 1;
    }

    fn compute_units() -> Units {
        let mut row_units = vec![HashSet::new(); 81];
        let mut col_units = vec![HashSet::new(); 81];
        let mut box_units = vec![HashSet::new(); 81];

        for row in 0..9 {
            for col in 0..9 {
                let index = row * 9 + col;

                // Row
                for c in 0..9 {
                    if c != col {
                        row_units[index].insert((row, c));
                    }
                }

                // Column
                for r in 0..9 {
                    if r != row {
                        col_units[index].insert((r, col));
                    }
                }

                // Box
                let box_row = (row / 3) * 3;
                let box_col = (col / 3) * 3;
                for r in 0..3 {
                    for c in 0..3 {
                        let br = box_row + r;
                        let bc = box_col + c;
                        if br != row || bc != col {
                            box_units[index].insert((br, bc));
                        }
                    }
                }
            }
        }

        Units {
            row_units,
            col_units,
            box_units,
        }
    }

    fn compute_candidates(&mut self) {
        self.candidates = vec![vec![HashSet::new(); 9]; 9];

        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    let mut possible = (1..=9).collect::<HashSet<_>>();
                    let index = row * 9 + col;

                    // Remove numbers already present in row
                    for &(r, c) in &self.units.row_units[index] {
                        possible.remove(&self.grid[r][c]);
                    }

                    // Remove numbers already present in column
                    for &(r, c) in &self.units.col_units[index] {
                        possible.remove(&self.grid[r][c]);
                    }

                    // Remove numbers already present in box
                    for &(r, c) in &self.units.box_units[index] {
                        possible.remove(&self.grid[r][c]);
                    }

                    self.candidates[row][col] = possible;
                }
            }
        }
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
