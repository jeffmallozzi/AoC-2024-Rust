use crate::{Solution, SolutionPair};
use regex::Regex;
use std::{fs::read_to_string, usize};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day03".to_string();
    let input = read_to_string(input_file).expect("No file!");

    let sol1: u64 = solution1(&input);
    let sol2: u64 = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

fn solution1(input: &str) -> u64 {
    let mut result: u64 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [mul1, mul2]) in re.captures_iter(input).map(|c| c.extract()) {
        result += mul1.parse::<u64>().expect("Eh!") * mul2.parse::<u64>().expect("Hey!");
    }

    result
}

fn solution2(input: &str) -> u64 {
    let mut result: u64 = 0;

    let mut split_by_dont: Vec<&str> = input.split("don't").collect();
    result += solution1(split_by_dont[0]);

    for segment in split_by_dont.split_off(1) {
        match segment.split_once("do") {
            Some(x) => result += solution1(x.1),
            None => continue,
        }
    }

    result
}
