use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file = "input/day14".to_string();
    let input = read_to_string(input_file).expect("Bad File");

    let sol1: isize = solution1(&input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Quadrant {
    One,
    Two,
    Three,
    Four,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Location {
    x: isize,
    y: isize,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Volocity {
    x: isize,
    y: isize,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Robot {
    location: Location,
    volocity: Volocity,
}

impl Robot {
    fn new(location: Location, volocity: Volocity) -> Self {
        Self { location, volocity }
    }

    fn walk(mut self, steps: isize) -> Self {
        print!(
            "Start location {:?}, Volocity {:?} - ",
            self.location, self.volocity
        );
        self.location.x += (self.volocity.x * steps);
        self.location.y += (self.volocity.y * steps);

        while self.location.x < 0 {
            self.location.x += 101;
        }
        while self.location.y < 0 {
            self.location.y += 103;
        }

        while self.location.x > 100 {
            self.location.x -= 101;
        }
        while self.location.y > 102 {
            self.location.y -= 103;
        }

        println!("End location {:?}", self.location);
        self
    }

    fn quadrant(self) -> Option<Quadrant> {
        if self.location.x < 50 && self.location.y < 51 {
            return Some(Quadrant::One);
        }
        if self.location.x < 50 && self.location.y > 51 {
            return Some(Quadrant::Two);
        }
        if self.location.x > 50 && self.location.y < 51 {
            return Some(Quadrant::Three);
        }
        if self.location.x > 50 && self.location.y > 51 {
            return Some(Quadrant::Four);
        }

        None
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

    let mut robots_final: Vec<Robot> = Vec::new();
    robots
        .iter_mut()
        .for_each(|r| robots_final.push(r.walk(100)));

    let mut quad_1 = 0;
    let mut quad_2 = 0;
    let mut quad_3 = 0;
    let mut quad_4 = 0;

    for robot in robots_final {
        println!("Robot location: {:?}", robot.location);
        match robot.quadrant() {
            Some(Quadrant::One) => quad_1 += 1,
            Some(Quadrant::Two) => quad_2 += 1,
            Some(Quadrant::Three) => quad_3 += 1,
            Some(Quadrant::Four) => quad_4 += 1,
            None => (),
        }
    }

    quad_1 * quad_2 * quad_3 * quad_4
}
