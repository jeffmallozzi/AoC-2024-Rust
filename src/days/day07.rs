use crate::{Solution, SolutionPair};
use std::{borrow::Borrow, fs::read_to_string};

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input_file: String = "input/day07".to_string();
    let input = read_to_string(input_file).expect("Hey!");

    let sol1: u64 = solution1(&input, Vec::from([add, mul]));
    let sol2: u64 = solution1(&input, Vec::from([add, mul, cat]));

    (Solution::from(sol1), Solution::from(sol2))
}

fn mag(a: u64) -> u32 {
    let mut res: u32 = 1;
    while a / (10_u64.pow(res)) >= 1 {
        res += 1;
    }
    res
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn mul(a: u64, b: u64) -> u64 {
    a * b
}

fn cat(a: u64, b: u64) -> u64 {
    (a * 10_u64.pow(mag(b))) + b
}

struct Possibilities {
    test_value: u64,
    values: Vec<u64>,
    operators: Vec<fn(u64, u64) -> u64>,
    is_new: bool,
}

impl Possibilities {
    fn new() -> Possibilities {
        Possibilities {
            test_value: 0,
            values: Vec::new(),
            operators: Vec::new(),
            is_new: true,
        }
    }

    fn add(&mut self, new_value: u64) {
        if self.is_new {
            self.values.push(new_value);
            self.is_new = false;
        } else {
            let mut new_values: Vec<u64> = Vec::new();
            for value in self.values.clone() {
                for func in self.operators.clone() {
                    let res = func(value, new_value);
                    if res <= self.test_value {
                        new_values.push(res);
                    }
                }
            }
            self.values = new_values;
        }
    }

    fn test(&self) -> bool {
        self.values.contains(&self.test_value)
    }
}

fn solution1(input: &str, operators: Vec<fn(u64, u64) -> u64>) -> u64 {
    let mut test_sum: u64 = 0;

    for line in input.lines() {
        let (test, values) = line.split_once(':').expect("No :");
        let test_value: u64 = test.trim().parse::<u64>().expect("Parese Error");
        let mut nums: Vec<u64> = Vec::new();
        for num in values.split_whitespace() {
            nums.push(num.parse::<u64>().expect("Parse Error"));
        }

        let mut possibilities = Possibilities::new();
        possibilities.test_value = test_value;
        for operator in operators.clone() {
            possibilities.operators.push(operator);
        }

        for num in nums {
            possibilities.add(num);
        }

        if possibilities.test() {
            test_sum += possibilities.test_value;
        }
    }

    test_sum
}

#[cfg(test)]
mod tests {
    use std::arch::x86_64::_SIDD_MASKED_POSITIVE_POLARITY;

    use super::*;

    #[test]
    fn day07_01() {
        let result = cat(1, 1);
        assert_eq!(result, 11);
    }

    #[test]
    fn day07_02() {
        let result = cat(23, 0);
        assert_eq!(result, 230);
    }

    #[test]
    fn day07_03() {
        let result = cat(450, 99);
        assert_eq!(result, 45099);
    }

    #[test]
    fn day07_04() {
        let result = cat(454545454545, 3477877);
        assert_eq!(result, 4545454545453477877);
    }

    #[test]
    fn day07_mag_01() {
        let result = mag(1);
        assert_eq!(result, 1);
    }

    #[test]
    fn day07_mag_02() {
        let result = mag(9);
        assert_eq!(result, 1);
    }

    #[test]
    fn day07_mag_03() {
        let result = mag(10);
        assert_eq!(result, 2);
    }
}
