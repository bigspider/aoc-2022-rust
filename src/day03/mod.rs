use std::{io::{Stdin, BufRead}};
use std::collections::HashSet;

fn get_value(c: u8) -> u32 {
    if c >= 'a' as u8 && c <= 'z' as u8 { return (c - ('a' as u8) + 1) as u32}
    else if c >= 'A' as u8 && c <= 'Z' as u8 { return (c - 'A' as u8 + 27) as u32}
    else { panic!("That's awkward"); }
}

pub fn part1(stdin: Stdin) {
    let mut total: u32 = 0;
    for line in stdin.lock().lines() {
        let content = line.expect("Failed to read input");
        assert!(content.len() % 2 == 0);
        let len = content.len() / 2;
        let (half1, half2) = content.split_at(len);

        let mut half1_set = HashSet::new();

        for &c in half1.as_bytes() {
            half1_set.insert(c);
        }
        for &c in half2.as_bytes() {
            if half1_set.contains(&c) {
                total += get_value(c);
                break;
            }
        }
    }

    println!("{}", total)
}

pub fn part2(stdin: Stdin) {
    let mut total: u32 = 0;
    let mut count = 0;

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    for line in stdin.lock().lines() {
        let content = line.expect("Failed to read input");
        if count == 0 {
            for &c in content.as_bytes() {
                set1.insert(c);
            }
        } else if count == 1 {
            for &c in content.as_bytes() {
                set2.insert(c);
            }
        } else {
            for &c in content.as_bytes() {
                if set1.contains(&c) && set2.contains(&c) {
                    total += get_value(c);
                    break;
                }
            }

            set1.clear();
            set2.clear();
        }

        count += 1;
        if count == 3 {
            count = 0;
        }
    }

    println!("{}", total)
}
