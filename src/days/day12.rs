use itertools::{enumerate, Product};

use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day12".to_string();
    let input = read_to_string(input_file).unwrap();

    let sol1: usize = solution1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Location {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Plot {
    location: Location,
    crop: char,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Region {
    locations: Vec<Location>,
    crop: char,
    area: usize,
    perimeter: usize,
}

impl Region {
    fn new(plot: Plot) -> Self {
        Region {
            locations: vec![plot.location],
            crop: plot.crop,
            area: 1,
            perimeter: 4,
        }
    }

    fn price(self) -> usize {
        self.area * self.perimeter
    }
}

fn solution1(input: &str) -> usize {
    let mut regions: Vec<Region> = Vec::new();

    regions.into_iter().map(|r| r.price()).sum()
}
