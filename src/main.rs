#![allow(unused)] // For beginning only.

use crate::prelude::*;
use std::time::Instant;

mod error;
mod prelude;
mod puzzle;
mod utils;

fn main() -> Result<()> {
    println!("Hello, world!");
    let mut sudoku_puzzle = pick_random_puzzle("sudoku.txt").expect("couldnt pick random puzzle");
    // let mut sudoku_puzzle =
    //     pick_worst_backtracking("sudoku.txt").expect("couldnt pick worst backtracking puzzle");
    sudoku_puzzle.print();

    let start_time = Instant::now();
    sudoku_puzzle.solve();
    let elapsed_time = start_time.elapsed();
    println!("Solved in {:.4} microseconds", elapsed_time.as_micros());
    sudoku_puzzle.print();
    Ok(())
}
