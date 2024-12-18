use itertools::{enumerate, Combinations, Itertools};

use crate::{Solution, SolutionPair};
use std::char;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day08".to_string();
    let input = read_to_string(input_file).expect("Hey!");

    let (sol1, sol2): (i64, i64) = solution1(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

struct Antinodes {
    a: (i64, i64),
    b: (i64, i64),
}

impl Iterator for Antinodes {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let next_point = (
            self.b.0 + (self.b.0 - self.a.0),
            self.b.1 + (self.b.1 - self.a.1),
        );
        self.a = self.b;
        self.b = next_point;
        Some(next_point)
    }
}

fn solution1(input: &str) -> (i64, i64) {
    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut antinodes: HashSet<(i64, i64)> = HashSet::new();
    let mut antinodes2: HashSet<(i64, i64)> = HashSet::new();
    let mut locations: HashSet<(i64, i64)> = HashSet::new();

    for (row_i, row) in enumerate(input.lines()) {
        for (col_i, col) in enumerate(row.chars()) {
            locations.insert((row_i.try_into().unwrap(), col_i.try_into().unwrap()));
            if col != '.' {
                antennas
                    .entry(col)
                    .or_insert(Vec::new())
                    .push((row_i.try_into().unwrap(), col_i.try_into().unwrap()));
            }
        }
    }

    for (frequency, spots) in antennas {
        for pair in spots.into_iter().permutations(2) {
            antinodes2.insert(pair[0]);
            antinodes2.insert(pair[1]);
            let mut antenna_pair = Antinodes {
                a: pair[0],
                b: pair[1],
            };
            let spot = antenna_pair.next().expect("Hey!");
            if locations.contains(&spot) {
                antinodes.insert(spot.clone());
                antinodes2.insert(spot);
            }
            for spot2 in antenna_pair {
                if locations.contains(&spot2) {
                    antinodes2.insert(spot2);
                } else {
                    break;
                }
            }
        }
    }

    (
        antinodes.len().try_into().unwrap(),
        antinodes2.len().try_into().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::Antinodes;

    #[test]
    fn test_day08_iter01() {
        let mut x: Antinodes = Antinodes {
            a: (1, 1),
            b: (2, 2),
        };
        assert_eq!(Some((3, 3)), x.next());
    }

    #[test]
    fn test_day08_iter02() {
        let mut x: Antinodes = Antinodes {
            a: (3, 3),
            b: (2, 2),
        };
        assert_eq!(Some((1, 1)), x.next());
    }

    #[test]
    fn test_day08_iter03() {
        let mut x: Antinodes = Antinodes {
            a: (1, 2),
            b: (2, 1),
        };
        assert_eq!(Some((3, 0)), x.next());
    }

    #[test]
    fn test_day08_iter04() {
        let mut x: Antinodes = Antinodes {
            a: (2, 1),
            b: (1, 2),
        };
        assert_eq!(Some((0, 3)), x.next());
    }

    #[test]
    fn test_day08_iter05() {
        let mut x: Antinodes = Antinodes {
            a: (1, 1),
            b: (2, 2),
        };
        x.next();
        assert_eq!(Some((4, 4)), x.next());
    }
}
