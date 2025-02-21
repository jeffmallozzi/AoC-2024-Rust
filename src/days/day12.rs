use itertools::{enumerate, Product};

use crate::{Solution, SolutionPair};
use std::arch::x86_64::_MM_FROUND_TO_ZERO;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};

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

    fn grow(mut self, plots: &mut Vec<Plot>) -> Self {
        let mut found: bool = true;

        while found {
            found = false;
            for (i plot) in enumerate(plots) {
                if plot.crop == self.crop {
                    let neighborCount = 0;
                    for location in self.locations {
                        if neighbors(location, plot.location) {
                            neighborCount += 1;
                        }
                    }
                    if neighborCount > 0 {
                        self.area += 1;
                        self.perimeter += (4 - (2*neighborCount));
                        plots.swap_remove()
                    }
                }
            }
        }
        self
    }
}

fn solution1(input: &str) -> usize {
    let mut regions: Vec<Region> = Vec::new();

    let mut plots: Vec<Plot> = Vec::new();

    for (y, line) in enumerate(input.lines()) {
        for (x, crop) in enumerate(line.chars()) {
            plots.push(Plot {
                location: Location { x, y },
                crop,
            });
        }
    }

    while !plots.is_empty() {
        let plot = plots.pop().unwrap();
        let mut region = Region::new(plot);
        let region = region.grow(&mut plots);
        regions.push(region);
    }

    regions.into_iter().map(|r| r.price()).sum()
}
