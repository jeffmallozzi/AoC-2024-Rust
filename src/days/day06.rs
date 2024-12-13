use core::hash;
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
    let sol2: usize = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
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
    visited: HashSet<Guard>,
    positions: HashSet<(usize, usize)>,
    guard: Guard,
    guarded: bool,
    in_a_loop: bool,
}

impl Lab {
    fn new() -> Lab {
        Lab {
            obsticles: HashSet::new(),
            visited: HashSet::new(),
            positions: HashSet::new(),
            guard: Guard::new(),
            guarded: true,
            in_a_loop: false,
        }
    }
    fn mosey(&mut self) {
        let next = self.next();
        if self.obsticles.contains(&next) {
            self.guard.turn();
            self.in_a_loop = !self.visited.insert(self.guard);
        } else {
            self.guard.position = next;
            if !self.positions.contains(&self.guard.position) {
                self.guarded = false;
            } else {
                self.in_a_loop = !self.visited.insert(self.guard);
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
                lab.visited.insert(lab.guard);
            } else if col == '#' {
                lab.obsticles.insert((row_i, col_i));
            }
            lab.positions.insert((row_i, col_i));
        }
    }

    while lab.guarded {
        lab.mosey();
    }

    let mut unique_positions: HashSet<(usize, usize)> = HashSet::new();
    for guard in lab.visited {
        unique_positions.insert(guard.position);
    }

    unique_positions.len()
}

fn solution2(input: &str) -> usize {
    let mut lab: Lab = Lab::new();
    let mut guard_start: (usize, usize) = (0, 0);
    let mut open_spots: HashSet<(usize, usize)> = HashSet::new();
    let mut count_loops: usize = 0;

    for (row_i, row) in enumerate(input.lines()) {
        for (col_i, col) in enumerate(row.chars()) {
            if col == '^' {
                guard_start = (row_i, col_i);
            } else if col == '#' {
                lab.obsticles.insert((row_i, col_i));
            } else {
                open_spots.insert((row_i, col_i));
            }
            lab.positions.insert((row_i, col_i));
        }
    }

    for new_obsticle in open_spots {
        lab.obsticles.insert(new_obsticle);
        lab.visited.drain();
        lab.guard.position = guard_start;
        lab.guard.direction = Direction::Up;
        lab.visited.insert(lab.guard);
        lab.guarded = true;
        lab.in_a_loop = false;

        while lab.guarded && !lab.in_a_loop {
            lab.mosey();
        }

        if lab.in_a_loop {
            count_loops += 1;
        }
        lab.obsticles.remove(&new_obsticle);
    }

    count_loops
}
