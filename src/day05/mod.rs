
use std::{io::{Stdin, BufRead}};


trait CanShrinkIfNeeded {
    fn shrink_if_needed(& mut self);
}

impl<T> CanShrinkIfNeeded for Vec<T> {
    fn shrink_if_needed(& mut self) {
        if self.len() * 4 < self.capacity() {
            self.shrink_to_fit();
        }
    }
}

pub fn part1(stdin: Stdin) {
    let mut first_line = true;
    let mut c: usize = 0; // number of columns

    let mut columns: Vec<Vec<char>> = Vec::new();

    for line in stdin.lock().lines() {
        let line_str = line.expect("Failed to read input");
        if first_line {
            c = (line_str.len() + 1) as usize / 4;

            for _ in 0..c {
                columns.push(Vec::new())
            }

            first_line = false;
        }

        if line_str.len() == 0 {
            break;
        }

        for (i, ch) in line_str.chars().skip(1).step_by(4).enumerate() {
            columns[i].push(ch);
        }
    }

    for i in 0..c {
        columns[i].reverse();
        columns[i].retain(|t| *t != ' ');
    }

    for line in stdin.lock().lines() {
        let line_str = line.expect("Failed to read input");
        let parts: Vec<&str> = line_str.split(" ").collect();

        assert!(parts.len() == 6);

        let n: usize = parts[1].parse().unwrap();
        let src: usize = parts[3].parse().unwrap();
        let dst: usize = parts[5].parse().unwrap();

        for _ in 0..n {
            let elem = columns[src - 1].pop().expect("Oops, tried to pop an empty stack");
            columns[dst - 1].push(elem)
        }

        columns[src - 1].shrink_if_needed(); // guarantee O(n) memory occupation
    }

    for i in 0..c {
        print!("{}", columns[i].last().expect("This is awkward"))
    }
    println!();
}

pub fn part2(stdin: Stdin) {
    let mut first_line = true;
    let mut c: usize = 0; // number of columns

    let mut columns: Vec<Vec<char>> = Vec::new();

    for line in stdin.lock().lines() {
        let line_str = line.expect("Failed to read input");
        if first_line {
            c = (line_str.len() + 1) as usize / 4;

            for _ in 0..c {
                columns.push(Vec::new())
            }

            first_line = false;
        }

        if line_str.len() == 0 {
            break;
        }

        for (i, ch) in line_str.chars().skip(1).step_by(4).enumerate() {
            columns[i].push(ch);
        }
    }

    for i in 0..c {
        columns[i].reverse();
        columns[i].retain(|t| *t != ' ');
    }

    for line in stdin.lock().lines() {
        let line_str = line.expect("Failed to read input");
        let parts: Vec<&str> = line_str.split(" ").collect();

        assert!(parts.len() == 6);

        let n: usize = parts[1].parse().unwrap();
        let src: usize = parts[3].parse().unwrap();
        let dst: usize = parts[5].parse().unwrap();

        let new_length = columns[src - 1].len() - n;
        let removed_elems: Vec<char> = columns[src - 1].drain(new_length..).collect();
        columns[dst - 1].extend_from_slice(&removed_elems);

        columns[src - 1].shrink_if_needed(); // guarantee O(n) memory occupation
    }

    for i in 0..c {
        print!("{}", columns[i].last().expect("This is awkward"))
    }
    println!();
}
