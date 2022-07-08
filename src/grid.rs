use std::fmt;
use std::collections::HashMap;

use crate::bitset::BitSet;

struct GridNums {
    r: [BitSet; 9],
    c: [BitSet; 9],
    b: [BitSet; 9],
}

#[derive(Debug, PartialEq)]
pub struct SudokuGrid {
    grid: [[Option<u8>; 9]; 9]
}

struct SearchHistory {
    r: usize,
    c: usize,
    constraint: BitSet,
    value: u8,
    related_coords: Vec<(usize, usize)>
}

impl SudokuGrid {
    pub fn new() -> SudokuGrid {
        SudokuGrid { grid: [[None; 9]; 9] }
    }

    pub fn solve(&mut self) {
        let mut constraints = self.get_constraints();
        let mut history = Vec::new();
        let mut recent_history: Option<SearchHistory> = None;

        // Backtracking search
        loop {
            // No constraints left. All elements are filled.
            if constraints.len() == 0 {
                break;
            }

            // TODO: set starting num to be from recently popped history (if exist)
            let search_r;
            let search_c;
            let search_constraint;
            let starting_num;
            
            match &recent_history {
                Some(hist) => {
                    (search_r, search_c, search_constraint) = (hist.r, hist.c, hist.constraint);
                    starting_num = hist.value + 1;
                },
                None => {
                    // No recent history - start with the smallest constraint set
                    (search_r, search_c, search_constraint) = SudokuGrid::find_smallest_constraint(&constraints);
                    starting_num = 1;
                }
            }

            let mut found_next_iteration = false;
            for value in starting_num..10 {
                if search_constraint.contains(value) {
                    self.set(search_r, search_c, value).unwrap();
                    let constraint = constraints.remove(&(search_r, search_c)).unwrap();
                    let related_coords = self.reduce_related_constraints(search_r, search_c, value, &mut constraints);
                    history.push(SearchHistory {
                        r: search_r,
                        c: search_c,
                        constraint,
                        value,
                        related_coords
                    });
                    recent_history = None;
                    found_next_iteration = true;
                    // On to the next search iteration
                    break
                }
            }

            if found_next_iteration {
                continue;
            } else {
                // Exhausted possibilities with current configuration
                // Backtrack history and reinsert constraints
                recent_history = history.pop();
                match &recent_history {
                    Some(hist) => {
                        self.set(hist.r, hist.c, hist.value).unwrap();
                        constraints.insert((hist.r, hist.c), hist.constraint);
                        SudokuGrid::reinsert_related_constraints(&hist.related_coords, hist.value, &mut constraints);
                    },
                    None => panic!("There is no recent history to backtrack into."),
                }
            }
        }

    }

    fn find_smallest_constraint(constraints: &HashMap<(usize, usize), BitSet>) -> (usize, usize, BitSet) {
        let mut search_r = None;
        let mut search_c = None;
        let mut smallest_constraint: Option<BitSet> = None;

        for (&(row, col), &constraint) in constraints {
            match smallest_constraint {
                Some(c) => {
                    if constraint.len() < c.len() {
                        smallest_constraint = Some(constraint);
                        search_r = Some(row);
                        search_c = Some(col);
                    }
                },
                None => {
                    smallest_constraint = Some(constraint);
                    search_r = Some(row);
                    search_c = Some(col);
                }
            }
        }

        (search_r.unwrap(), search_c.unwrap(), smallest_constraint.unwrap())
    }

