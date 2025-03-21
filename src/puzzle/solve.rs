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
            // progress |= self.hidden_pairs();
            // progress |= self.hidden_triples();
            progress |= self.x_wing();
            // progress |= self.y_wing();
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
        if success {
            println!(
                "Solved with brute force in {:.4} ms",
                elapsed_time.as_secs_f64() * 1000.0
            );
        } else {
            println!(
                "Could not solve in {:.4} ms",
                elapsed_time.as_secs_f64() * 1000.0
            );
        }
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
        let mut changed = false;

        // Iterate over all units (rows, columns, and boxes)
        for units in [
            &self.units.row_units,
            &self.units.col_units,
            &self.units.box_units,
        ] {
            for unit in units.iter() {
                // Collect cells with exactly two candidates
                let mut candidate_cells = Vec::new();
                for &(row, col) in unit {
                    if self.grid[row][col] == 0 && self.candidates[row][col].len() == 2 {
                        candidate_cells.push((row, col));
                    }
                }

                // Check for naked pairs
                for i in 0..candidate_cells.len() {
                    for j in i + 1..candidate_cells.len() {
                        let (row1, col1) = candidate_cells[i];
                        let (row2, col2) = candidate_cells[j];

                        // Ensure we are comparing two distinct cells
                        if (row1, col1) != (row2, col2) {
                            // Check if the two cells have the same candidates
                            if self.candidates[row1][col1] == self.candidates[row2][col2] {
                                let pair_values = self.candidates[row1][col1].clone();

                                // Remove the pair values from other cells in the unit
                                for &(r, c) in unit {
                                    if (r, c) != (row1, col1) && (r, c) != (row2, col2) {
                                        for &num in &pair_values {
                                            if self.candidates[r][c].remove(&num) {
                                                changed = true;
                                            }
                                        }
                                    }
                                }
                            }
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
        let mut changed = false;

        for units in [
            &self.units.row_units,
            &self.units.col_units,
            &self.units.box_units,
        ] {
            for unit in units.iter() {
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
                                let triplet_values: HashSet<u8> =
                                    self.candidates[row1][col1].clone();

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
        }

        if changed {
            self.increment_method_count("naked_triples");
        }
        changed
    }

    /// Hidden Pairs: Two numbers appear in exactly two places within a unit. Remove other candidates from those cells.
    pub fn hidden_pairs(&mut self) -> bool {
        let changed = self.hidden_subgroup(2);
        if changed {
            self.increment_method_count("hidden_pairs");
        }
        changed
    }

    /// Hidden Triples: Three numbers appear in exactly three places within a unit.
    pub fn hidden_triples(&mut self) -> bool {
        let changed = self.hidden_subgroup(3);
        if changed {
            self.increment_method_count("hidden_triples");
        }
        changed
    }

    /// General function for Hidden Pairs/Triples
    pub fn hidden_subgroup(&mut self, size: usize) -> bool {
        let mut changed = false;
        // Clone the units to avoid borrowing self while iterating
        let units = self.units.clone();

        for units in [units.row_units, units.col_units, units.box_units] {
            for unit in units.iter() {
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
        }
        changed
    }

    pub fn x_wing(&mut self) -> bool {
        let mut changed = false;

        for num in 1..=9 {
            // Check rows
            for row1 in 0..8 {
                for row2 in row1 + 1..9 {
                    let mut cols = Vec::new();
                    for col in 0..9 {
                        if self.grid[row1][col] == 0 && self.grid[row2][col] == 0 {
                            let candidates1 = &self.candidates[row1][col];
                            let candidates2 = &self.candidates[row2][col];

                            if candidates1.contains(&num) && candidates2.contains(&num) {
                                cols.push(col);
                            }
                        }
                    }

                    if cols.len() == 2 {
                        let (col1, col2) = (cols[0], cols[1]);

                        let mut valid = true;
                        for r in 0..9 {
                            if r != row1
                                && r != row2
                                && self.grid[r][col1] == 0
                                && self.candidates[r][col1].contains(&num)
                            {
                                valid = false;
                            }
                            if r != row1
                                && r != row2
                                && self.grid[r][col2] == 0
                                && self.candidates[r][col2].contains(&num)
                            {
                                valid = false;
                            }
                        }

                        if valid {
                            for r in 0..9 {
                                if r != row1 && r != row2 {
                                    if self.candidates[r][col1].remove(&num)
                                        || self.candidates[r][col2].remove(&num)
                                    {
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Check columns
            for col1 in 0..8 {
                for col2 in col1 + 1..9 {
                    let mut rows = Vec::new();
                    for row in 0..9 {
                        if self.grid[row][col1] == 0 && self.grid[row][col2] == 0 {
                            let candidates1 = &self.candidates[row][col1];
                            let candidates2 = &self.candidates[row][col2];

                            if candidates1.contains(&num) && candidates2.contains(&num) {
                                rows.push(row);
                            }
                        }
                    }

                    if rows.len() == 2 {
                        let (row1, row2) = (rows[0], rows[1]);

                        let mut valid = true;
                        for c in 0..9 {
                            if c != col1
                                && c != col2
                                && self.grid[row1][c] == 0
                                && self.candidates[row1][c].contains(&num)
                            {
                                valid = false;
                            }
                            if c != col1
                                && c != col2
                                && self.grid[row2][c] == 0
                                && self.candidates[row2][c].contains(&num)
                            {
                                valid = false;
                            }
                        }

                        if valid {
                            for c in 0..9 {
                                if c != col1 && c != col2 {
                                    if self.candidates[row1][c].remove(&num)
                                        || self.candidates[row2][c].remove(&num)
                                    {
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if changed {
            self.increment_method_count("x_wing");
        }

        changed
    }

    pub fn y_wing(&mut self) -> bool {
        let mut changed = false;

        let mut bivalue_cells = Vec::new();

        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 && self.candidates[row][col].len() == 2 {
                    let values: Vec<u8> = self.candidates[row][col].iter().cloned().collect();
                    bivalue_cells.push((row, col, values[0], values[1]));
                }
            }
        }

        for i in 0..bivalue_cells.len() {
            let (row1, col1, a, b) = bivalue_cells[i];

            for j in i + 1..bivalue_cells.len() {
                let (row2, col2, x, y) = bivalue_cells[j];

                if row1 == row2 || col1 == col2 || (row1 / 3 == row2 / 3 && col1 / 3 == col2 / 3) {
                    if (a == x || a == y) && (b != x && b != y) {
                        let pivot = a;
                        let wing1 = b;
                        let wing2 = if a == x { y } else { x };

                        for k in j + 1..bivalue_cells.len() {
                            let (row3, col3, c, d) = bivalue_cells[k];

                            if (row2 == row3
                                || col2 == col3
                                || (row2 / 3 == row3 / 3 && col2 / 3 == col3 / 3))
                                && (c == wing2 && d == pivot || d == wing2 && c == pivot)
                            {
                                let elimination_target = if c == pivot { d } else { c };

                                for r in 0..9 {
                                    for c in 0..9 {
                                        if (r, c) != (row1, col1)
                                            && (r, c) != (row2, col2)
                                            && (r, c) != (row3, col3)
                                        {
                                            if self.candidates[r][c].remove(&elimination_target) {
                                                changed = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if changed {
            self.increment_method_count("y_wing");
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
        if !self.is_valid(row, col, value) {
            println!("set_value is not valid");
            return;
        }
        self.grid[row][col] = value;
        self.candidates[row][col].clear(); // No candidates left

        // Same row
        for c in 0..9 {
            if c != col {
                self.candidates[row][c].remove(&value);
            }
        }

        // Same column
        for r in 0..9 {
            if r != row {
                self.candidates[r][col].remove(&value);
            }
        }

        // Same box
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in 0..3 {
            for c in 0..3 {
                let br = box_row + r;
                let bc = box_col + c;
                if (br != row || bc != col) {
                    self.candidates[br][bc].remove(&value);
                }
            }
        }
    }
}
