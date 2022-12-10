
use std::{io::{Stdin, BufRead}, cmp::max};

fn read_input(stdin: &Stdin) -> Vec<Vec<u8>> {
    stdin.lock().lines()
        .map(|l| l.expect("Failed to read input"))
        .map(|line| line.into_bytes())
        .collect()
}

pub fn part1(stdin: Stdin) {
    let input = read_input(&stdin);
    let mut visible = vec![vec![false; input.len()]; input[0].len()];

    // TODO: figure out how to avoid code replication. An iterator?
    for row in 0..input.len() {
        let mut cur: i32 = -1;
        for col in 0..input[0].len() {
            if input[row][col] as i32 > cur {
                visible[row][col] = true;
                cur = input[row][col] as i32;
            }
        }
    }

    for row in 0..input.len() {
        let mut cur: i32 = -1;
        for col in (0..input[0].len()).rev() {
            if input[row][col] as i32 > cur {
                visible[row][col] = true;
                cur = input[row][col] as i32;
            }
        }
    }

    for col in 0..input.len() {
        let mut cur: i32 = -1;
        for row in 0..input[0].len() {
            if input[row][col] as i32 > cur {

                visible[row][col] = true;
                cur = input[row][col] as i32;
            }
        }
    }

    for col in 0..input.len() {
        let mut cur: i32 = -1;
        for row in (0..input[0].len()).rev() {
            if input[row][col] as i32 > cur {
                visible[row][col] = true;
                cur = input[row][col] as i32;
            }
        }
    }

    let result = visible.iter().flatten().filter(|&&x| x).count();
    println!("{}", result);
}


pub fn part2(stdin: Stdin) {
    let input = read_input(&stdin);

    // Could be done in O(n^2), or with less code repetition, or both :P
    let mut best: i32 = -1;
    for row in 1..input.len() - 1 {
        for col in 1..input[0].len() - 1 {
            let cur = input[row][col];

            let mut all_scores: Vec<i32> = Vec::new();

            // TOP
            let mut score = 0;
            for r in (0..row).rev() {
                score += 1;
                if input[r][col] >= cur {
                    break;
                }
            }
            all_scores.push(score);

            // BOTTOM
            score = 0;
            for r in row + 1..input.len() {
                score += 1;
                if input[r][col] >= cur {
                    break;
                }
            }
            all_scores.push(score);

            // LEFT
            score = 0;
            for c in (0..col).rev() {
                score += 1;
                if input[row][c] >= cur {
                    break;
                }
            }
            all_scores.push(score);

            // RIGHT
            score = 0;
            for c in col + 1..input[0].len() {
                score += 1;
                if input[row][c] >= cur {
                    break;
                }
            }
            all_scores.push(score);

            println!("{:?}, {}", all_scores, score);
            let score = all_scores[0] * all_scores[1] * all_scores[2] * all_scores[3];
            best = max(best, score);
        }
    }
    
    println!("{}", best);
}
