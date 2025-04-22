use itertools::{all, enumerate, Product};

use crate::{Solution, SolutionPair};
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::isize;
use std::ops::{Index, IndexMut};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day12".to_string();
    let input = read_to_string(input_file).unwrap();

    let sol1: (isize, isize) = solution1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1.0), Solution::from(sol1.1))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Location {
    x: isize,
    y: isize,
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
    area: isize,
    perimeter: isize,
    sides: isize,
}

impl Region {
    fn new(plot: Plot) -> Self {
        Region {
            locations: HashSet::from([plot.location]),
            crop: plot.crop,
            area: 1,
            perimeter: 4,
            sides: 4,
        }
    }

    fn price(self) -> isize {
        self.area * self.perimeter
    }

    fn dicount_price(self) -> isize {
        self.area * self.sides
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
                        found = true;
                        self.locations.insert(plot.location);
                        self.area += 1;
                        self.perimeter += (4 - (2 * neighbor_count));
                        self.sides +=
                            side_delta(enumerate_surrounding(&plot.location, &self.locations));
                        plots.remove(&plot);
                    }
                }
            }
        }
        self
    }

    fn sides(self) -> isize {
        0
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

fn enumerate_surrounding(loc: &Location, locations: &HashSet<Location>) -> Vec<isize> {
    let mut result: Vec<isize> = Vec::new();

    if locations.contains(&Location {
        x: loc.x - 1,
        y: loc.y - 1,
    }) {
        result.push(1);
    }
    if locations.contains(&Location {
        x: loc.x,
        y: loc.y - 1,
    }) {
        result.push(2);
    }
    if locations.contains(&Location {
        x: loc.x + 1,
        y: loc.y - 1,
    }) {
        result.push(3);
    }
    if locations.contains(&Location {
        x: loc.x - 1,
        y: loc.y,
    }) {
        result.push(8);
    }
    if locations.contains(&Location {
        x: loc.x + 1,
        y: loc.y,
    }) {
        result.push(4);
    }
    if locations.contains(&Location {
        x: loc.x - 1,
        y: loc.y + 1,
    }) {
        result.push(7);
    }
    if locations.contains(&Location {
        x: loc.x,
        y: loc.y + 1,
    }) {
        result.push(6);
    }
    if locations.contains(&Location {
        x: loc.x + 1,
        y: loc.y + 1,
    }) {
        result.push(5);
    }

    result
}

fn side_delta(mut suround: Vec<isize>) -> isize {
    if all([2, 3, 6, 8], |n| suround.contains(&n)) {
        return -4;
    }

    while !suround.contains(&2) || suround.contains(&8) {
        suround = rotate_ccw(suround);
    }

    if all([4, 6, 8], |n| !suround.contains(&n)) {
        if suround.contains(&1) && suround.contains(&3) {
            return 3;
        }

        if !suround.contains(&1) && !suround.contains(&3) {
            return 0;
        }

        return 2;
    }

    if suround.contains(&4) && suround.contains(&6) {
        if all([1, 7], |n| suround.contains(&n)) {
            return 0;
        }

        if all([1, 7], |n| !suround.contains(&n)) {
            return -4;
        }

        return -2;
    }

    if suround.contains(&4) {
        if suround.contains(&1) & suround.contains(&5) {
            return 2;
        }

        if !suround.contains(&1) && !suround.contains(&5) {
            return -2;
        }

        return 0;
    }

    if suround.contains(&6) {
        if all([1, 3, 7, 5], |n| suround.contains(&n)) {
            return 4;
        }

        if all([1, 3, 7, 5], |n| !suround.contains(&n)) {
            return -4;
        }

        let mut count = 0;
        for n in [1, 3, 5, 7] {
            if suround.contains(&n) {
                count += 1;
            }
        }

        if count == 3 {
            return 2;
        }

        if count == 1 {
            return -2;
        }

        return 0;
    }

    0
}

fn rotate_ccw(suround: Vec<isize>) -> Vec<isize> {
    let mut result: Vec<isize> = Vec::new();

    for i in suround {
        if i > 2 {
            result.push(i - 2);
        } else {
            result.push(i + 6);
        }
    }

    result
}

fn solution1(input: &str) -> (isize, isize) {
    let mut regions: Vec<Region> = Vec::new();

    let mut plots: HashSet<Plot> = HashSet::new();

    for (y, line) in enumerate(input.lines()) {
        for (x, crop) in enumerate(line.chars()) {
            plots.insert(Plot {
                location: Location {
                    x: x.try_into().expect("Conversion failed"),
                    y: y.try_into().expect("Conversion failed"),
                },
                crop,
            });
        }
    }

    while !plots.is_empty() {
        let plot = *plots.clone().iter().next().unwrap();
        plots.remove(&plot);
        let mut region = Region::new(plot);
        let region = region.grow(&mut plots);
        regions.push(region);
    }

    regions
        .into_iter()
        .map(|r| (r.clone().price(), r.dicount_price()))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .expect("Uh oh")
}