    fn reduce_related_constraints(
        &self, 
        row_num: usize, 
        col_num: usize, 
        value: u8, 
        constraints: &mut HashMap<(usize, usize), BitSet>
    ) -> Vec<(usize, usize)> {
        let mut related_coords = Vec::new();

        for j in 0..9 {
            if let Some(constraint) = constraints.get_mut(&(row_num, j)) {
                if constraint.contains(value) {
                    constraint.remove(value);
                    related_coords.push((row_num, j));
                }
            }
        }

        for i in 0..9 {
            if let Some(constraint) = constraints.get_mut(&(i, col_num)) {
                if constraint.contains(value) {
                    constraint.remove(value);
                    related_coords.push((i, col_num));
                }
            }
        }

        let box_r = row_num / 3 * 3;
        let box_c = col_num / 3 * 3;

        for i in box_r..box_r + 3 {
            for j in box_c..box_c + 3 {
                if let Some(constraint) = constraints.get_mut(&(i, j)) {
                    if constraint.contains(value) {
                        constraint.remove(value);
                        related_coords.push((i, j));
                    }
                }
            }
        }

        related_coords
    }

    fn reinsert_related_constraints(
        related_coords: &Vec<(usize, usize)>,
        value: u8,
        constraints: &mut HashMap<(usize, usize), BitSet>
    ) {
        for &(r, c) in related_coords {
            if let Some(constraint) = constraints.get_mut(&(r, c)) {
                constraint.insert(value);
            }
        }
    }

    pub fn from(grid_str: &str) -> SudokuGrid {
        let mut grid = SudokuGrid::new();
        for (row, row_str) in grid_str.lines().enumerate() {
            for (col, elem) in row_str.chars().enumerate() {
                if let Some(value) = elem.to_digit(10) {
                    if value > 0 {
                        grid.set(row, col, value as u8).unwrap();
                    }
                }
            }
        }
        if grid.has_conflicts() {
            panic!("Provided grid str has conflicts");
        }
        grid
    }

    fn get_constraints(&self) -> HashMap<(usize, usize), BitSet> {
        // Populate the constraints
        let grid_nums = self.get_grid_nums();
        let mut constraints = HashMap::new(); 
        let valid_mask = 1022;

        for i in 0..9 {
            for j in 0..9 {
                if let None = self.grid[i][j] {
                    let box_index = i / 3 + j / 3 * 3;
                    let mut conflicting_nums = BitSet::new();
                    conflicting_nums.extend(&grid_nums.r[i]);
                    conflicting_nums.extend(&grid_nums.c[j]);
                    conflicting_nums.extend(&grid_nums.b[box_index]);
                    conflicting_nums.flip(valid_mask);
                    let constraint = conflicting_nums;
                    constraints.insert((i, j), constraint);
                }
            }
        }

        constraints
    }

    fn get_grid_nums(&self) -> GridNums {
        let mut grid_nums = GridNums {
            r: [BitSet::new(); 9],
            c: [BitSet::new(); 9],
            b: [BitSet::new(); 9],
        };

        for i in 0..9 {
            for j in 0..9 {
                if let Some(value) = self.grid[i][j] {
                    let box_index = i / 3 + j / 3 * 3;
                    grid_nums.r[i].insert(value);
                    grid_nums.c[j].insert(value);
                    grid_nums.b[box_index].insert(value);
                }
            }
        }

        grid_nums
    }

    pub fn is_correct(&self) -> bool {
         for i in 0..9 {
            for j in 0..9 {
                if let None = self.grid[i][j] {
                    return false
                }
            }
        }

        !self.has_conflicts()
    }

    pub fn has_conflicts(&self) -> bool {
        let mut grid_nums = GridNums {
            r: [BitSet::new(); 9],
            c: [BitSet::new(); 9],
            b: [BitSet::new(); 9],
        };

        for i in 0..9 {
            for j in 0..9 {
                if let Some(value) = self.grid[i][j] {
                    let box_index = i / 3 + j / 3 * 3;
                    if grid_nums.r[i].contains(value) || grid_nums.c[j].contains(value) || grid_nums.b[box_index].contains(value) {
                        return true
                    }
                    grid_nums.r[i].insert(value);
                    grid_nums.c[j].insert(value);
                    grid_nums.b[box_index].insert(value);
                }
            }
        }

        false
    }

