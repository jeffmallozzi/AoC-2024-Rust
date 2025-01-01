use itertools::enumerate;

use crate::{Solution, SolutionPair};
use itertools::Itertools;
use std::{
    collections::HashMap,
    collections::HashSet,
    fs::read_to_string,
    iter::{Map, Sum},
};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day10".to_string();
    let input = read_to_string(input_file).unwrap();

    let sol: (usize, usize) = solution1(&input);

    (Solution::from(sol.0), Solution::from(sol.1))
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Location {
    row: usize,
    col: usize,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct MapTile {
    elevation: usize,
    nine_cache: Option<usize>,
}

struct TrailMap {
    tiles: HashMap<Location, MapTile>,
}

impl TrailMap {
    fn new() -> Self {
        TrailMap {
            tiles: HashMap::new(),
        }
    }
}

fn walk1(location: Location, trailmap: &TrailMap) -> HashSet<Location> {
    let mut trail_ends: HashSet<Location> = HashSet::new();
    match trailmap.tiles.get(&location) {
        None => trail_ends,
        Some(tile) => {
            if tile.elevation == 9 {
                trail_ends.insert(location);
                return trail_ends;
            } else {
                trail_ends.extend(
                    get_neigbors(location)
                        .iter()
                        .filter(|l| {
                            trailmap.tiles.contains_key(l)
                                && trailmap.tiles.get(l).unwrap().elevation == tile.elevation + 1
                        })
                        .map(|l| walk1(*l, trailmap).into_iter())
                        .flatten(),
                );

                return trail_ends;
            }
        }
    }
}

fn walk2(location: Location, trailmap: &TrailMap) -> usize {
    let mut trails: usize = 0;
    match trailmap.tiles.get(&location) {
        None => trails,
        Some(tile) => {
            if tile.elevation == 9 {
                return 1;
            } else {
                get_neigbors(location)
                    .iter()
                    .filter(|l| {
                        trailmap.tiles.contains_key(l)
                            && trailmap.tiles.get(l).unwrap().elevation == tile.elevation + 1
                    })
                    .map(|l| walk2(*l, trailmap))
                    .sum()
            }
        }
    }
}

fn get_neigbors(location: Location) -> Vec<Location> {
    let mut neigbors: Vec<Location> = Vec::new();
    neigbors.push(Location {
        row: location.row,
        col: location.col + 1,
    });
    neigbors.push(Location {
        row: location.row + 1,
        col: location.col,
    });
    if location.row > 0 {
        neigbors.push(Location {
            row: location.row - 1,
            col: location.col,
        });
    }
    if location.col > 0 {
        neigbors.push(Location {
            row: location.row,
            col: location.col - 1,
        });
    }

    neigbors
}

fn solution1(input: &str) -> (usize, usize) {
    let mut trail_map: TrailMap = TrailMap::new();

    for (row_i, row) in enumerate(input.lines()) {
        for (col_i, elevation) in enumerate(row.chars()) {
            trail_map.tiles.insert(
                Location {
                    row: row_i,
                    col: col_i,
                },
                MapTile {
                    elevation: (elevation.to_digit(10).unwrap() as usize),
                    nine_cache: None,
                },
            );
        }
    }

    trail_map
        .tiles
        .iter()
        .filter(|t| t.1.elevation == 0)
        .map(|t| (walk1(*t.0, &trail_map).len(), walk2(*t.0, &trail_map)))
        .fold(((0 as usize), (0 as usize)), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1)
        })
}
