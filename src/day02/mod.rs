use std::{io::{Stdin, BufRead}};

pub fn part1(stdin: Stdin) {
    let mut opponent_moves = Vec::new();
    let mut my_moves = Vec::new();

    for line in stdin.lock().lines() {
        let num_str = line.expect("Failed to read input");
        assert!(num_str.len() == 3);
        opponent_moves.push(num_str.chars().nth(0).unwrap());
        my_moves.push(num_str.chars().nth(2).unwrap());
    }

    let mut total = 0;
    for (i, &opp_move) in opponent_moves.iter().enumerate() {
        let my_move = my_moves[i];
        let my_move_int = my_move as i32 - 'X' as i32 + 1;

        let opp_move_int = opp_move as i32 - 'A' as i32 + 1;

        total += my_move_int;
        println!("Move score: {}", my_move_int);
        total += if my_move_int == opp_move_int {
            3
        } else if (my_move_int - opp_move_int + 3) % 3 == 1 {
            6
        } else {
            0
        }
    }

    println!("{}", total)
}

pub fn part2(stdin: Stdin) {
    let mut opponent_moves = Vec::new();
    let mut outcomes = Vec::new();

    for line in stdin.lock().lines() {
        let num_str = line.expect("Failed to read input");
        assert!(num_str.len() == 3);
        opponent_moves.push(num_str.chars().nth(0).unwrap());
        outcomes.push(num_str.chars().nth(2).unwrap());
    }

    let mut total = 0;
    for (i, &opp_move) in opponent_moves.iter().enumerate() {
        let outcome = outcomes[i];

        let opp_move_int = opp_move as i32 - 'A' as i32 + 1;

        fn get_move(n: i32) -> i32 {
            if n % 3 == 0 { 3 } else { n % 3 }
        }

        if outcome == 'X' {
            total += 0 + get_move(opp_move_int - 1);
        } else if outcome == 'Y' {
            total += 3 + opp_move_int;
        } else if outcome == 'Z' {
            total += 6 + get_move(opp_move_int + 1);
        } else {
            panic!("This is awkward")
        }
    }

    println!("{}", total)
}
