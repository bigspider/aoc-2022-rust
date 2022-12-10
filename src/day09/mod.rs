
use std::{io::{Stdin, BufRead}, collections::HashSet, ops::{Add, Index, IndexMut}, ops::Sub};

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Pair(i32, i32);

impl Pair {
    fn is_disjoint(self) -> bool {
        self.0.abs() > 1 || self.1.abs() > 1
    }

    fn follow(&mut self, other: Self) {
        if (other - *self).is_disjoint() {
            for i in 0..2 {
                match (other[i] - self[i]).abs() {
                    0 | 1 => self[i] = other[i],
                    2 => self[i] = (self[i] + other[i]) / 2,
                    _ => panic!("Unexpected"),
                }
            }
        }
    }
}

impl Index<i32> for Pair {
    type Output = i32;
    fn index(&self, index: i32) -> &i32 {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Out of bounds: {}", index),
        }
    }
}

impl IndexMut<i32> for Pair {
    fn index_mut(&mut self, index: i32) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            _ => panic!("Out of bounds: {}", index),
        }
    }
}

impl Add for Pair {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Pair(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Pair {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Pair(self.0 - other.0, self.1 - other.1)
    }
}

fn get_dir(dir_str: &str) -> Option<Pair> {
    match dir_str {
        "U" => Some(Pair(0, -1)),
        "D" => Some(Pair(0, 1)),
        "L" => Some(Pair(-1, 0)),
        "R" => Some(Pair(1, 0)),
        _ => None
    }
}

pub fn part1(stdin: Stdin) {
    let mut visited: HashSet<Pair> = HashSet::new();
    visited.insert(Pair(0, 0));
    let mut cur_head = Pair(0, 0);
    let mut cur_tail = Pair(0, 0);

    for line in stdin.lock().lines() {
        let content = line.expect("Failed to read input");
        let parts = content.split(' ').collect::<Vec<&str>>();
        let steps: u32 = parts[1].parse().expect("Failed to parse number of steps");

        let dir = get_dir(parts[0]).expect("Invalid direction");

        for _ in 0..steps {
            cur_head = cur_head + dir;
            cur_tail.follow(cur_head);
            visited.insert(cur_tail);
        }
    }

    println!("{}", visited.len());
}


pub fn part2(stdin: Stdin) {
    let mut visited: HashSet<Pair> = HashSet::new();
    visited.insert(Pair(0, 0));

    let mut pieces = vec![Pair(0, 0); 10];

    for line in stdin.lock().lines() {
        let content = line.expect("Failed to read input");
        let parts = content.split(' ').collect::<Vec<&str>>();
        let steps: u32 = parts[1].parse().expect("Failed to parse number of steps");

        let dir = get_dir(parts[0]).expect("Invalid direction");

        for _ in 0..steps {
            pieces[0] = pieces[0] + dir;
            for i in 1..pieces.len() {
                let prev = pieces[i - 1];
                pieces[i].follow(prev);
            }
            visited.insert(*pieces.last().expect("Expected to have at least one piece"));
        }
    }

    println!("{}", visited.len());
}
