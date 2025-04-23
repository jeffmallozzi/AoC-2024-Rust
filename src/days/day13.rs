use crate::{Solution, SolutionPair};
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file = "input/day13".to_string();
    let input = read_to_string(input_file).expect("bad path");

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
struct Moves {
    move_x: usize,
    move_y: usize,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Machine {
    button_a: Moves,
    button_b: Moves,
    prize_location: Location,
}

impl Machine {
    fn new(ax: usize, ay: usize, bx: usize, by: usize, px: usize, py: usize) -> Self {
        Self {
            button_a: Moves {
                move_x: ax,
                move_y: ay,
            },
            button_b: Moves {
                move_x: bx,
                move_y: by,
            },
            prize_location: Location { x: px, y: py },
        }
    }

    fn valid_combos(&self) -> HashSet<Combo> {
        find_combos(
            self.button_a.move_x,
            self.button_b.move_x,
            self.prize_location.x,
        )
        .intersection(&find_combos(
            self.button_a.move_y,
            self.button_b.move_y,
            self.prize_location.y,
        ))
        .cloned()
        .collect()
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Combo {
    presses_a: usize,
    presses_b: usize,
}

impl Combo {
    fn tokens(self) -> usize {
        (self.presses_a * 3) + self.presses_b
    }
}

fn find_combos(a: usize, b: usize, value: usize) -> HashSet<Combo> {
    let mut combos: HashSet<Combo> = HashSet::new();

    for (presses_a, presses_b) in (1..(value / a)).cartesian_product(1..value / b) {
        if (a * presses_a) + (b * presses_b) == value {
            combos.insert(Combo {
                presses_a,
                presses_b,
            });
        }
    }

    combos
}

fn solution1(input: &str) -> usize {
    let mut total_tokens = 0;
    let machine_re = Regex::new(r"Button A: X\+(?P<ax>[0-9]+), Y\+(?P<ay>[0-9]+)\nButton B: X\+(?P<bx>[0-9]+), Y\+(?P<by>[0-9]+)\nPrize: X=(?P<px>[0-9]+), Y=(?P<py>[0-9]+)").expect("Hey!");

    for captures in machine_re.captures_iter(input) {
        let machine: Machine = Machine::new(
            captures["ax"].parse().expect("Hey"),
            captures["ay"].parse().expect("Hey"),
            captures["bx"].parse().expect("Hey"),
            captures["by"].parse().expect("Hey"),
            captures["px"].parse().expect("Hey"),
            captures["py"].parse().expect("Hey"),
        );

        total_tokens += machine
            .valid_combos()
            .iter()
            .map(|c| c.tokens())
            .min()
            .unwrap_or(0);
    }

    total_tokens
}
