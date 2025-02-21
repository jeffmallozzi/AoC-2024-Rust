use itertools::{enumerate, Product};

use crate::{Solution, SolutionPair};
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

impl<T> HashSet<T>
where
    T: std::hash::Hash + Eq,
{
    pub fn pop(&mut self) -> Option<T> {
        if let Some(&item) = self.iter().next() {
            self.take(&item)
        } else {
            None
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    locations: HashSet<Location>,
    crop: char,
    area: usize,
    perimeter: usize,
}

impl Region {
    fn new(plot: Plot) -> Self {
        Region {
            locations: HashSet::from([plot.location]),
            crop: plot.crop,
            area: 1,
            perimeter: 4,
        }
    }

    fn price(self) -> usize {
        self.area * self.perimeter
    }

    fn grow(mut self, plots: &mut HashSet<Plot>) -> Self {
        let mut found: bool = true;

        while found {
            found = false;
            for plot in plots.clone() {
                if plot.crop == self.crop {
                    let mut neighbor_count = 0;
                    for location in self.locations.clone() {
                        if neighbors(location, plot.location) {
                            neighbor_count += 1;
                        }
                    }
                    if neighbor_count > 0 {
                        self.locations.insert(plot.location);
                        self.area += 1;
                        self.perimeter += (4 - (2 * neighbor_count));
                        plots.remove(&plot);
                    }
                }
            }
        }
        self
    }
}

fn neighbors(loc1: Location, loc2: Location) -> bool {
    if (loc1.x.abs_diff(loc2.x) == 1 && loc1.y == loc2.y) {
        return true;
    }
    if (loc1.y.abs_diff(loc2.y) == 1 && loc1.x == loc2.x) {
        return true;
    }
    false
}

fn solution1(input: &str) -> usize {
    let mut regions: Vec<Region> = Vec::new();

    let mut plots: HashSet<Plot> = HashSet::new();

    for (y, line) in enumerate(input.lines()) {
        for (x, crop) in enumerate(line.chars()) {
            plots.insert(Plot {
                location: Location { x, y },
                crop,
            });
        }
    }

    while !plots.is_empty() {
        let plot = plots.into_iter().collect().pop();
        let mut region = Region::new(plot);
        let region = region.grow(&mut plots);
        regions.push(region);
    }

    regions.into_iter().map(|r| r.price()).sum()
}
