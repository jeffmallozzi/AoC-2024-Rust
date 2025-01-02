use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day11".to_string();
    let input = read_to_string(input_file).unwrap();

    let sol1: usize = solution2(&input, 5, 5);
    let sol2: usize = solution2(&input, 15, 5);

    (Solution::from(sol1), Solution::from(sol2))
}

fn blink(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return Vec::from([1]);
    }

    if stone.to_string().len() % 2 == 0 {
        return stone
            .to_string()
            .split_at_checked(stone.to_string().len() / 2)
            .map(|(a, b)| vec![a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()])
            .unwrap();
    }

    vec![stone * 2024]
}

fn blink_n(stone: usize, n: usize) -> Vec<usize> {
    let mut stones = vec![stone];

    for _ in 0..n {
        stones = stones.iter().flat_map(|s| blink(*s)).collect();
    }

    stones
}

fn solution1(input: &str) -> usize {
    let mut stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    for _ in 0..25 {
        stones = stones.iter().flat_map(|s| blink(*s)).collect();
    }

    stones.len()
}

fn solution2(input: &str, g: usize, n: usize) -> usize {
    let mut stones: HashMap<usize, usize> = HashMap::new();

    for stone in input.split_whitespace() {
        *stones.entry(stone.parse::<usize>().unwrap()).or_insert(0) += 1;
    }

    let mut five_blink_cache: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..g {
        let mut next_stones: HashMap<usize, usize> = HashMap::new();
        for (stone, count) in stones.iter() {
            let new_stones = five_blink_cache
                .entry(*stone)
                .or_insert(blink_n(*stone, n))
                .clone();

            for new_stone in new_stones {
                *next_stones.entry(new_stone).or_insert(0) += count;
            }
        }
        stones = next_stones;
    }

    stones.values().sum()
}
