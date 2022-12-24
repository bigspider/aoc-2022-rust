
use std::{io::{Stdin, BufRead}, str::FromStr, collections::HashMap};

#[derive(Debug, Clone)]
enum Operator { Add, Sub, Mul, Div }

#[derive(Debug, Clone)]
enum Monkey {
    Op { name: String, op: Operator, operand1: String, operand2: String },
    Value { name: String, val: i64 }
}

#[derive(Debug)]
enum MonkeyParseError {
    ParseIntError,
    ParseMonkeyError,
}

impl Monkey {
    fn get_name(&self) -> String {
        match self {
            Monkey::Op { name, op: _, operand1: _, operand2: _ } => name.clone(),
            Monkey::Value { name, val: _ } => name.clone(),
        }
    }
}

impl FromStr for Monkey {
    type Err = MonkeyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        match parts.len() {
            2 => {
                Ok(Monkey::Value {
                    name: parts[0][0..4].chars().collect::<String>(),
                    val: if let Ok(s) = parts[1].parse::<i64>() {
                        s
                    } else {
                        return Err(MonkeyParseError::ParseMonkeyError)
                    }
                })
            }
            4 => {
                let op = match parts[2] {
                    "+" => Operator::Add,
                    "-" => Operator::Sub,
                    "*" => Operator::Mul,
                    "/" => Operator::Div,
                    _ => return Err(MonkeyParseError::ParseMonkeyError),
                };

                let name = parts[0][0..4].chars().collect::<String>();
                let operand1 = parts[1][0..4].chars().collect::<String>();
                let operand2 = parts[3][0..4].chars().collect::<String>();
                Ok(Monkey::Op { name, op, operand1, operand2 })
            }
            _ => Err(MonkeyParseError::ParseMonkeyError)
        }
    }
}

fn read_input(stdin: &Stdin) -> HashMap<String, Monkey> {
    stdin.lock().lines()
        .map(|line| line.expect("IO Error").parse::<Monkey>().expect("Parsing error"))
        .map(|monkey| (monkey.get_name(), monkey))
        .collect()
    }

fn solve(results: &mut HashMap<String, i64>, monkeys: &HashMap<String, Monkey>, name: &String, is_part2: bool) -> Option<i64> {
    if let Some(&res) = results.get(name) {
        return Some(res);
    }

    if is_part2 && name == "humn" { return None }

    let monkey = monkeys.get(name).expect("Monkey not found");

    let result = match monkey {
        Monkey::Value { name: _, val } => *val,
        Monkey::Op { name: _, op, operand1, operand2 } => {
            let op1 = solve(results, monkeys, operand1, is_part2);
            let op2 = solve(results, monkeys, operand2, is_part2);

            // only for part 2, if any child returns None, we also return None
            op1?;
            op2?;

            match op {
                Operator::Add => op1.unwrap() + op2.unwrap(),
                Operator::Sub => op1.unwrap() - op2.unwrap(),
                Operator::Mul => op1.unwrap() * op2.unwrap(),
                Operator::Div => op1.unwrap() / op2.unwrap(),
            }
        },
    };

    results.insert(name.clone(), result);

    Some(result)
}

fn solve2(results: &HashMap<String, i64>, monkeys: &HashMap<String, Monkey>, name: &String, expected: i64) -> i64 {
    match monkeys.get(name).expect("Monkey not found") {
        Monkey::Value { name: _, val: _ } => {
            if name == "humn" {
                expected // yell the answer
            } else {
                panic!("This is unexpected");
            }
        },
        Monkey::Op { name: _, op, operand1, operand2 } => {
            let res1 = results.get(operand1);
            let res2 = results.get(operand2);
            if name == "root" {
                // one child must be fully specified, so that gives us the starting point for `expected`
                // we recurse on the other child
                if let Some(&num1) = res1 {
                    solve2(results, monkeys, operand2, num1)
                } else if let Some(&num2) = res2 {
                    solve2(results, monkeys, operand1, num2)
                } else {
                    panic!("This should never happen");
                }
            } else {
                // again, one child must be fully specified already
                // we compute the expected result for the other child (where we recurse)
                if let Some(&num1) = res1 {
                    // the first child is specified
                    match op {
                        Operator::Add => solve2(results, monkeys, operand2, expected - num1), // num1 + res2 == expected
                        Operator::Sub => solve2(results, monkeys, operand2, num1 - expected), // num1 - res2 == expected
                        Operator::Mul => solve2(results, monkeys, operand2, expected / num1), // num1 * res2 == expected
                        Operator::Div => solve2(results, monkeys, operand2, num1 / expected), // num1 / res2 == expected
                    }
                } else if let Some(&num2) = res2 {
                    // the second child is specified
                    match op {
                        Operator::Add => solve2(results, monkeys, operand1, expected - num2), // res1 + num2 == expected
                        Operator::Sub => solve2(results, monkeys, operand1, expected + num2), // res1 - num2 == expected
                        Operator::Mul => solve2(results, monkeys, operand1, expected / num2), // res1 * num2 == expected
                        Operator::Div => solve2(results, monkeys, operand1, expected * num2), // res1 / num2 == expected
                    }
                } else {
                    panic!("This should never happen");
                }
            }
        },
    }
}

pub fn part1(stdin: Stdin) {
    let monkeys: HashMap<String, Monkey> = read_input(&stdin);
    let mut results: HashMap<String, i64> = HashMap::new();
    println!("{}", solve(&mut results, &monkeys, &String::from("root"), false).unwrap());
}

pub fn part2(stdin: Stdin) {
    let monkeys: HashMap<String, Monkey> = read_input(&stdin);

    let root_name = String::from("root");

    // first, solve as before to compute all the "known" subtrees
    // (except fail if the subtree contains "humn")
    let mut results: HashMap<String, i64> = HashMap::new();
    solve(&mut results, &monkeys, &root_name, true);
    results.remove(&root_name);

    println!("{}", solve2(&results, &monkeys, &root_name, 0));
}

