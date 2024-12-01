use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::iter::zip;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day01.txt".to_string();
    let input = read_to_string(input_file).expect("String Error");

    //println!("{}", input);

    let sol: (i64, i64) = solution1(&input);
    let sol2: i64 = 0;

    (Solution::from(sol.0), Solution::from(sol.1))
}

fn solution1(input: &str) -> (i64, i64) {
    let mut dist_sum: i64 = 0;
    let mut similarity_score: i64 = 0;
    let mut left_list: Vec<i64> = Vec::new();
    let mut right_list: Vec<i64> = Vec::new();
    let mut right_counts: HashMap<i64, i64> = HashMap::new();

    for line in input.lines() {
        let line_nums: Vec<&str> = line.trim().split_whitespace().collect();
        left_list.push(line_nums[0].parse().expect("Not parseable"));
        right_list.push(line_nums[1].parse().expect("Not parsable"));
        *right_counts
            .entry(line_nums[1].parse().expect("Not parsable"))
            .or_insert(0) += 1;
    }

    left_list.sort();
    right_list.sort();

    for nums in left_list.iter().zip(right_list.iter()) {
        dist_sum += (nums.0 - nums.1).abs();
        similarity_score += *nums.0 * *right_counts.entry(*nums.0).or_insert(0);
    }

    (dist_sum, similarity_score)
}
