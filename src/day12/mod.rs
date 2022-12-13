
use std::{io::{Stdin, BufRead}, cmp::min};

const NIL: i32 = i32::MAX - 1; // a large value that we can increment without overflow

fn read_input(stdin: &Stdin) -> (Vec<Vec<u8>>, (usize, usize), (usize, usize)) {
    let mut heights: Vec<Vec<u8>> = Vec::new();
    let mut start_pos: Option<(usize, usize)>  = None;
    let mut end_pos : Option<(usize, usize)>  = None;
    for (row, line) in stdin.lock().lines().enumerate() {
        let mut line_heights: Vec<u8> = line.expect("Failed to read input").bytes().collect();

        for col in 0..line_heights.len() {
            match line_heights[col] as char {
                'S' => {
                    line_heights[col] = 'a' as u8;
                    start_pos = Some((row, col));
                },
                'E' => {
                    line_heights[col] = 'z' as u8;
                    end_pos = Some((row, col));
                }
                _ => {}
            }
        }
        heights.push(line_heights);
    }

    (heights, start_pos.expect("Start position not found"), end_pos.expect("End position not found"))
}

fn compute_dists(heights: &Vec<Vec<u8>>, start_pos: &(usize, usize), end_pos: &(usize, usize)) -> Vec<Vec<i32>> {
    // TODO: easy to make this O(n^2) but we are feeling lazy :P
    let mut dist = vec![vec![NIL; heights[0].len()]; heights.len()];
    dist[end_pos.0][end_pos.1] = 0;
    let n_rows = dist.len();
    let n_cols = dist[0].len();
    while dist[start_pos.0][start_pos.1] == NIL {
        for row in 0..n_rows {
            for col in 0..n_cols {
                let el = heights[row][col];
                let mut new_dists = vec![dist[row][col]];
                if row > 0 && heights[row-1][col] <= el + 1 { new_dists.push(1 + dist[row-1][col]) }
                if col > 0 && heights[row][col-1] <= el + 1 { new_dists.push(1 + dist[row][col-1]) }
                if row < n_rows - 1 && heights[row+1][col] <= el + 1 { new_dists.push(1 + dist[row+1][col]) }
                if col < n_cols - 1 && heights[row][col+1] <= el + 1 { new_dists.push(1 + dist[row][col+1]) }
                let new_value = *new_dists.iter().min().expect("Now this is unexpected");
                dist[row][col] = new_value
            }
        }
    }

    dist
}

pub fn part1(stdin: Stdin) {
    let (heights, start_pos, end_pos) = read_input(&stdin);
    let dist = compute_dists(&heights, &start_pos, &end_pos);

    println!("{}", dist[start_pos.0][start_pos.1]);
}


pub fn part2(stdin: Stdin) {
    let (heights, start_pos, end_pos) = read_input(&stdin);
    let dist = compute_dists(&heights, &start_pos, &end_pos);

    let mut best = i32::MAX;
    for row in 0..heights.len() {
        for col in 0..heights[0].len() {
            if heights[row][col] == 'a' as u8 {
                best = min(best, dist[row][col]);
            }
        }
    }

    println!("{}", best);
}
