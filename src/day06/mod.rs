
use std::{io::{Stdin, BufRead}};
use std::collections::HashSet;

fn all_different(s: &str) -> bool {
    let set: HashSet<char> = s.chars().collect();
    set.len() != s.len()
}

fn find_marker(s: &str, marker_length: usize) -> Option<usize> {
    (marker_length..s.len())
        .find(|&i| !all_different(&s[i - marker_length..i]))
}

pub fn part1(stdin: Stdin) {
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read input");
    println!("{}", find_marker(&input, 4).expect("Not found"));
}

pub fn part2(stdin: Stdin) {
    let mut input = String::new();
    stdin.lock().read_line(&mut input).expect("Failed to read input");
    println!("{}", find_marker(&input, 14).expect("Not found"));
}
