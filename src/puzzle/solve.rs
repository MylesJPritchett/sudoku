use crate::prelude::*;

use std::collections::HashSet;
use std::time::Instant;

impl SudokuBoard {
    pub fn solve(&mut self) -> bool {
        let start_time = Instant::now();
        loop {
            let mut progress = false;
            progress |= self.fill_single_candidates();
            progress |= self.naked_pairs();
            progress |= self.naked_triples();
            progress |= self.hidden_pairs();
            progress |= self.hidden_triples();
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
        println!("single");
        let mut changed = false;
        loop {
            let mut progress = false;
            for row in 0..9 {
                for col in 0..9 {
                    if self.grid[row][col] == 0 && self.candidates[row][col].len() == 1 {
                        let value = *self.candidates[row][col].iter().next().unwrap();
                        self.set_value(row, col, value);
                        progress = true;
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

    /// Naked Pairs: Finds two cells in a row, column, or box that have the same two candidates.
    /// Removes those candidates from other cells in the same unit.
    pub fn naked_pairs(&mut self) -> bool {
        println!("naked_pairs");
        let mut changed = false;

        for unit in &self.units {
            let mut candidates = Vec::new();

            for &(row, col) in unit {
                if self.grid[row][col] == 0 {
                    let possible = &self.candidates[row][col]; // Immutable borrow
                    if possible.len() == 2 {
                        candidates.push((row, col));
                    }
                }
            }

            for i in 0..candidates.len() {
                for j in i + 1..candidates.len() {
                    let (row1, col1) = candidates[i];
                    let (row2, col2) = candidates[j];

                    if self.candidates[row1][col1] == self.candidates[row2][col2] {
                        let pair_values: HashSet<u8> = self.candidates[row1][col1].clone();

                        // Avoid simultaneous mutable & immutable borrow using split_at_mut()
                        let (before, after) = self.candidates.split_at_mut(row1);
                        let (target_row, after) = after.split_at_mut(1);
                        let target = &mut target_row[0];

                        let mut updated = false;
                        for &(r, c) in unit {
                            if (r, c) != (row1, col1) && (r, c) != (row2, col2) {
                                for &num in &pair_values {
                                    if target[c].remove(&num) {
                                        updated = true;
                                    }
                                }
                            }
                        }
                        if updated {
                            changed = true;
                            println!("nakep pair update success")
                        }
                    }
                }
            }
        }

        if changed {
            self.increment_method_count("naked_pairs");
        }
        changed
    }

    /// Naked Triples: Same as Naked Pairs but with 3 cells.
    pub fn naked_triples(&mut self) -> bool {
        println!("naked_triples");
        let mut changed = false;

        for unit in &self.units {
            let mut candidates = Vec::new();

            for &(row, col) in unit {
                if self.grid[row][col] == 0 {
                    let possible = &self.candidates[row][col]; // Immutable borrow
                    if possible.len() == 3 {
                        candidates.push((row, col));
                    }
                }
            }

            for i in 0..candidates.len() {
                for j in i + 1..candidates.len() {
                    for k in j + 1..candidates.len() {
                        let (row1, col1) = candidates[i];
                        let (row2, col2) = candidates[j];
                        let (row3, col3) = candidates[k];

                        if self.candidates[row1][col1] == self.candidates[row2][col2]
                            && self.candidates[row2][col2] == self.candidates[row3][col3]
                        {
                            let triplet_values: HashSet<u8> = self.candidates[row1][col1].clone();

                            // Avoid simultaneous mutable & immutable borrow using split_at_mut()
                            let (before, after) = self.candidates.split_at_mut(row1);
                            let (target_row, after) = after.split_at_mut(1);
                            let target = &mut target_row[0];

                            let mut updated = false;
                            for &(r, c) in unit {
                                if (r, c) != (row1, col1)
                                    && (r, c) != (row2, col2)
                                    && (r, c) != (row3, col3)
                                {
                                    for &num in &triplet_values {
                                        if target[c].remove(&num) {
                                            updated = true;
                                        }
                                    }
                                }
                            }
                            if updated {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        if changed {
            self.increment_method_count("naked_triples");
        }
        changed
    }

    /// Hidden Pairs: Two numbers appear in exactly two places within a unit. Remove other candidates from those cells.
    pub fn hidden_pairs(&mut self) -> bool {
        println!("hidden_pairs");
        let changed = self.hidden_subgroup(2);
        if changed {
            self.increment_method_count("hidden_pairs");
        }
        changed
    }

    /// Hidden Triples: Three numbers appear in exactly three places within a unit.
    pub fn hidden_triples(&mut self) -> bool {
        println!("hidden_triples");
        let changed = self.hidden_subgroup(3);
        if changed {
            self.increment_method_count("hidden_triples");
        }
        changed
    }

    /// General function for Hidden Pairs/Triples
    pub fn hidden_subgroup(&mut self, size: usize) -> bool {
        println!("hidden_subgroup");
        let mut changed = false;
        // Clone the units to avoid borrowing self while iterating
        let units = self.units.clone();
        for unit in &units {
            let mut candidate_map: std::collections::HashMap<u8, Vec<(usize, usize)>> =
                std::collections::HashMap::new();
            for &(row, col) in unit {
                if self.grid[row][col] == 0 {
                    for &num in &self.candidates[row][col] {
                        candidate_map.entry(num).or_default().push((row, col));
                    }
                }
            }
            let candidate_entries: Vec<_> = candidate_map.into_iter().collect();
            for i in 0..candidate_entries.len() {
                for j in i + 1..candidate_entries.len() {
                    if size == 2
                        && candidate_entries[i].1.len() == 2
                        && candidate_entries[j].1.len() == 2
                    {
                        let set1: std::collections::HashSet<(usize, usize)> =
                            candidate_entries[i].1.iter().cloned().collect();
                        let set2: std::collections::HashSet<(usize, usize)> =
                            candidate_entries[j].1.iter().cloned().collect();
                        if set1 == set2 {
                            let mut updated = false;
                            for (row, col) in &set1 {
                                updated |= self.remove_all_candidates_except(
                                    *row,
                                    *col,
                                    &[candidate_entries[i].0, candidate_entries[j].0],
                                );
                            }
                            if updated {
                                changed = true;
                            }
                        }
                    }
                    for k in j + 1..candidate_entries.len() {
                        if size == 3
                            && candidate_entries[i].1.len() == 3
                            && candidate_entries[j].1.len() == 3
                        {
                            let set1: std::collections::HashSet<(usize, usize)> =
                                candidate_entries[i].1.iter().cloned().collect();
                            let set2: std::collections::HashSet<(usize, usize)> =
                                candidate_entries[j].1.iter().cloned().collect();
                            let set3: std::collections::HashSet<(usize, usize)> =
                                candidate_entries[k].1.iter().cloned().collect();
                            if set1 == set2 && set2 == set3 {
                                let mut updated = false;
                                for (row, col) in &set1 {
                                    updated |= self.remove_all_candidates_except(
                                        *row,
                                        *col,
                                        &[candidate_entries[i].0, candidate_entries[j].0],
                                    );
                                }
                                if updated {
                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }
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

    fn remove_all_candidates_except(&mut self, row: usize, col: usize, allowed: &[u8]) -> bool {
        println!("remove_all_candidates_except");
        if self.grid[row][col] != 0 {
            return false;
        }

        let allowed_set: HashSet<u8> = allowed.iter().cloned().collect();
        let candidates = &mut self.candidates[row][col]; // Mutable borrow of precomputed candidates

        // Check if we are actually removing something
        let before_size = candidates.len();
        candidates.retain(|&x| allowed_set.contains(&x));
        let after_size = candidates.len();

        if after_size < before_size {
            if after_size == 1 {
                self.grid[row][col] = *candidates.iter().next().unwrap();
            }
            return true; // We actually removed something
        }

        false
    }

    fn set_value(&mut self, row: usize, col: usize, value: u8) {
        println!("set_value");
        self.grid[row][col] = value;
        self.candidates[row][col].clear(); // No candidates left

        // Remove `value` from the candidates of all affected cells
        for &(r, c) in &self.units[row * 9 + col] {
            self.candidates[r][c].remove(&value);
        }
    }
}
