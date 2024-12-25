use itertools::{enumerate, FilterMapOk};

use crate::{Solution, SolutionPair};
use std::{borrow::Borrow, clone, fs::read_to_string, str::SplitAsciiWhitespace};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file = "input/day09".to_string();
    let input = read_to_string(input_file).expect("Hey!");

    let sol1: i64 = solution1(&input);
    let sol2: i64 = solution2(&input);

    (Solution::from(sol1), Solution::from(sol2))
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct File {
    id: i64,
    location: i64,
    length: i64,
}

#[derive(Debug, Clone)]
struct Disk {
    sectors: Vec<i64>,
    files: Vec<File>,
}

impl Disk {
    fn new() -> Self {
        Disk {
            sectors: Vec::new(),
            files: Vec::new(),
        }
    }

    fn add(&mut self, mut file: File) {
        let location = self.sectors.len();
        file.location = (location as i64);
        for sector in 0..file.length {
            self.sectors.push(file.id);
        }
        self.files.push(file);
    }

    fn find_space(&self, size: i64) -> Option<i64> {
        for (i, sector) in enumerate(self.sectors.clone()) {
            if sector < 0 && i < self.sectors.len() - (size as usize) {
                if self.sectors[i..i + (size as usize)].iter().all(|x| x < &0) {
                    return Some((i as i64));
                }
            }
        }
        None
    }

    fn move_file(&mut self, mut file_index: i64, mut location: i64) {
        let mut old_index = self.files[(file_index as usize)].location;
        let id = self.files[(file_index as usize)].id;
        let length = self.files[(file_index as usize)].length;

        self.files[(file_index as usize)].location = location;
        for _ in 0..length {
            self.sectors[(file_index as usize)] = id;
            self.sectors[(old_index as usize)] = -1;
            old_index += 1;
            file_index += 1;
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

fn solution2(input: &str) -> i64 {
    let mut disk = Disk::new();
    let mut file_id: i64 = 0;
    let mut indicator: i64 = 1;
    let mut checksum: i64 = 0;

    for number in input.trim().chars() {
        disk.add(File {
            id: file_id * indicator,
            location: 0,
            length: (number.to_digit(10).expect("Hey!") as i64),
        });
        if indicator > 0 {
            file_id += 1;
        }
        indicator *= -1;
        //file_id += 1;
    }

    let files = disk.files.as_mut_slice();
    files.reverse();

    for file in files {
        let free_space = disk.find_space(file.length);
        match free_space {
            Some(t) => println!("Free space at sector: {}", t),
            None => continue,
        }
    }

    0
}
