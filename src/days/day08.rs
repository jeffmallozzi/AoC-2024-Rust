use itertools::{enumerate, Combinations, Itertools};

use crate::{Solution, SolutionPair};
use std::char;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day08".to_string();
    let input = read_to_string(input_file).expect("Hey!");

    let sol1: usize = solution1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn get_antinodes(pair: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    assert_eq!(pair.len(), 2);

    let ax = pair[0].0;
    let ay = pair[0].1;
    let bx = pair[1].0;
    let by = pair[1].1;

    let difx = bx - ax;
    let dify = by - ay;

    vec![(bx + difx, by + dify), (ax - difx, ay - dify)]
}

fn solution1(input: &str) -> usize {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    let mut locations: HashSet<(usize, usize)> = HashSet::new();

    for (row_i, row) in enumerate(input.lines()) {
        for (col_i, col) in enumerate(row.chars()) {
            locations.insert((row_i, col_i));
            if col != '.' {
                antennas
                    .entry(col)
                    .or_insert(Vec::new())
                    .push((row_i, col_i));
            }
        }
    }

    for (frequency, spots) in antennas {
        for pair in spots.into_iter().combinations(2) {
            for point in get_antinodes(pair) {
                if locations.contains(&point) {
                    antinodes.insert(point);
                }
            }
        }
    }

    antinodes.len()
}
