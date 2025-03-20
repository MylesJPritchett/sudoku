#![allow(unused)] // For beginning only.

use crate::prelude::*;

mod error;
mod prelude;
mod puzzle;
mod utils;

fn main() -> Result<()> {
    println!("Hello, world!");
    let sudoku_puzzle = pick_random_puzzle("sudoku.txt").expect("couldnt pick random puzzle");
    sudoku_puzzle.print();
    Ok(())
}
