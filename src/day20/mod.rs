
use std::{io::{Stdin, BufRead}};

fn read_input(stdin: &Stdin) -> Vec<(usize, i64)> {
    stdin.lock().lines().enumerate()
        .map(|(i, line)| {
            let content = line.expect("IO Error");
            (i, content.parse::<i64>().expect("Parse error"))
        })
        .collect()
}

fn move_num<T: Copy>(vector: &mut Vec<T>, pos: usize, new_pos: usize) {
    let el = vector.remove(pos);
    vector.splice(new_pos..new_pos, std::iter::once(el));
}

fn mix(numbers: &mut Vec<(usize, i64)>) {
    let n = numbers.len();

    for i in 0..n {
        let pos = (0..n).find(|&t| numbers[t].0 == i).unwrap();
        let num = numbers[pos].1;
        let m = (n - 1) as i64;
        let final_pos = if pos as i64 + num <= 0 {
            (pos as i64 + num - 1).rem_euclid(m) + 1
        } else {
            (pos as i64 + num) % m
        };

        move_num(numbers, pos, final_pos as usize);
    }
}

fn get_answer(numbers: &Vec<(usize, i64)>) -> i64 {
    let zero_pos = numbers.iter().position(|&(_, num)| num == 0).expect("Not found");

    (1..=3).map(|i| numbers[(zero_pos + i * 1000) % numbers.len()].1).sum()
}

pub fn part1(stdin: Stdin) {
    let mut numbers = read_input(&stdin);
    mix(&mut numbers);
    println!("{}", get_answer(&numbers));
}

pub fn part2(stdin: Stdin) {
    let mut numbers = read_input(&stdin);

    for num in &mut numbers {
        num.1 *= 811589153;
    }

    for _ in 0..10 {
        mix(&mut numbers);
    }

    println!("{}", get_answer(&numbers));
}
