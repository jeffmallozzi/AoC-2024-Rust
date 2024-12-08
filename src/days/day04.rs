use crate::{Solution, SolutionPair};
use itertools::enumerate;
use regex::Regex;
use std::{arch::x86_64::_MM_EXCEPT_DENORM, char::REPLACEMENT_CHARACTER, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file = "input/day04".to_string();
    let input = read_to_string(input_file).expect("Not a file");

    let sol1: u64 = solution1(&input);
    let sol2: u64 = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solution1(input: &str) -> u64 {
    let mut found: u64 = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for (row_i, row) in enumerate(grid.clone()) {
        for (col_i, letter) in enumerate(row.clone()) {
            match letter {
                'X' => {
                    if col_i <= 136 {
                        if row[col_i + 1..col_i + 4] == ['M', 'A', 'S'] {
                            found += 1;
                        }
                    }
                    if col_i <= 136 && row_i <= 136 {
                        if grid[row_i + 1][col_i + 1] == 'M'
                            && grid[row_i + 2][col_i + 2] == 'A'
                            && grid[row_i + 3][col_i + 3] == 'S'
                        {
                            found += 1;
                        }
                    }
                    if col_i >= 3 && row_i <= 136 {
                        if grid[row_i + 1][col_i - 1] == 'M'
                            && grid[row_i + 2][col_i - 2] == 'A'
                            && grid[row_i + 3][col_i - 3] == 'S'
                        {
                            found += 1;
                        }
                    }
                    if row_i <= 136 {
                        if grid[row_i + 1][col_i] == 'M'
                            && grid[row_i + 2][col_i] == 'A'
                            && grid[row_i + 3][col_i] == 'S'
                        {
                            found += 1;
                        }
                    }
                }
                'S' => {
                    if col_i <= 136 {
                        if row[col_i + 1..col_i + 4] == ['A', 'M', 'X'] {
                            found += 1;
                        }
                    }
                    if col_i <= 136 && row_i <= 136 {
                        if grid[row_i + 1][col_i + 1] == 'A'
                            && grid[row_i + 2][col_i + 2] == 'M'
                            && grid[row_i + 3][col_i + 3] == 'X'
                        {
                            found += 1;
                        }
                    }
                    if col_i >= 3 && row_i <= 136 {
                        if grid[row_i + 1][col_i - 1] == 'A'
                            && grid[row_i + 2][col_i - 2] == 'M'
                            && grid[row_i + 3][col_i - 3] == 'X'
                        {
                            found += 1;
                        }
                    }
                    if row_i <= 136 {
                        if grid[row_i + 1][col_i] == 'A'
                            && grid[row_i + 2][col_i] == 'M'
                            && grid[row_i + 3][col_i] == 'X'
                        {
                            found += 1;
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    found
}

fn solution2(input: &str) -> u64 {
    let mut found: u64 = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect());
    }

    for (row_i, row) in enumerate(grid.clone()) {
        for (col_i, letter) in enumerate(row.clone()) {
            if row_i > 0 && row_i < 139 && col_i > 0 && col_i < 139 && letter == 'A' {
                let mut corners: Vec<char> = Vec::new();
                corners.push(grid[row_i - 1][col_i - 1]);
                corners.push(grid[row_i + 1][col_i + 1]);
                corners.push(grid[row_i - 1][col_i + 1]);
                corners.push(grid[row_i + 1][col_i - 1]);

                if (corners[0..2] == ['M', 'S'] || corners[0..2] == ['S', 'M'])
                    && (corners[2..4] == ['M', 'S'] || corners[2..4] == ['S', 'M'])
                {
                    found += 1;
                }
            }
        }
    }

    found
}
