use std::time::{Instant, Duration};
use std::fs;
use std::error::Error;

mod bitset;
mod grid;

use grid::SudokuGrid;


pub fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut grid_str = String::new();
    let mut num_puzzles = 0;
    let mut total_duration = Duration::new(0, 0);

    for line in contents.lines() {
        if line == "-" {
            let num_newlines = grid_str.as_bytes().iter().filter(|&&c| c == b'\n').count();
            if num_newlines == 9 {
                num_puzzles += 1;
                let duration = parse_and_solve(&grid_str)?;
                total_duration += duration;
            }
            grid_str = String::new();
        } else {
            grid_str = grid_str + "\n" + line;
        }
    }

    let num_newlines = grid_str.as_bytes().iter().filter(|&&c| c == b'\n').count();
    if num_newlines == 9 {
        num_puzzles += 1;
        let duration = parse_and_solve(&grid_str)?;
        total_duration += duration;
    }

    println!("Solved {} sudoku puzzles from {} in {:?}.", num_puzzles, filename, total_duration);
    println!("It took an average of {:?} to solve each sudoku puzzle.", total_duration / num_puzzles);

    Ok(())
}

fn parse_and_solve(grid_str: &str) -> Result<Duration, &str> {
    let mut grid = SudokuGrid::from(grid_str.trim());
    let puzzle_text = format!("{}", grid);
    let start = Instant::now();
    grid.solve();
    let duration = start.elapsed();
    let solution_text = format!("{}", grid);
    if !grid.is_correct() {
        return Err("Solution is incorrect! Oh no!");
    }

    println!("{}", side_by_side(&puzzle_text, &solution_text));
    println!("Solved in {:?}\n", duration);
    Ok(duration)
}

fn side_by_side(puzzle_text: &str, solution_text: &str) -> String {
    let mut output = String::from("Puzzle                          Solution\n");
    for (p, l) in puzzle_text.lines().zip(solution_text.lines()) {
        output += &format!("{}   {}\n", p, l);
    };
    output
}
