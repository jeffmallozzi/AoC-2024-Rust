use itertools::enumerate;

use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day05".to_string();
    let input = read_to_string(input_file).expect("Hey!");

    let sol: (u64, u64) = solution1(&input);

    (Solution::from(sol.0), Solution::from(sol.1))
}

fn middle_page(pages: Vec<&str>) -> u64 {
    pages[pages.len() / 2].parse::<u64>().expect("Error!")
}

fn sort_func(a: &str, b: &str, rules: HashMap<&str, HashSet<&str>>) -> std::cmp::Ordering {
    if rules.clone().entry(a).or_insert(HashSet::new()).contains(b) {
        return std::cmp::Ordering::Less;
    }

    if rules.clone().entry(b).or_insert(HashSet::new()).contains(a) {
        return std::cmp::Ordering::Greater;
    }

    std::cmp::Ordering::Equal
}

fn solution1(input: &str) -> (u64, u64) {
    let mut rules: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut manuals: Vec<Vec<&str>> = Vec::new();
    let mut middle_sum: u64 = 0;
    let mut incorrect_sum: u64 = 0;

    for line in input.lines() {
        if line.contains("|") {
            let (first, after) = line.split_once("|").expect("No | ");
            rules
                .entry(first)
                .or_insert(HashSet::new())
                .insert(after.trim());
        }

        if line.contains(",") {
            manuals.push(line.trim().split(",").collect());
        }
    }

    for mut manual in manuals {
        let mut valid: bool = true;
        for (page_i, page) in enumerate(manual.clone()) {
            let before_pages = HashSet::from_iter(manual.clone()[..page_i].iter().map(|p| *p));
            if rules
                .entry(page)
                .or_insert(HashSet::new())
                .intersection(&before_pages)
                .collect::<Vec<_>>()
                .len()
                > 0
            {
                valid = false;
                manual.sort_by(|a, b| sort_func(a, b, rules.clone()));
            }
        }

        if valid {
            middle_sum += middle_page(manual);
        } else {
            incorrect_sum += middle_page(manual)
        }
    }

    (middle_sum, incorrect_sum)
}
