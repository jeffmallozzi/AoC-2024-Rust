use itertools::enumerate;
use std::collections::HashSet;

use crate::{Solution, SolutionPair};
use std::{fs::read_to_string, sync::atomic::AtomicUsize};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day06".to_string();
    let input: String = read_to_string(input_file).expect("Hey!");

    let sol1: usize = solution1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Guard {
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn new() -> Guard {
        Guard {
            position: (0, 0),
            direction: Direction::Up,
        }
    }

    fn turn(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }
}

struct Lab {
    obsticles: HashSet<(usize, usize)>,
    visited: HashSet<(usize, usize)>,
    positions: HashSet<(usize, usize)>,
    guard: Guard,
    guarded: bool,
}

impl Lab {
    fn new() -> Lab {
        Lab {
            obsticles: HashSet::new(),
            visited: HashSet::new(),
            positions: HashSet::new(),
            guard: Guard::new(),
            guarded: true,
        }
    }
    fn mosey(&mut self) {
        let next = self.next();
        if self.obsticles.contains(&next) {
            self.guard.turn();
        } else {
            self.guard.position = next;
            if !self.positions.contains(&self.guard.position) {
                self.guarded = false;
            } else {
                self.visited.insert(self.guard.position);
            }
        }
    }

    fn next(&mut self) -> (usize, usize) {
        match self.guard.direction {
            Direction::Up => (self.guard.position.0 - 1, self.guard.position.1),
            Direction::Right => (self.guard.position.0, self.guard.position.1 + 1),
            Direction::Down => (self.guard.position.0 + 1, self.guard.position.1),
            Direction::Left => (self.guard.position.0, self.guard.position.1 - 1),
        }
    }
}

fn solution1(input: &str) -> usize {
    let mut lab: Lab = Lab::new();

    for (row_i, row) in enumerate(input.lines()) {
        for (col_i, col) in enumerate(row.chars()) {
            if col == '^' {
                lab.guard.position = (row_i, col_i);
                lab.visited.insert(lab.guard.position);
            } else if col == '#' {
                lab.obsticles.insert((row_i, col_i));
            }
            lab.positions.insert((row_i, col_i));
        }
    }

    while lab.guarded {
        lab.mosey();
    }

    lab.visited.len()
}
