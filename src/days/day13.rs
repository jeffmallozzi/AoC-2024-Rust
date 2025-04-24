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
struct Moves {
    move_x: isize,
    move_y: isize,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Machine {
    button_a: Moves,
    button_b: Moves,
    prize_location: Location,
}

impl Machine {
    fn new(ax: isize, ay: isize, bx: isize, by: isize, px: isize, py: isize) -> Self {
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

    fn least_tokens(&self) -> isize {
        // Stolen from the reddit comment: https://www.reddit.com/r/adventofcode/comments/1hd4wda/comment/m32wabe/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

        let t1 = (self.button_b.move_x * self.prize_location.y)
            - (self.button_b.move_y * self.prize_location.x);
        let t2 = (self.button_b.move_x * self.button_a.move_y)
            - (self.button_b.move_y * self.button_a.move_x);

        if t1 % t2 != 0 {
            return 0;
        }

        let ak = t1 / t2;

        let t3 = self.prize_location.x - (self.button_a.move_x * ak);

        if t3 % self.button_b.move_x != 0 {
            return 0;
        }

        let bk = t3 / self.button_b.move_x;

        (ak * 3) + bk
    }
}

fn solution1(input: &str) -> (isize, isize) {
    let mut total_tokens_part1 = 0;
    let mut total_tokens_part2 = 0;
    let mut machine_count = 0;

    let machine_re = Regex::new(r"Button A: X\+(?P<ax>[0-9]+), Y\+(?P<ay>[0-9]+)\nButton B: X\+(?P<bx>[0-9]+), Y\+(?P<by>[0-9]+)\nPrize: X=(?P<px>[0-9]+), Y=(?P<py>[0-9]+)").expect("Hey!");

    for captures in machine_re.captures_iter(input) {
        let mut machine: Machine = Machine::new(
            captures["ax"].parse().expect("Hey"),
            captures["ay"].parse().expect("Hey"),
            captures["bx"].parse().expect("Hey"),
            captures["by"].parse().expect("Hey"),
            captures["px"].parse().expect("Hey"),
            captures["py"].parse().expect("Hey"),
        );

        total_tokens_part1 += machine.least_tokens();

        machine.prize_location.x += 10000000000000;
        machine.prize_location.y += 10000000000000;

        total_tokens_part2 += machine.least_tokens();
    }

    (total_tokens_part1, total_tokens_part2)
}
