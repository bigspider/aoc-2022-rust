use std::{io::{Stdin, BufRead}};



pub fn part1(stdin: Stdin) {
    let mut cur_sum = 0;
    let mut max_sum = 0;

    for line in stdin.lock().lines() {
        let num_str = line.expect("Failed to read input");

        if let Ok(num) = num_str.parse::<u32>() {
            cur_sum += num;
        } else {
            cur_sum = 0;
        }

        if cur_sum > max_sum {
            max_sum = cur_sum
        }
    }

    println!("{}", max_sum)
}

pub fn part2(stdin: Stdin) {
    let mut nums = Vec::new();
    let mut cur_sum = 0;

    for line in stdin.lock().lines() {
        let num_str = line.expect("Failed to read input");

        if let Ok(num) = num_str.parse::<u32>() {
            cur_sum += num;
        } else {
            nums.push(cur_sum);
            cur_sum = 0;
        }
    }

    nums.sort();

    println!("{}", nums.iter().rev().take(3).sum::<u32>())
}
