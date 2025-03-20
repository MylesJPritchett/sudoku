#![allow(unused)] // For beginning only.

use crate::prelude::*;

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

    sudoku_puzzle.solve();
    sudoku_puzzle.print();
    Ok(())
}
