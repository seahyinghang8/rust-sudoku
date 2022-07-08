# Sudoku Solver in Rust

This is a fast Sudoku solver written in Rust as a form of practice. Unlike conventional backtracking search, this solver is written with a loop. Optimization techniques such as constraint propagation and variable ordering are used to reduce the search domain.

## Get Started

To run the Sudoku solver on a bunch of puzzles, just run
```bash
cargo run puzzles/test.txt
```

And you will see the following output

```bash
Puzzle                          Solution
 .  .  3 | .  2  . | 6  .  .     4  8  3 | 9  2  1 | 6  5  7 
 9  .  . | 3  .  5 | .  .  1     9  6  7 | 3  4  5 | 8  2  1 
 .  .  1 | 8  .  6 | 4  .  .     2  5  1 | 8  7  6 | 4  9  3 
---------+---------+---------   ---------+---------+---------
 .  .  8 | 1  .  2 | 9  .  .     5  4  8 | 1  3  2 | 9  7  6 
 7  .  . | .  .  . | .  .  8     7  2  9 | 5  6  4 | 1  3  8 
 .  .  6 | 7  .  8 | 2  .  .     1  3  6 | 7  9  8 | 2  4  5 
---------+---------+---------   ---------+---------+---------
 .  .  2 | 6  .  9 | 5  .  .     3  7  2 | 6  8  9 | 5  1  4 
 8  .  . | 2  .  3 | .  .  9     8  1  4 | 2  5  3 | 7  6  9 
 .  .  5 | .  1  . | 3  .  .     6  9  5 | 4  1  7 | 3  8  2 

Solved in 1.367225ms

Puzzle                          Solution
 2  .  . | .  8  . | 3  .  .     2  4  5 | 9  8  1 | 3  7  6 
 .  6  . | .  7  . | .  8  4     1  6  9 | 2  7  3 | 5  8  4 
 .  3  . | 5  .  . | 2  .  9     8  3  7 | 5  6  4 | 2  1  9 
---------+---------+---------   ---------+---------+---------
 .  .  . | 1  .  5 | 4  .  8     9  7  6 | 1  2  5 | 4  3  8 
 .  .  . | .  .  . | .  .  .     5  1  3 | 4  9  8 | 6  2  7 
 4  .  2 | 7  .  6 | .  .  .     4  8  2 | 7  3  6 | 9  5  1 
---------+---------+---------   ---------+---------+---------
 3  .  1 | .  .  7 | .  4  .     3  9  1 | 6  5  7 | 8  4  2 
 7  2  . | .  4  . | .  6  .     7  2  8 | 3  4  9 | 1  6  5 
 .  .  4 | .  1  . | .  .  3     6  5  4 | 8  1  2 | 7  9  3 

Solved in 1.312919ms

Puzzle                          Solution
 .  .  . | .  .  . | 9  .  7     4  6  2 | 8  3  1 | 9  5  7 
 .  .  . | 4  2  . | 1  8  .     7  9  5 | 4  2  6 | 1  8  3 
 .  .  . | 7  .  5 | .  2  6     3  8  1 | 7  9  5 | 4  2  6 
---------+---------+---------   ---------+---------+---------
 1  .  . | 9  .  4 | .  .  .     1  7  3 | 9  8  4 | 2  6  5 
 .  5  . | .  .  . | .  4  .     6  5  9 | 3  1  2 | 7  4  8 
 .  .  . | 5  .  7 | .  .  9     2  4  8 | 5  6  7 | 3  1  9 
---------+---------+---------   ---------+---------+---------
 9  2  . | 1  .  8 | .  .  .     9  2  6 | 1  7  8 | 5  3  4 
 .  3  4 | .  5  9 | .  .  .     8  3  4 | 2  5  9 | 6  7  1 
 5  .  7 | .  .  . | .  .  .     5  1  7 | 6  4  3 | 8  9  2 

Solved in 2.875243ms

Solved 3 sudoku puzzles from puzzles/test.txt in 5.555387ms.
It took an average of 1.851795ms to solve each sudoku puzzle.
```
The output will print out each puzzle and its solution, followed by some statistics.

## Example Puzzles
More puzzles are found in the puzzles/ folder. 

## Tests
Some tests are written but they are far from extensive. :P