use crate::{Solution, SolutionPair};
use regex::Regex;
use std::{fs::read_to_string, intrinsics::ptr_offset_from_unsigned};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file = "input/day14".to_string();
    let input = read_to_string(input_file).expect("Bad File");

    let sol1: u64 = 0;
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

struct Location {
    x: isize,
    y: isize,
}

struct Volocity {
    x: isize,
    y: isize,
}

struct Robot {
    location: Location,
    volocity: Volocity,
}

impl Robot {
    fn new(location: Location, volocity: Volocity) -> Self {
        Self { location, volocity }
    }

    fn walk(mut self) {
        self.location.x = (self.location.x + self.volocity.x) % 101;
        self.location.y = (self.location.y + self.volocity.y) % 103;
    }
}

fn solution1(input: &str) -> isize {
    let re =
        Regex::new(r"p=(?P<locx>[0-9]+),(?P<locy>[0-9]+) v=(?P<volx>-?[0-9]+),(?P<voly>-?[0-9]+)")
            .expect("Hey!");
    let mut robots: Vec<Robot> = Vec::new();

    for line in input.lines() {
        let captures = re.captures(line).expect("Hey!");

        robots.push(Robot::new(
            Location {
                x: captures["locx"].parse().expect("Hey"),
                y: captures["locy"].parse().expect("Hey"),
            },
            Volocity {
                x: captures["volx"].parse().expect("Hey"),
                y: captures["voly"].parse().expect("Hey"),
            },
        ));
    }

    for i in (0..100) {
        for robot in robots {
            robot.walk();
        }
    }

    let mut quad_1 = 0;
    let mut quad_2 = 0;
    let mut quad_3 = 0;
    let mut quad_4 = 0;

    for robot in robots {
        if robot.location.x < 50 {
            if robot.location.y < 51 {
                quad_1 += 1;
            } else if robot.location.y > 51 {
                quad_2 += 1;
            }
        } else if robot.location.x > 50 {
            if robot.location.y < 51 {
                quad_3 += 1;
            } else if robot.location.y > 51 {
                quad_4 += 1;
            }
        }
    }

    quad_1 * quad_2 * quad_3 * quad_4
}
