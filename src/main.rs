#![allow(unused)] // For beginning only.

use crate::prelude::*;

use std::time::Instant;

mod error;
mod prelude;
mod puzzle;
mod utils;

fn main() -> Result<()> {
    let (successes, failures) = solve_all_puzzles("sudoku.txt")?;
    println!("Solved: {}, Failed: {}", successes, failures);
    Ok(())
}

pub fn solve_all_puzzles(filename: &str) -> Result<(usize, usize)> {
    let start_time = Instant::now();
    let puzzles = read_sudoku_puzzles(filename)?;
    let mut success_count = 0;
    let mut failure_count = 0;

    for mut puzzle in puzzles {
        if puzzle.solve() {
            success_count += 1;
        } else {
            failure_count += 1;
        }
    }
    let elapsed_time = start_time.elapsed();

    println!(
        "All Puzzles in {:.4} ms",
        elapsed_time.as_secs_f64() * 1000.0
    );

    Ok((success_count, failure_count))
}
