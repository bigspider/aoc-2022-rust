
use std::{io::{Stdin}};
extern crate num;

use num::integer::lcm;

struct Monkey {
    items: Vec<u32>,
    operation: String,
    test_divisor: u32,
    if_true: usize,
    if_false: usize,
}

impl Monkey {
    fn new_from(stdin: &Stdin) -> Option<Monkey> {
        let mut line = String::new();

        stdin.read_line(&mut line).ok()?; // skip the first line

        // Starting items
        line.clear();
        stdin.read_line(&mut line).ok()?;
        if line.trim().len() == 0 {
            return None;
        }
        let items_line_parts: Vec<&str> = line.split(": ").collect();
        let nums: Vec<&str> = items_line_parts[1]
            .split(", ").collect();
        let items: Vec<u32> = nums.iter().map(|&f| f.trim().parse::<u32>().expect("Invalid input")).collect();

        // Operation
        line.clear();
        stdin.read_line(&mut line).expect("Invalid input");
        let op_line_parts: Vec<&str> = line.split(": ").collect();
        let operation: String = op_line_parts.get(1).expect("Invalid input").to_string();

        // Operation
        line.clear();
        stdin.read_line(&mut line).expect("Invalid input");
        let test_divisor: u32 = line.split(" ").last().expect("Invalid input").trim().parse().expect("Invalid input");

        // throws
        line.clear();
        stdin.read_line(&mut line).expect("Invalid input");
        let if_true: usize = line.split(" ").last().expect("Invalid input").trim().parse().expect("Invalid input");
        line.clear();
        stdin.read_line(&mut line).expect("Invalid input");
        let if_false: usize = line.split(" ").last().expect("Invalid input").trim().parse().expect("Invalid input");

        line.clear();
        stdin.read_line(&mut line).expect("Invalid input"); // skip final empty line

        return Some(Monkey { items, operation, test_divisor, if_true, if_false })
    }
}


fn apply_op(old: u32, op: &str) -> u32 {
    fn parse_operand(operand: &str, old: u32) -> u32 {
        if operand.trim() == "old" { old } else { operand.trim().parse::<u32>().expect("Invalid operand") }
    }

    let parts: Vec<&str> = op.split(" = ").nth(1).expect("Invalid operation").split(" ").collect();
    let lhs = parse_operand(parts[0], old);
    let rhs = parse_operand(parts[2], old);

    match parts[1] {
        "+" => (lhs + rhs) / 3,
        "*" => (lhs * rhs) / 3,
        _ => panic!("Invalid operator")
    }
}

fn apply_op_mod(old: u32, op: &str, modulus: u32) -> u32 {
    fn parse_operand(operand: &str, old: u32) -> u32 {
        if operand.trim() == "old" { old } else { operand.trim().parse::<u32>().expect("Invalid operand") }
    }

    let parts: Vec<&str> = op.split(" = ").nth(1).expect("Invalid operation").split(" ").collect();
    let lhs = parse_operand(parts[0], old);
    let rhs = parse_operand(parts[2], old);

    match parts[1] {
        "+" => (lhs + rhs) % modulus,
        "*" => ((lhs as u64 * rhs as u64) % modulus as u64) as u32,
        _ => panic!("Invalid operator")
    }
}

pub fn part1(stdin: Stdin) {
    let mut monkeys = Vec::new();

    loop {
        match Monkey::new_from(&stdin) {
            Some(m) => monkeys.push(m),
            None => break
        }
    }

    let mut n_inspected = vec![0 as usize; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {

            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();

            n_inspected[i] += items.len();

            for old in items {
                let new = apply_op(old, &monkeys[i].operation);

                if new % monkeys[i].test_divisor == 0 {
                    let if_true = monkeys[i].if_true; 
                    monkeys[if_true].items.push(new);
                } else {
                    let if_false = monkeys[i].if_false; 
                    monkeys[if_false].items.push(new);
                }
            }
        }
    }

    n_inspected.sort();
    println!("{}", n_inspected[monkeys.len() - 2] * n_inspected[monkeys.len() - 1]);
}


pub fn part2(stdin: Stdin) {
    let mut monkeys = Vec::new();

    loop {
        match Monkey::new_from(&stdin) {
            Some(m) => monkeys.push(m),
            None => break
        }
    }

    let mut modulus: u32 = 1;
    for m in &monkeys {
        modulus = lcm(modulus, m.test_divisor);
    }

    let mut n_inspected = vec![0 as usize; monkeys.len()];

    for _ in 0..10000 {
        for i in 0..monkeys.len() {

            let items = monkeys[i].items.clone();
            monkeys[i].items.clear();

            n_inspected[i] += items.len();

            for old in items {
                let new = apply_op_mod(old, &monkeys[i].operation, modulus);

                if new % monkeys[i].test_divisor == 0 {
                    let if_true = monkeys[i].if_true; 
                    monkeys[if_true].items.push(new);
                } else {
                    let if_false = monkeys[i].if_false; 
                    monkeys[if_false].items.push(new);
                }
            }
        }
    }

    n_inspected.sort();
    println!("{}", n_inspected[monkeys.len() - 2] * n_inspected[monkeys.len() - 1]);
}
