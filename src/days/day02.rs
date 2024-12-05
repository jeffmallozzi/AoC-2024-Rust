use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::{collections::hash_map::IntoValues, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day02".to_string();
    let input: String = read_to_string(input_file).expect("Uh Oh");

    let sol: (i64, i64) = solution1(&input);

    (Solution::from(sol.0), Solution::from(sol.0 + sol.1))
}

fn is_safe(values: Vec<i64>) -> bool {
    let difs: Vec<i64> = values
        .iter()
        .tuple_windows()
        .map(|y: (&i64, &i64)| y.0 - y.1)
        .collect();
    let mut slope: i64 = 1;
    if difs[0] < 0 {
        slope = -1;
    }

    for dif in difs {
        if !((dif * slope) >= 1 && (dif * slope) <= 3) {
            return false;
        }
    }

    true
}

fn solution1(input: &str) -> (i64, i64) {
    let mut safe_reports: i64 = 0;
    let mut safe_with_dampener: i64 = 0;

    for line in input.lines() {
        let values: Vec<i64> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Parse  Error"))
            .collect();

        if is_safe(values.clone()) {
            safe_reports += 1;
        } else {
            for i in 0..values.len() {
                let mut dampened_values = values.clone();
                dampened_values.remove(i);
                if is_safe(dampened_values) {
                    safe_with_dampener += 1;
                    break;
                }
            }
        }
    }
    (safe_reports, safe_with_dampener)
}
