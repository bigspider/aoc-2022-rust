
use std::{io::{Stdin, BufRead}};

pub fn part1(stdin: Stdin) {
    let mut X: i32 = 1;
    let mut cycle: usize = 1;
    let mut result = 0;

    fn tick(result: &mut i32, X: i32, cycle: &mut usize) {
        if *cycle % 40 == 20 && *cycle <= 220 {
            *result += (*cycle as i32) * X;
        }
        *cycle += 1;
    }

    for line in stdin.lock().lines() {
        let content = line.expect("Failed tZo read input");
        let parts = content.split(' ').collect::<Vec<&str>>();

        match parts[0] {
            "noop" => {
                tick(&mut result, X, &mut cycle);
            }
            "addx" => {
                let V: i32 = parts[1].parse().expect("Failed to parse operand");

                tick(&mut result, X, &mut cycle);
                tick(&mut result, X, &mut cycle);
                X += V;
            }
            _ => panic!("Unexpected opcode")
        }
    }

    println!("{}", result);
}


pub fn part2(stdin: Stdin) {
    let mut X: i32 = 1;
    let mut cycle: usize = 1;

    fn tick(X: i32, cycle: &mut usize) {
        if (X - ((*cycle - 1) % 40) as i32).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if *cycle % 40 == 0 {
            println!();
        }

        *cycle += 1;
    }

    for line in stdin.lock().lines() {
        let content = line.expect("Failed to read input");
        let parts = content.split(' ').collect::<Vec<&str>>();

        match parts[0] {
            "noop" => {
                tick(X, &mut cycle);
            }
            "addx" => {
                let V: i32 = parts[1].parse().expect("Failed to parse operand");

                tick(X, &mut cycle);
                tick(X, &mut cycle);
                X += V;
            }
            _ => panic!("Unexpected opcode")
        }
    }
}
