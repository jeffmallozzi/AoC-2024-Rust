use crate::{Solution, SolutionPair};
use regex::Regex;
use std::{fs::read_to_string, thread::sleep, time};

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

    fn walk(&mut self, steps: isize) {
        //print!(
        //    "Start location {:?}, Volocity {:?} - ",
        //    self.location, self.volocity
        //);
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

        //println!("End location {:?}", self.location);
    }

    fn quadrant(&self) -> Option<Quadrant> {
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

fn display_robots(robots: &Vec<Robot>) {
    for y in (0..102) {
        for x in (0..100) {
            let mut found = false;
            for robot in robots {
                if robot.location.x == x && robot.location.y == y {
                    found = true;
                    print!("X");
                    break;
                }
            }
            if !found {
                print!(" ");
            }
        }
        println!("");
    }
    //print!("{}[2J", 27 as char);

    //for robot in robots {
    //    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    //    let row = robot.location.y;
    //    let col = robot.location.x;

    //    print!("{esc}[2J{esc}[{r};{c}H", esc = 27 as char, r = row, c = col);
    //    print!("R");
    //}
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

    //let mut robots_final: Vec<Robot> = Vec::new();
    //robots
    //    .iter_mut()
    //   .for_each(|r| robots_final.push(r.walk(100)));
    //
    //let mut robot_display: Vec<Vec<char>> = Vec::new();

    let mut result: isize = 0;
    for step in (1..10000) {
        robots.iter_mut().for_each(|r| r.walk(1));
        //display_robots(&robots);
        //println!("{}", step);
        //sleep(time::Duration::from_millis(250));

        let mut quad_1 = 0;
        let mut quad_2 = 0;
        let mut quad_3 = 0;
        let mut quad_4 = 0;

        for robot in robots.clone() {
            //println!("Robot location: {:?}", robot.location);
            match robot.quadrant() {
                Some(Quadrant::One) => quad_1 += 1,
                Some(Quadrant::Two) => quad_2 += 1,
                Some(Quadrant::Three) => quad_3 += 1,
                Some(Quadrant::Four) => quad_4 += 1,
                None => (),
            }
        }

        let safty_factor = quad_1 * quad_2 * quad_3 * quad_4;
        if step == 100 {
            result = safty_factor
        }

        if safty_factor < 50000000 {
            display_robots(&robots);
            println!("{step}");
        }
    }
    result
}
