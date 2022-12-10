use std::{io::{Stdin, BufRead}};

fn parse_pair(data: &str) -> (i32, i32) {
    let values: Vec<i32> = data.split('-')
    .map(|z| z.parse::<i32>().unwrap())
    .collect();
    (values[0], values[1])
}

trait Interval {
    type T;
    fn contains(&self, other: &(Self::T, Self::T)) -> bool;
    fn overlaps(&self, other: &(Self::T, Self::T)) -> bool;
}

impl Interval for (i32, i32) {
    type T = i32;
    fn contains(&self, other: &(i32, i32)) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }
    fn overlaps(&self, other: &(i32, i32)) -> bool {
        !(self.1 < other.0 || other.1 < self.0)
    }
}

pub fn part1(stdin: Stdin) {
    let mut total: u32 = 0;
    for line in stdin.lock().lines() {
        let content = line.expect("Failed to read input");
        let parts = content.split(',').collect::<Vec<&str>>();

        assert!(parts.len() == 2);

        let pair1 = parse_pair(parts[0]);
        let pair2 = parse_pair(parts[1]);

        if pair1.contains(&pair2) || pair2.contains(&pair1) {
            total += 1;
        }
    }

    println!("{}", total)
}

pub fn part2(stdin: Stdin) {
    let mut total: u32 = 0;
    for line in stdin.lock().lines() {
        let content = line.expect("Failed to read input");
        let parts = content.split(',').collect::<Vec<&str>>();

        assert!(parts.len() == 2);

        let pair1 = parse_pair(parts[0]);
        let pair2 = parse_pair(parts[1]);

        if pair1.overlaps(&pair2) {
            total += 1;
        }
    }

    println!("{}", total)
}