    pub fn set(&mut self, row_num: usize, col_num: usize, value: u8) -> Result<(), String> {
        if value < 1 || value > 9 as u8 {
            return Err(format!("Value {} is out of bounds. 1 <= value <= {}", value, 9));
        }

        if row_num >= 9 {
            return Err(format!("Row {} is out of bounds. Max row index is {}.", row_num, 9));
        }

        if col_num >= 9 {
            return Err(format!("Col {} is out of bounds. Max col index is {}.", col_num, 9));
        }

        self.grid[row_num][col_num] = Some(value);
        
        Ok(())
    }
    
}

impl fmt::Display for SudokuGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        for (i, row) in self.grid.iter().enumerate() {
            if i > 0 {
                write!(f, "\n")?;
                if i % 3 == 0 {
                    for j in 0..(9 / 3) {
                        if j > 0 {
                            write!(f, "+")?;
                        }
                        write!(f, "---------")?;
                    }
                    write!(f, "\n")?;
                }
            }
            
            for (j, &elem) in row.iter().enumerate() {
                if j > 0 && j % 3 == 0 {
                    write!(f, "|")?;
                }
                match elem {
                    Some(num) => write!(f, " {} ", num)?,
                    None => write!(f, " . ")?,
                }
            }
        }

        Ok(())
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_easy() {
        let puzzle_str = "\
...26.7.1
68..7..9.
19...45..
82.1...4.
..46.29..
.5...3.28
..93...74
.4..5..36
7.3.18...
";
        let solution_str = "\
435269781
682571493
197834562
826195347
374682915
951743628
519326874
248957136
763418259";

        let mut puzzle_grid = SudokuGrid::from(puzzle_str);
        let solution_grid = SudokuGrid::from(solution_str);

        puzzle_grid.solve();
        assert!(puzzle_grid.is_correct());
        assert_eq!(puzzle_grid, solution_grid);
    }

    #[test]
    fn solve_hard() {
        let puzzle_str = "\
...6..4..
7....36..
....91.8.
.........
.5.18...3
...3.6.45
.4.2...6.
9.3......
.2....1..
";
        let solution_str = "\
581672439
792843651
364591782
438957216
256184973
179326845
845219367
913768524
627435198";

        let mut puzzle_grid = SudokuGrid::from(puzzle_str);
        let solution_grid = SudokuGrid::from(solution_str);

        puzzle_grid.solve();
        assert!(puzzle_grid.is_correct());
        assert_eq!(puzzle_grid, solution_grid);
    }

    #[test]
    fn is_correct() {
        let solution_str = "\
435269781
682571493
197834562
826195347
374682915
951743628
519326874
248957136
763418259";

        let solution_grid = SudokuGrid::from(solution_str);
        assert!(solution_grid.is_correct());
    }

    #[test]
    fn unfilled_is_not_correct() {
        let unfilled_str = "\
...6..4..
7....36..
....91.8.
.........
.5.18...3
...3.6.45
.4.2...6.
9.3......
.2....1..";

        let unfilled_grid = SudokuGrid::from(unfilled_str);
        assert!(!unfilled_grid.is_correct());
    }

    #[test]
    #[should_panic]
    fn row_conflict() {
        let row_conflict_str = "\
435269718
682571493
197834562
826195347
374682915
951743628
519326874
248957136
763418259";

        SudokuGrid::from(row_conflict_str);
    }

    #[test]
    #[should_panic]
    fn col_conflict() {
        let col_conflict_str = "\
435269781
682571493
197834562
826195345
374682917
951743628
519326874
248957136
763418259";

        SudokuGrid::from(col_conflict_str);
    }

    #[test]
    #[should_panic]
    fn box_conflict() {
        let box_conflict_str = "\
581672439
792843651
438957216
364591782
256184973
179326845
845219367
913768524
627435198";

        SudokuGrid::from(box_conflict_str);
    }
}