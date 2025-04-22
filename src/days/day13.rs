use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file = "input/day13".to_string();
    let input = read_to_string(input_file).expect("bad path");

    let sol1: usize = solution1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn solution1(input: &str) -> usize {
    for line in input.lines() {
        println!("{}", line);
    }

    0
}
