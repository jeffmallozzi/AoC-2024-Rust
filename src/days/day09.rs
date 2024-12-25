use itertools::{enumerate, FilterMapOk, Itertools};
use regex::bytes::CaptureLocations;

use crate::{Solution, SolutionPair};
use std::{
    arch::x86_64::_mm256_undefined_pd, borrow::Borrow, clone, collections::HashMap,
    fs::read_to_string, iter::FilterMap, num::NonZeroUsize, str::SplitAsciiWhitespace,
};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file = "input/day09".to_string();
    let input = read_to_string(input_file).expect("Hey!");

    let sol1: i64 = solution1(&input);
    let sol2: usize = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct File {
    id: usize,
    length: usize,
}

#[derive(Debug, Clone)]
struct Disk {
    sectors: Vec<Option<usize>>,
    files: HashMap<usize, File>,
    space: HashMap<usize, usize>,
}

impl Disk {
    fn new() -> Self {
        Disk {
            sectors: Vec::new(),
            files: HashMap::new(),
            space: HashMap::new(),
        }
    }

    fn add_file(&mut self, mut file: File) {
        let location = self.sectors.len();
        for sector in 0..file.length {
            self.sectors.push(Some(file.id));
        }
        self.files.insert(location, file);
    }

    fn add_space(&mut self, length: usize) {
        let location = self.sectors.len();
        self.space.insert(location, length);
        for sector in 0..length {
            self.sectors.push(None);
        }
    }

    fn find_space(&mut self, size: usize) -> Option<usize> {
        self.space
            .iter()
            .filter(|&(_, v)| *v >= size)
            .min_by_key(|&(k, _)| k)
            .map(|(&k, _)| k)
    }

    fn get_file(&mut self, location: usize) -> Option<File> {
        let file = self.files.get(&location);
        match file {
            None => None,
            Some(f) => Some(*f),
        }
    }

    fn move_file(&mut self, location: usize) {
        let file = self.get_file(location);
        match file {
            None => return,
            Some(f) => match self.find_space(f.length) {
                None => return,
                Some(new_location) => {
                    let space_length = *self.space.get(&new_location).unwrap();
                    if new_location >= location {
                        return;
                    }
                    for i in 0..f.length {
                        self.sectors[new_location + i] = Some(f.id);
                        self.sectors[location + i] = None;
                    }
                    self.files.remove(&location);
                    self.files.insert(new_location, f);
                    self.space.remove(&new_location);
                    self.space.insert(location, f.length);
                    if space_length > f.length {
                        self.space
                            .insert(new_location + f.length, space_length - f.length);
                    }
                }
            },
        }
    }
}

fn solution1(input: &str) -> i64 {
    let mut disk: Vec<i64> = Vec::new();
    let mut file_id: i64 = 0;
    let mut indicator: i64 = 1;
    let mut checksum: i64 = 0;

    for number in input.trim().chars() {
        for _ in 0..number.to_digit(10).expect("Hey") {
            disk.push(file_id * indicator);
        }
        if indicator == 1 {
            file_id += 1
        }
        indicator *= -1;
    }

    let mut head: usize = 0;
    let mut tail: usize = disk.len() - 1;

    while head < tail {
        if disk[head] < 0 {
            while disk[tail] < 0 {
                disk.pop();
                tail -= 1;
            }
            disk[head] = disk.pop().expect("Hey!");
            tail -= 1;
        }

        head += 1;
    }

    for (i, num) in enumerate(disk) {
        if num >= 0 {
            checksum += i64::try_from(i).expect("Hey") * num;
        }
    }

    checksum
}

fn solution2(input: &str) -> usize {
    let mut disk = Disk::new();
    let mut file_id: usize = 0;
    let mut indicator: i8 = 1;
    let mut checksum: usize = 0;

    for number in input.trim().chars() {
        let length = (number.to_digit(10).unwrap() as usize);
        if indicator > 0 {
            disk.add_file(File {
                id: file_id,
                length,
            });
            file_id += 1;
        } else {
            disk.add_space(length);
        }
        indicator *= -1;
    }

    let mut files: Vec<usize> = disk
        .files
        .iter()
        .map(|(location, file)| *location)
        .sorted()
        .collect();
    files.reverse();

    for location in files {
        disk.move_file(location);
    }

    for (i, num) in enumerate(disk.sectors) {
        match num {
            Some(n) => checksum += n * i,
            None => continue,
        }
    }

    checksum
}
