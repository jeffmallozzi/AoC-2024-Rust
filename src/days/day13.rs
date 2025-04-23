use crate::{Solution, SolutionPair};
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashSet, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file = "input/day13".to_string();
    let input = read_to_string(input_file).expect("bad path");

    let sol1: usize = solution1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Combo {
    a: usize,
    b: usize,
}

impl Combo {
    fn tokens(self) -> usize {
        (self.a * 3) + self.b
    }
}

fn find_combos(a: usize, b: usize, value: usize) -> Option<HashSet<Combo>> {
    let mut combos: HashSet<Combo> = HashSet::new();

    for (presses_a, presses_b) in (1..(value / a)).cartesian_product(1..value / b) {
        if (a * presses_a) + (b * presses_b) == value {
            combos.insert(Combo {
                a: presses_a,
                b: presses_b,
            });
        }
    }

    if combos.is_empty() {
        return None;
    }

    Some(combos)
}

fn solution1(input: &str) -> usize {
    let mut total_tokens = 0;
    let mut ax = 0;
    let mut ay = 0;
    let mut bx = 0;
    let mut by = 0;
    let mut x = 0;
    let mut y = 0;

    let button_regex = Regex::new(r"Button [A,B]: X\+([0-9]+), Y\+([0-9]+) ");

    for line in input.lines() {
        if line.starts_with("Button A: ") {}
    }

    total_tokens
}
